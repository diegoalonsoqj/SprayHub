//! VTF thumbnail decoder.
//!
//! Decodes the full-resolution image of a Valve Texture Format file (not the
//! tiny embedded low-res thumbnail), downscaling via mipmaps to ~256px, and
//! wraps it in an uncompressed BMP encoded as a base64 data URL suitable for an
//! `<img>` tag. Supports the formats sprays use in practice: DXT1/DXT3/DXT5 and
//! common uncompressed layouts (RGBA/BGRA/ARGB/ABGR/RGB/BGR). No image crates.

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

/// Decode a VTF file's main image into a `data:image/bmp;base64,...` URL.
pub fn thumbnail_data_url(bytes: &[u8]) -> AppResult<String> {
    let (width, height, rgb) = decode_main_image(bytes)?;
    let bmp = encode_bmp_24(width, height, &rgb);
    Ok(format!("data:image/bmp;base64,{}", base64_encode(&bmp)))
}

fn decode_main_image(bytes: &[u8]) -> AppResult<(u32, u32, Vec<u8>)> {
    let header = parse_header(bytes)?;

    if header.width == 0 || header.height == 0 {
        return Err(AppError::Validation("VTF has zero dimensions".into()));
    }

    let data_start = high_res_data_start(bytes, &header)?;
    let (level, mip_w, mip_h) = choose_mip(header.width, header.height, header.mip_count);
    let offset = offset_to_mip(data_start, &header, level)?;

    let size = image_data_size(mip_w, mip_h, header.high_format)
        .ok_or_else(|| AppError::Validation(format!(
            "unsupported VTF image format: {}",
            header.high_format
        )))?;
    let end = offset
        .checked_add(size)
        .ok_or_else(|| AppError::Validation("VTF size overflow".into()))?;
    if end > bytes.len() {
        return Err(AppError::Validation("VTF image data is truncated".into()));
    }

    let rgb = decode_image(&bytes[offset..end], mip_w, mip_h, header.high_format)?;
    Ok((mip_w, mip_h, rgb))
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
            AppError::Validation(format!("unsupported VTF image format: {}", header.high_format))
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

/// Decode one image to a row-major RGB888 buffer.
fn decode_image(data: &[u8], width: u32, height: u32, format: i32) -> AppResult<Vec<u8>> {
    match format {
        FMT_DXT1 | FMT_DXT1_ONEBITALPHA => Ok(decode_bc(data, width, height, 8, 0, true)),
        FMT_DXT3 => Ok(decode_bc(data, width, height, 16, 8, false)),
        FMT_DXT5 => Ok(decode_bc(data, width, height, 16, 8, false)),
        FMT_RGBA8888 => Ok(decode_linear(data, width, height, 4, [0, 1, 2])),
        FMT_ABGR8888 => Ok(decode_linear(data, width, height, 4, [3, 2, 1])),
        FMT_ARGB8888 => Ok(decode_linear(data, width, height, 4, [1, 2, 3])),
        FMT_BGRA8888 | FMT_BGRX8888 => Ok(decode_linear(data, width, height, 4, [2, 1, 0])),
        FMT_RGB888 => Ok(decode_linear(data, width, height, 3, [0, 1, 2])),
        FMT_BGR888 => Ok(decode_linear(data, width, height, 3, [2, 1, 0])),
        other => Err(AppError::Validation(format!(
            "unsupported VTF image format: {other}"
        ))),
    }
}

/// Decode an uncompressed image given bytes-per-pixel and the source byte
/// indices that map to (R, G, B).
fn decode_linear(data: &[u8], width: u32, height: u32, bpp: usize, rgb_idx: [usize; 3]) -> Vec<u8> {
    let mut out = vec![0u8; (width * height * 3) as usize];
    let pixel_count = (width * height) as usize;
    for p in 0..pixel_count {
        let src = p * bpp;
        if src + bpp > data.len() {
            break;
        }
        let dst = p * 3;
        out[dst] = data[src + rgb_idx[0]];
        out[dst + 1] = data[src + rgb_idx[1]];
        out[dst + 2] = data[src + rgb_idx[2]];
    }
    out
}

/// Decode a block-compressed image (DXT1/3/5). `block_bytes` is the per-block
/// stride, `color_offset` the offset of the 8-byte color block inside it, and
/// `dxt1_mode` enables DXT1's 3-color/transparent variant.
fn decode_bc(
    data: &[u8],
    width: u32,
    height: u32,
    block_bytes: usize,
    color_offset: usize,
    dxt1_mode: bool,
) -> Vec<u8> {
    let mut out = vec![0u8; (width * height * 3) as usize];
    let blocks_x = blocks_for(width);
    let blocks_y = blocks_for(height);

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let block_index = (by * blocks_x + bx) as usize;
            let base = block_index * block_bytes + color_offset;
            if base + 8 > data.len() {
                continue;
            }
            let c0 = read_u16(data, base);
            let c1 = read_u16(data, base + 2);
            let lookup = read_u32(data, base + 4);
            let palette = build_palette(c0, c1, dxt1_mode);

            for py in 0..4 {
                for px in 0..4 {
                    let x = bx * 4 + px;
                    let y = by * 4 + py;
                    if x >= width || y >= height {
                        continue;
                    }
                    let bit = 2 * (py * 4 + px);
                    let idx = ((lookup >> bit) & 0b11) as usize;
                    let (r, g, b) = palette[idx];
                    let dst = ((y * width + x) * 3) as usize;
                    out[dst] = r;
                    out[dst + 1] = g;
                    out[dst + 2] = b;
                }
            }
        }
    }
    out
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
        palette[3] = (0, 0, 0); // 1-bit alpha: transparent black
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

/// Encode a row-major RGB888 buffer as an uncompressed 24-bit BMP (bottom-up).
fn encode_bmp_24(width: u32, height: u32, rgb: &[u8]) -> Vec<u8> {
    let row_bytes = width * 3;
    let padding = (4 - (row_bytes % 4)) % 4;
    let stride = row_bytes + padding;
    let pixel_data_size = stride * height;
    let file_header = 14u32;
    let info_header = 40u32;
    let offset = file_header + info_header;
    let file_size = offset + pixel_data_size;

    let mut out = Vec::with_capacity(file_size as usize);
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&file_size.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&offset.to_le_bytes());

    out.extend_from_slice(&info_header.to_le_bytes());
    out.extend_from_slice(&(width as i32).to_le_bytes());
    out.extend_from_slice(&(height as i32).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&24u16.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&pixel_data_size.to_le_bytes());
    out.extend_from_slice(&2835i32.to_le_bytes());
    out.extend_from_slice(&2835i32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());

    for y in (0..height).rev() {
        for x in 0..width {
            let src = ((y * width + x) * 3) as usize;
            out.push(rgb[src + 2]); // B
            out.push(rgb[src + 1]); // G
            out.push(rgb[src]); // R
        }
        for _ in 0..padding {
            out.push(0);
        }
    }
    out
}

/// Standard base64 encoder (RFC 4648).
fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
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
    fn rgb565_white_and_black() {
        assert_eq!(rgb565_to_888(0xFFFF), (255, 255, 255));
        assert_eq!(rgb565_to_888(0x0000), (0, 0, 0));
    }

    #[test]
    fn dxt1_solid_block_decodes_white() {
        let mut block = Vec::new();
        block.extend_from_slice(&0xFFFFu16.to_le_bytes());
        block.extend_from_slice(&0x0000u16.to_le_bytes());
        block.extend_from_slice(&0x00000000u32.to_le_bytes());
        let rgb = decode_bc(&block, 4, 4, 8, 0, true);
        assert_eq!(rgb.len(), 4 * 4 * 3);
        assert_eq!(&rgb[0..3], &[255, 255, 255]);
    }

    #[test]
    fn linear_bgra_swaps_channels() {
        // One pixel, BGRA = (10, 20, 30, 255) -> RGB (30, 20, 10)
        let data = [10u8, 20, 30, 255];
        let rgb = decode_linear(&data, 1, 1, 4, [2, 1, 0]);
        assert_eq!(rgb, vec![30, 20, 10]);
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
        // 512x512 with full mip chain -> level 1 (256x256)
        assert_eq!(choose_mip(512, 512, 10), (1, 256, 256));
        // 256x256 -> level 0
        assert_eq!(choose_mip(256, 256, 9), (0, 256, 256));
        // 64x64 -> level 0
        assert_eq!(choose_mip(64, 64, 7), (0, 64, 64));
    }

    #[test]
    fn rejects_non_vtf() {
        assert!(thumbnail_data_url(b"not a vtf file at all 0123456789").is_err());
    }
}
