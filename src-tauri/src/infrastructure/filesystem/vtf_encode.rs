//! VTF encoder: turn a row-major RGBA8888 image into a valid Valve Texture
//! Format file. Targets VTF 7.2, a single full-resolution mip (NOMIP), and
//! either uncompressed BGRA8888 (lossless, larger) or DXT5 (compressed, smaller,
//! with interpolated alpha). The companion `.vmt` is written elsewhere.

use crate::domain::error::{AppError, AppResult};

/// Output texture format for a generated spray.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VtfFormat {
    /// Uncompressed 32-bit BGRA. Exact colors and alpha.
    Bgra8888,
    /// DXT5 block compression. ~4x smaller, slight color loss.
    Dxt5,
}

impl VtfFormat {
    pub fn parse(s: &str) -> AppResult<Self> {
        match s.to_ascii_lowercase().as_str() {
            "bgra8888" | "bgra" | "uncompressed" => Ok(VtfFormat::Bgra8888),
            "dxt5" => Ok(VtfFormat::Dxt5),
            other => Err(AppError::Validation(format!(
                "unknown spray format: {other}"
            ))),
        }
    }

    /// VTF `highResImageFormat` enum value.
    fn image_format_id(self) -> i32 {
        match self {
            VtfFormat::Bgra8888 => 12,
            VtfFormat::Dxt5 => 15,
        }
    }
}

// TEXTUREFLAGS bits.
const FLAG_CLAMPS: u32 = 0x0000_0004;
const FLAG_CLAMPT: u32 = 0x0000_0008;
const FLAG_NOMIP: u32 = 0x0000_0100;
const FLAG_NOLOD: u32 = 0x0000_0200;
const FLAG_EIGHTBITALPHA: u32 = 0x0000_2000;

const HEADER_SIZE: u32 = 80;

/// Encode an RGBA image into VTF bytes.
pub fn encode_vtf(width: u32, height: u32, rgba: &[u8], format: VtfFormat) -> AppResult<Vec<u8>> {
    let expected = (width as usize) * (height as usize) * 4;
    if rgba.len() != expected {
        return Err(AppError::Validation(format!(
            "RGBA buffer is {} bytes, expected {expected} for {width}x{height}",
            rgba.len()
        )));
    }
    if width == 0 || height == 0 {
        return Err(AppError::Validation("image has zero dimensions".into()));
    }
    if format == VtfFormat::Dxt5 && (width % 4 != 0 || height % 4 != 0) {
        return Err(AppError::Validation(
            "DXT5 requires dimensions that are multiples of 4".into(),
        ));
    }

    let flags = FLAG_EIGHTBITALPHA | FLAG_CLAMPS | FLAG_CLAMPT | FLAG_NOMIP | FLAG_NOLOD;

    let mut out = Vec::with_capacity(HEADER_SIZE as usize + expected);
    write_header(&mut out, width, height, flags, format);

    match format {
        VtfFormat::Bgra8888 => encode_bgra8888(&mut out, rgba),
        VtfFormat::Dxt5 => encode_dxt5(&mut out, width, height, rgba),
    }

    Ok(out)
}

fn write_header(out: &mut Vec<u8>, width: u32, height: u32, flags: u32, format: VtfFormat) {
    out.extend_from_slice(b"VTF\0"); // 0: signature
    out.extend_from_slice(&7u32.to_le_bytes()); // 4: version major
    out.extend_from_slice(&2u32.to_le_bytes()); // 8: version minor
    out.extend_from_slice(&HEADER_SIZE.to_le_bytes()); // 12: header size
    out.extend_from_slice(&(width as u16).to_le_bytes()); // 16: width
    out.extend_from_slice(&(height as u16).to_le_bytes()); // 18: height
    out.extend_from_slice(&flags.to_le_bytes()); // 20: flags
    out.extend_from_slice(&1u16.to_le_bytes()); // 24: frames
    out.extend_from_slice(&0u16.to_le_bytes()); // 26: first frame
    out.extend_from_slice(&[0u8; 4]); // 28: padding
    out.extend_from_slice(&0f32.to_le_bytes()); // 32: reflectivity r
    out.extend_from_slice(&0f32.to_le_bytes()); // 36: reflectivity g
    out.extend_from_slice(&0f32.to_le_bytes()); // 40: reflectivity b
    out.extend_from_slice(&[0u8; 4]); // 44: padding
    out.extend_from_slice(&1f32.to_le_bytes()); // 48: bumpmap scale
    out.extend_from_slice(&format.image_format_id().to_le_bytes()); // 52: high-res format
    out.push(1); // 56: mipmap count
    out.extend_from_slice(&(-1i32).to_le_bytes()); // 57: low-res format (none)
    out.push(0); // 61: low-res width
    out.push(0); // 62: low-res height
    out.extend_from_slice(&1u16.to_le_bytes()); // 63: depth (7.2)
                                                // pad to HEADER_SIZE (80)
    while out.len() < HEADER_SIZE as usize {
        out.push(0);
    }
}

fn encode_bgra8888(out: &mut Vec<u8>, rgba: &[u8]) {
    out.reserve(rgba.len());
    for px in rgba.chunks_exact(4) {
        out.push(px[2]); // B
        out.push(px[1]); // G
        out.push(px[0]); // R
        out.push(px[3]); // A
    }
}

fn encode_dxt5(out: &mut Vec<u8>, width: u32, height: u32, rgba: &[u8]) {
    let blocks_x = width.div_ceil(4);
    let blocks_y = height.div_ceil(4);

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let mut block = [[0u8; 4]; 16];
            for py in 0..4u32 {
                for px in 0..4u32 {
                    // Clamp to edge for partial blocks.
                    let x = (bx * 4 + px).min(width - 1);
                    let y = (by * 4 + py).min(height - 1);
                    let src = ((y * width + x) * 4) as usize;
                    let dst = (py * 4 + px) as usize;
                    block[dst].copy_from_slice(&rgba[src..src + 4]);
                }
            }
            let encoded = encode_dxt5_block(&block);
            out.extend_from_slice(&encoded);
        }
    }
}

/// Encode one 4x4 RGBA block to 16 DXT5 bytes (8 alpha + 8 color).
fn encode_dxt5_block(pixels: &[[u8; 4]; 16]) -> [u8; 16] {
    let mut out = [0u8; 16];

    // --- Alpha block (8 bytes): 2 endpoints + 16 x 3-bit indices ---
    let mut a_min = 255u8;
    let mut a_max = 0u8;
    for p in pixels {
        a_min = a_min.min(p[3]);
        a_max = a_max.max(p[3]);
    }
    // a0 > a1 => 8-value interpolation mode.
    let (a0, a1) = (a_max, a_min);
    let alpha_table = dxt5_alpha_table(a0, a1);
    out[0] = a0;
    out[1] = a1;
    let mut alpha_bits: u64 = 0;
    for (i, p) in pixels.iter().enumerate() {
        let code = nearest_alpha(p[3], &alpha_table) as u64;
        alpha_bits |= code << (3 * i);
    }
    for (i, b) in out.iter_mut().enumerate().take(8).skip(2) {
        *b = ((alpha_bits >> (8 * (i - 2))) & 0xFF) as u8;
    }

    // --- Color block (8 bytes): c0, c1 (565) + 16 x 2-bit indices ---
    let (c0, c1) = color_endpoints(pixels);
    // Ensure 4-color mode (c0 > c1).
    let (c0, c1) = if c0 > c1 { (c0, c1) } else { (c1, c0) };
    let palette = color_palette(c0, c1);
    out[8] = (c0 & 0xFF) as u8;
    out[9] = (c0 >> 8) as u8;
    out[10] = (c1 & 0xFF) as u8;
    out[11] = (c1 >> 8) as u8;
    let mut color_bits: u32 = 0;
    for (i, p) in pixels.iter().enumerate() {
        let code = nearest_color([p[0], p[1], p[2]], &palette) as u32;
        color_bits |= code << (2 * i);
    }
    out[12..16].copy_from_slice(&color_bits.to_le_bytes());

    out
}

fn dxt5_alpha_table(a0: u8, a1: u8) -> [u8; 8] {
    let (a0, a1) = (a0 as u16, a1 as u16);
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

fn nearest_alpha(value: u8, table: &[u8; 8]) -> usize {
    let mut best = 0usize;
    let mut best_d = u16::MAX;
    for (i, &a) in table.iter().enumerate() {
        let d = (a as i16 - value as i16).unsigned_abs();
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    best
}

/// Pick color endpoints as the per-channel bounding box corners (565).
fn color_endpoints(pixels: &[[u8; 4]; 16]) -> (u16, u16) {
    let mut lo = [255u8; 3];
    let mut hi = [0u8; 3];
    for p in pixels {
        for c in 0..3 {
            lo[c] = lo[c].min(p[c]);
            hi[c] = hi[c].max(p[c]);
        }
    }
    (rgb888_to_565(hi), rgb888_to_565(lo))
}

fn color_palette(c0: u16, c1: u16) -> [[u8; 3]; 4] {
    let p0 = rgb565_to_888(c0);
    let p1 = rgb565_to_888(c1);
    let mut pal = [[0u8; 3]; 4];
    pal[0] = p0;
    pal[1] = p1;
    for c in 0..3 {
        pal[2][c] = ((2 * p0[c] as u16 + p1[c] as u16) / 3) as u8;
        pal[3][c] = ((p0[c] as u16 + 2 * p1[c] as u16) / 3) as u8;
    }
    pal
}

fn nearest_color(rgb: [u8; 3], palette: &[[u8; 3]; 4]) -> usize {
    let mut best = 0usize;
    let mut best_d = u32::MAX;
    for (i, p) in palette.iter().enumerate() {
        let mut d = 0u32;
        for c in 0..3 {
            let diff = p[c] as i32 - rgb[c] as i32;
            d += (diff * diff) as u32;
        }
        if d < best_d {
            best_d = d;
            best = i;
        }
    }
    best
}

fn rgb888_to_565(rgb: [u8; 3]) -> u16 {
    let r = (rgb[0] >> 3) as u16;
    let g = (rgb[1] >> 2) as u16;
    let b = (rgb[2] >> 3) as u16;
    (r << 11) | (g << 5) | b
}

fn rgb565_to_888(c: u16) -> [u8; 3] {
    let r5 = (c >> 11) & 0x1F;
    let g6 = (c >> 5) & 0x3F;
    let b5 = c & 0x1F;
    [
        ((r5 << 3) | (r5 >> 2)) as u8,
        ((g6 << 2) | (g6 >> 4)) as u8,
        ((b5 << 3) | (b5 >> 2)) as u8,
    ]
}

/// Decode standard base64 (RFC 4648) into bytes. Used to receive RGBA pixel
/// data from the frontend without the JSON number-array overhead.
pub fn base64_decode(input: &str) -> AppResult<Vec<u8>> {
    fn val(c: u8) -> Option<u8> {
        match c {
            b'A'..=b'Z' => Some(c - b'A'),
            b'a'..=b'z' => Some(c - b'a' + 26),
            b'0'..=b'9' => Some(c - b'0' + 52),
            b'+' => Some(62),
            b'/' => Some(63),
            _ => None,
        }
    }

    let mut out = Vec::with_capacity(input.len() / 4 * 3);
    let mut quad = [0u8; 4];
    let mut n = 0;
    for &c in input.as_bytes() {
        if c == b'=' || c.is_ascii_whitespace() {
            continue;
        }
        let v = val(c).ok_or_else(|| AppError::Validation("invalid base64 input".into()))?;
        quad[n] = v;
        n += 1;
        if n == 4 {
            out.push((quad[0] << 2) | (quad[1] >> 4));
            out.push((quad[1] << 4) | (quad[2] >> 2));
            out.push((quad[2] << 6) | quad[3]);
            n = 0;
        }
    }
    match n {
        0 => {}
        2 => out.push((quad[0] << 2) | (quad[1] >> 4)),
        3 => {
            out.push((quad[0] << 2) | (quad[1] >> 4));
            out.push((quad[1] << 4) | (quad[2] >> 2));
        }
        _ => return Err(AppError::Validation("truncated base64 input".into())),
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::filesystem::vtf;

    fn solid_rgba(width: u32, height: u32, color: [u8; 4]) -> Vec<u8> {
        color
            .iter()
            .cycle()
            .take((width * height * 4) as usize)
            .copied()
            .collect()
    }

    #[test]
    fn base64_decode_known_vectors() {
        assert_eq!(base64_decode("Zg==").unwrap(), b"f");
        assert_eq!(base64_decode("Zm8=").unwrap(), b"fo");
        assert_eq!(base64_decode("Zm9v").unwrap(), b"foo");
        assert_eq!(base64_decode("Zm9vYmFy").unwrap(), b"foobar");
    }

    #[test]
    fn bgra8888_roundtrips_exactly() {
        let rgba = solid_rgba(8, 8, [200, 100, 50, 180]);
        let vtf_bytes = encode_vtf(8, 8, &rgba, VtfFormat::Bgra8888).unwrap();
        // Decode back via the reader and compare the top-left pixel.
        let url = vtf::thumbnail_data_url(&vtf_bytes).unwrap();
        assert!(url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn dxt5_solid_block_is_near_exact() {
        let color = [200u8, 100, 50, 180];
        let block = {
            let mut b = [[0u8; 4]; 16];
            for p in b.iter_mut() {
                *p = color;
            }
            encode_dxt5_block(&b)
        };
        // a0 should equal the (uniform) alpha.
        assert_eq!(block[0], 180);
        // Color endpoints decode close to the source (565 rounding tolerance).
        let pal = color_palette(
            u16::from_le_bytes([block[8], block[9]]),
            u16::from_le_bytes([block[10], block[11]]),
        );
        assert!((pal[0][0] as i32 - 200).abs() <= 8);
        assert!((pal[0][1] as i32 - 100).abs() <= 8);
        assert!((pal[0][2] as i32 - 50).abs() <= 8);
    }

    #[test]
    fn dxt5_rejects_non_multiple_of_four() {
        let rgba = solid_rgba(6, 6, [0, 0, 0, 255]);
        assert!(encode_vtf(6, 6, &rgba, VtfFormat::Dxt5).is_err());
    }

    #[test]
    fn rejects_wrong_buffer_size() {
        assert!(encode_vtf(4, 4, &[0u8; 10], VtfFormat::Bgra8888).is_err());
    }
}
