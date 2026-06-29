//! VTF thumbnail decoder.
//!
//! Decodes the full-resolution image of a Valve Texture Format file (not the
//! tiny embedded low-res thumbnail), downscaling via mipmaps to ~256px, and
//! encodes it as a base64 PNG data URL with a real alpha channel so transparent
//! spray areas stay transparent. Supports DXT1/DXT3/DXT5 and common
//! uncompressed layouts (RGBA/BGRA/ARGB/ABGR/RGB/BGR). No image crates.

use crate::domain::error::{AppError, AppResult};

const VTF_SIGNATURE: &[u8; 4] = b"VTF\0";

// VTF image format enum values we handle.
const FMT_RGBA8888: i32 = 0;
const FMT_ABGR8888: i32 = 1;
const FMT_RGB888: i32 = 2;
const FMT_BGR888: i32 = 3;
const FMT_ARGB8888: i32 = 11;
const FMT_BGRA8888: i32 = 12;
const FMT_DXT1: i32 = 13;
const FMT_DXT3: i32 = 14;
const FMT_DXT5: i32 = 15;
const FMT_BGRX8888: i32 = 16;
const FMT_DXT1_ONEBITALPHA: i32 = 20;

/// Target maximum thumbnail dimension; we pick the largest mip that fits.
const TARGET_MAX_DIM: u32 = 256;

#[derive(Clone, Copy)]
enum Bc {
    Dxt1,
    Dxt3,
    Dxt5,
}

struct VtfHeader {
    version_minor: u32,
    header_size: usize,
    width: u32,
    height: u32,
    frames: u32,
    mip_count: u32,
    high_format: i32,
    low_format: i32,
    low_w: u32,
    low_h: u32,
}

/// Decode a VTF file's main image into a `data:image/png;base64,...` URL.
pub fn thumbnail_data_url(bytes: &[u8]) -> AppResult<String> {
    let (width, height, rgba) = decode_main_image(bytes)?;
    let png = encode_png_rgba(width, height, &rgba);
    Ok(format!("data:image/png;base64,{}", base64_encode(&png)))
}

/// Returns `(width, height, rgba)` where `rgba` is row-major RGBA8888.
fn decode_main_image(bytes: &[u8]) -> AppResult<(u32, u32, Vec<u8>)> {
    let header = parse_header(bytes)?;

    if header.width == 0 || header.height == 0 {
        return Err(AppError::Validation("VTF has zero dimensions".into()));
    }

    let data_start = high_res_data_start(bytes, &header)?;
    let (level, mip_w, mip_h) = choose_mip(header.width, header.height, header.mip_count);
    let offset = offset_to_mip(data_start, &header, level)?;

    let size = image_data_size(mip_w, mip_h, header.high_format).ok_or_else(|| {
        AppError::Validation(format!(
            "unsupported VTF image format: {}",
            header.high_format
        ))
    })?;
    let end = offset
        .checked_add(size)
        .ok_or_else(|| AppError::Validation("VTF size overflow".into()))?;
    if end > bytes.len() {
        return Err(AppError::Validation("VTF image data is truncated".into()));
    }

    let rgba = decode_image(&bytes[offset..end], mip_w, mip_h, header.high_format)?;
    Ok((mip_w, mip_h, rgba))
}

fn parse_header(bytes: &[u8]) -> AppResult<VtfHeader> {
    if bytes.len() < 64 || &bytes[0..4] != VTF_SIGNATURE {
        return Err(AppError::Validation("not a valid VTF file".into()));
    }
    Ok(VtfHeader {
        version_minor: read_u32(bytes, 8),
        header_size: read_u32(bytes, 12) as usize,
        width: read_u16(bytes, 16) as u32,
        height: read_u16(bytes, 18) as u32,
        frames: read_u16(bytes, 24).max(1) as u32,
        mip_count: (bytes[56] as u32).max(1),
        high_format: read_i32(bytes, 52),
        low_format: read_i32(bytes, 57),
        low_w: bytes[61] as u32,
        low_h: bytes[62] as u32,
    })
}

/// Locate where the high-resolution image data begins. For VTF >= 7.3 this is
/// stored in a resource entry; earlier versions place it right after the header
/// and the (optional) low-res thumbnail.
fn high_res_data_start(bytes: &[u8], header: &VtfHeader) -> AppResult<usize> {
    if header.version_minor >= 3 && bytes.len() >= 80 {
        let num_resources = read_u32(bytes, 68) as usize;
        let mut entry = 80usize;
        for _ in 0..num_resources {
            if entry + 8 > bytes.len() {
                break;
            }
            let tag = [bytes[entry], bytes[entry + 1], bytes[entry + 2]];
            let res_offset = read_u32(bytes, entry + 4) as usize;
            // 0x30 = high-res image data resource.
            if tag == [0x30, 0x00, 0x00] {
                return Ok(res_offset);
            }
            entry += 8;
        }
    }

    // Pre-7.3 layout: header, then low-res thumbnail, then high-res mipmaps.
    let low_res_size = if header.low_w > 0 && header.low_h > 0 && header.low_format == FMT_DXT1 {
        image_data_size(header.low_w, header.low_h, FMT_DXT1).unwrap_or(0)
    } else {
        0
    };
    Ok(header.header_size + low_res_size)
}

/// Pick the largest mip level whose dimensions fit within `TARGET_MAX_DIM`.
fn choose_mip(width: u32, height: u32, mip_count: u32) -> (u32, u32, u32) {
    let mut level = 0;
    while level + 1 < mip_count {
        let mw = (width >> level).max(1);
        let mh = (height >> level).max(1);
        if mw <= TARGET_MAX_DIM && mh <= TARGET_MAX_DIM {
            break;
        }
        level += 1;
    }
    let mw = (width >> level).max(1);
    let mh = (height >> level).max(1);
    (level, mw, mh)
}

/// Byte offset of `level` (frame 0). Mipmaps are stored smallest-first, so every
/// smaller mip (higher index) precedes the one we want.
fn offset_to_mip(start: usize, header: &VtfHeader, level: u32) -> AppResult<usize> {
    let mut offset = start;
    for i in (level + 1)..header.mip_count {
        let mw = (header.width >> i).max(1);
        let mh = (header.height >> i).max(1);
        let size = image_data_size(mw, mh, header.high_format).ok_or_else(|| {
            AppError::Validation(format!(
                "unsupported VTF image format: {}",
                header.high_format
            ))
        })?;
        offset = offset
            .checked_add(header.frames as usize * size)
            .ok_or_else(|| AppError::Validation("VTF offset overflow".into()))?;
    }
    Ok(offset)
}

/// Size in bytes of one image of the given dimensions and format.
fn image_data_size(width: u32, height: u32, format: i32) -> Option<usize> {
    let blocks = (blocks_for(width) * blocks_for(height)) as usize;
    let pixels = (width * height) as usize;
    let size = match format {
        FMT_DXT1 | FMT_DXT1_ONEBITALPHA => blocks * 8,
        FMT_DXT3 | FMT_DXT5 => blocks * 16,
        FMT_RGBA8888 | FMT_ABGR8888 | FMT_ARGB8888 | FMT_BGRA8888 | FMT_BGRX8888 => pixels * 4,
        FMT_RGB888 | FMT_BGR888 => pixels * 3,
        _ => return None,
    };
    Some(size)
}

fn blocks_for(dim: u32) -> u32 {
    dim.div_ceil(4).max(1)
}

/// Decode one image to a row-major RGBA8888 buffer.
fn decode_image(data: &[u8], width: u32, height: u32, format: i32) -> AppResult<Vec<u8>> {
    let rgba = match format {
        FMT_DXT1 | FMT_DXT1_ONEBITALPHA => decode_bc(data, width, height, Bc::Dxt1),
        FMT_DXT3 => decode_bc(data, width, height, Bc::Dxt3),
        FMT_DXT5 => decode_bc(data, width, height, Bc::Dxt5),
        FMT_RGBA8888 => decode_linear(data, width, height, 4, 0, 1, 2, Some(3)),
        FMT_ABGR8888 => decode_linear(data, width, height, 4, 3, 2, 1, Some(0)),
        FMT_ARGB8888 => decode_linear(data, width, height, 4, 1, 2, 3, Some(0)),
        FMT_BGRA8888 => decode_linear(data, width, height, 4, 2, 1, 0, Some(3)),
        FMT_BGRX8888 => decode_linear(data, width, height, 4, 2, 1, 0, None),
        FMT_RGB888 => decode_linear(data, width, height, 3, 0, 1, 2, None),
        FMT_BGR888 => decode_linear(data, width, height, 3, 2, 1, 0, None),
        other => {
            return Err(AppError::Validation(format!(
                "unsupported VTF image format: {other}"
            )))
        }
    };
    Ok(rgba)
}

/// Decode an uncompressed image into RGBA. `r`/`g`/`b`/`a` are source byte
/// indices within each pixel; `a == None` means fully opaque.
#[allow(clippy::too_many_arguments)]
fn decode_linear(
    data: &[u8],
    width: u32,
    height: u32,
    bpp: usize,
    r: usize,
    g: usize,
    b: usize,
    a: Option<usize>,
) -> Vec<u8> {
    let mut out = vec![0u8; (width * height * 4) as usize];
    let pixel_count = (width * height) as usize;
    for p in 0..pixel_count {
        let src = p * bpp;
        if src + bpp > data.len() {
            break;
        }
        let dst = p * 4;
        out[dst] = data[src + r];
        out[dst + 1] = data[src + g];
        out[dst + 2] = data[src + b];
        out[dst + 3] = a.map(|ai| data[src + ai]).unwrap_or(255);
    }
    out
}

/// Decode a block-compressed image (DXT1/3/5) into RGBA.
fn decode_bc(data: &[u8], width: u32, height: u32, kind: Bc) -> Vec<u8> {
    let (block_bytes, color_offset) = match kind {
        Bc::Dxt1 => (8usize, 0usize),
        Bc::Dxt3 | Bc::Dxt5 => (16, 8),
    };
    let dxt1_mode = matches!(kind, Bc::Dxt1);

    let mut out = vec![0u8; (width * height * 4) as usize];
    let blocks_x = blocks_for(width);
    let blocks_y = blocks_for(height);

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let block = (by * blocks_x + bx) as usize * block_bytes;
            let cbase = block + color_offset;
            if cbase + 8 > data.len() {
                continue;
            }
            let c0 = read_u16(data, cbase);
            let c1 = read_u16(data, cbase + 2);
            let lookup = read_u32(data, cbase + 4);
            let palette = build_palette(c0, c1, dxt1_mode);
            let dxt5_alpha = match kind {
                Bc::Dxt5 => Some(dxt5_alpha_table(data, block)),
                _ => None,
            };

            for py in 0..4u32 {
                for px in 0..4u32 {
                    let x = bx * 4 + px;
                    let y = by * 4 + py;
                    if x >= width || y >= height {
                        continue;
                    }
                    let p = (py * 4 + px) as usize;
                    let idx = ((lookup >> (2 * p)) & 0b11) as usize;
                    let (r, g, b) = palette[idx];
                    let alpha = match kind {
                        Bc::Dxt1 => {
                            if dxt1_mode && c0 <= c1 && idx == 3 {
                                0
                            } else {
                                255
                            }
                        }
                        Bc::Dxt3 => {
                            let nibble = (data[block + p / 2] >> ((p % 2) * 4)) & 0x0F;
                            nibble * 17
                        }
                        Bc::Dxt5 => {
                            let code = dxt5_code(data, block, p);
                            dxt5_alpha.map(|t| t[code]).unwrap_or(255)
                        }
                    };
                    let dst = ((y * width + x) * 4) as usize;
                    out[dst] = r;
                    out[dst + 1] = g;
                    out[dst + 2] = b;
                    out[dst + 3] = alpha;
                }
            }
        }
    }
    out
}

/// Build the 8-entry alpha lookup table for a DXT5 alpha block at `block`.
fn dxt5_alpha_table(data: &[u8], block: usize) -> [u8; 8] {
    let a0 = data[block] as u16;
    let a1 = data[block + 1] as u16;
    let mut t = [0u8; 8];
    t[0] = a0 as u8;
    t[1] = a1 as u8;
    if a0 > a1 {
        for i in 1..=6u16 {
            t[(i + 1) as usize] = (((7 - i) * a0 + i * a1) / 7) as u8;
        }
    } else {
        for i in 1..=4u16 {
            t[(i + 1) as usize] = (((5 - i) * a0 + i * a1) / 5) as u8;
        }
        t[6] = 0;
        t[7] = 255;
    }
    t
}

/// Read the 3-bit alpha code for pixel `p` of a DXT5 block at `block`.
fn dxt5_code(data: &[u8], block: usize, p: usize) -> usize {
    let mut bits: u64 = 0;
    for i in 0..6 {
        bits |= (data[block + 2 + i] as u64) << (8 * i);
    }
    ((bits >> (3 * p)) & 0x7) as usize
}

fn build_palette(c0: u16, c1: u16, dxt1_mode: bool) -> [(u8, u8, u8); 4] {
    let (r0, g0, b0) = rgb565_to_888(c0);
    let (r1, g1, b1) = rgb565_to_888(c1);
    let mut palette = [(0u8, 0u8, 0u8); 4];
    palette[0] = (r0, g0, b0);
    palette[1] = (r1, g1, b1);
    if !dxt1_mode || c0 > c1 {
        palette[2] = (mix(r0, r1, 2, 1), mix(g0, g1, 2, 1), mix(b0, b1, 2, 1));
        palette[3] = (mix(r0, r1, 1, 2), mix(g0, g1, 1, 2), mix(b0, b1, 1, 2));
    } else {
        palette[2] = (mix(r0, r1, 1, 1), mix(g0, g1, 1, 1), mix(b0, b1, 1, 1));
        palette[3] = (0, 0, 0); // paired with 1-bit alpha = transparent
    }
    palette
}

fn mix(a: u8, b: u8, wa: u16, wb: u16) -> u8 {
    ((a as u16 * wa + b as u16 * wb) / (wa + wb)) as u8
}

fn rgb565_to_888(c: u16) -> (u8, u8, u8) {
    let r5 = (c >> 11) & 0x1F;
    let g6 = (c >> 5) & 0x3F;
    let b5 = c & 0x1F;
    let r = ((r5 << 3) | (r5 >> 2)) as u8;
    let g = ((g6 << 2) | (g6 >> 4)) as u8;
    let b = ((b5 << 3) | (b5 >> 2)) as u8;
    (r, g, b)
}

// ---------------------------------------------------------------------------
// PNG encoding (RGBA, 8-bit, no filtering, stored DEFLATE blocks). Hand-rolled
// to avoid an image/zlib dependency.
// ---------------------------------------------------------------------------

fn encode_png_rgba(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    // Raw scanlines, each prefixed with filter byte 0 (None).
    let row = (width * 4) as usize;
    let mut raw = Vec::with_capacity(height as usize * (1 + row));
    for y in 0..height as usize {
        raw.push(0);
        let start = y * row;
        raw.extend_from_slice(&rgba[start..start + row]);
    }

    let mut out = Vec::new();
    out.extend_from_slice(&[137, 80, 78, 71, 13, 10, 26, 10]); // PNG signature

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.push(8); // bit depth
    ihdr.push(6); // color type: truecolor + alpha
    ihdr.push(0); // compression
    ihdr.push(0); // filter
    ihdr.push(0); // interlace
    write_chunk(&mut out, b"IHDR", &ihdr);
    write_chunk(&mut out, b"IDAT", &zlib_store(&raw));
    write_chunk(&mut out, b"IEND", &[]);
    out
}

/// Wrap raw bytes in a zlib stream using uncompressed (stored) DEFLATE blocks.
fn zlib_store(raw: &[u8]) -> Vec<u8> {
    let mut z = Vec::with_capacity(raw.len() + raw.len() / 0xFFFF * 5 + 16);
    z.push(0x78); // CMF
    z.push(0x01); // FLG (valid checksum, fastest)

    if raw.is_empty() {
        z.push(0x01);
        z.extend_from_slice(&0u16.to_le_bytes());
        z.extend_from_slice(&0xFFFFu16.to_le_bytes());
    } else {
        let mut i = 0;
        while i < raw.len() {
            let len = core::cmp::min(0xFFFF, raw.len() - i);
            let is_final = i + len >= raw.len();
            z.push(if is_final { 1 } else { 0 }); // BFINAL, BTYPE=00 (stored)
            z.extend_from_slice(&(len as u16).to_le_bytes());
            z.extend_from_slice(&(!(len as u16)).to_le_bytes());
            z.extend_from_slice(&raw[i..i + len]);
            i += len;
        }
    }

    z.extend_from_slice(&adler32(raw).to_be_bytes());
    z
}

fn write_chunk(out: &mut Vec<u8>, ctype: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(ctype);
    out.extend_from_slice(data);
    let mut crc_input = Vec::with_capacity(4 + data.len());
    crc_input.extend_from_slice(ctype);
    crc_input.extend_from_slice(data);
    out.extend_from_slice(&crc32(&crc_input).to_be_bytes());
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &b in bytes {
        crc ^= b as u32;
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

fn adler32(bytes: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a = 1u32;
    let mut b = 0u32;
    for &x in bytes {
        a = (a + x as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}

/// Standard base64 encoder (RFC 4648).
fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 0x3F) as usize] as char);
        out.push(TABLE[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(TABLE[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(TABLE[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn read_u16(b: &[u8], i: usize) -> u16 {
    u16::from_le_bytes([b[i], b[i + 1]])
}
fn read_u32(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}
fn read_i32(b: &[u8], i: usize) -> i32 {
    i32::from_le_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_known_vectors() {
        assert_eq!(base64_encode(b""), "");
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    fn crc32_known_vector() {
        // CRC-32 of "123456789" is 0xCBF43926.
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn adler32_known_vector() {
        // Adler-32 of "Wikipedia" is 0x11E60398.
        assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);
    }

    #[test]
    fn rgb565_white_and_black() {
        assert_eq!(rgb565_to_888(0xFFFF), (255, 255, 255));
        assert_eq!(rgb565_to_888(0x0000), (0, 0, 0));
    }

    #[test]
    fn dxt1_solid_block_decodes_opaque_white() {
        let mut block = Vec::new();
        block.extend_from_slice(&0xFFFFu16.to_le_bytes());
        block.extend_from_slice(&0x0000u16.to_le_bytes());
        block.extend_from_slice(&0x00000000u32.to_le_bytes());
        let rgba = decode_bc(&block, 4, 4, Bc::Dxt1);
        assert_eq!(rgba.len(), 4 * 4 * 4);
        assert_eq!(&rgba[0..4], &[255, 255, 255, 255]);
    }

    #[test]
    fn linear_bgra_swaps_channels_and_keeps_alpha() {
        // BGRA = (10, 20, 30, 128) -> RGBA (30, 20, 10, 128)
        let data = [10u8, 20, 30, 128];
        let rgba = decode_linear(&data, 1, 1, 4, 2, 1, 0, Some(3));
        assert_eq!(rgba, vec![30, 20, 10, 128]);
    }

    #[test]
    fn image_data_size_formats() {
        assert_eq!(image_data_size(4, 4, FMT_DXT1), Some(8));
        assert_eq!(image_data_size(4, 4, FMT_DXT5), Some(16));
        assert_eq!(image_data_size(2, 2, FMT_RGBA8888), Some(16));
        assert_eq!(image_data_size(2, 2, FMT_RGB888), Some(12));
        assert_eq!(image_data_size(4, 4, 999), None);
    }

    #[test]
    fn choose_mip_prefers_largest_within_budget() {
        assert_eq!(choose_mip(512, 512, 10), (1, 256, 256));
        assert_eq!(choose_mip(256, 256, 9), (0, 256, 256));
        assert_eq!(choose_mip(64, 64, 7), (0, 64, 64));
    }

    #[test]
    fn png_has_signature_and_chunks() {
        let rgba = vec![255u8; 2 * 2 * 4];
        let png = encode_png_rgba(2, 2, &rgba);
        assert_eq!(&png[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        // IHDR type immediately follows the 8-byte signature + 4-byte length.
        assert_eq!(&png[12..16], b"IHDR");
    }

    #[test]
    fn rejects_non_vtf() {
        assert!(thumbnail_data_url(b"not a vtf file at all 0123456789").is_err());
    }
}
