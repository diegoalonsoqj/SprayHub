//! Minimal VTF thumbnail decoder.
//!
//! Valve Texture Format files embed a small low-resolution thumbnail (almost
//! always DXT1, up to 16x16) right after the header. We decode that thumbnail
//! to RGB, wrap it in an uncompressed BMP, and base64-encode it into a data URL
//! suitable for an `<img>` tag. This keeps the dependency footprint at zero
//! image crates. Decoding the full-resolution mipmap is left as future work
//! (see roadmap: PNG→VTF / VTFLib wrapper).

use crate::domain::error::{AppError, AppResult};

const VTF_SIGNATURE: &[u8; 4] = b"VTF\0";
/// Image format enum value for DXT1 in the VTF spec.
const IMAGE_FORMAT_DXT1: i32 = 13;

/// Decode a VTF file's embedded thumbnail into a `data:image/bmp;base64,...` URL.
pub fn thumbnail_data_url(bytes: &[u8]) -> AppResult<String> {
    let (width, height, rgb) = decode_low_res(bytes)?;
    let bmp = encode_bmp_24(width, height, &rgb);
    Ok(format!("data:image/bmp;base64,{}", base64_encode(&bmp)))
}

/// Parse the header and decode the low-res thumbnail to RGB888.
fn decode_low_res(bytes: &[u8]) -> AppResult<(u32, u32, Vec<u8>)> {
    if bytes.len() < 64 || &bytes[0..4] != VTF_SIGNATURE {
        return Err(AppError::Validation("not a valid VTF file".into()));
    }

    let header_size = read_u32(bytes, 12) as usize;
    let low_res_format = read_i32(bytes, 57);
    let low_res_w = bytes[61] as u32;
    let low_res_h = bytes[62] as u32;

    if low_res_w == 0 || low_res_h == 0 {
        return Err(AppError::Validation(
            "VTF has no embedded thumbnail".into(),
        ));
    }
    if low_res_format != IMAGE_FORMAT_DXT1 {
        return Err(AppError::Validation(format!(
            "unsupported thumbnail format: {low_res_format}"
        )));
    }

    let block_count = blocks_for(low_res_w) * blocks_for(low_res_h);
    let expected = (block_count * 8) as usize;
    let start = header_size;
    let end = start
        .checked_add(expected)
        .ok_or_else(|| AppError::Validation("thumbnail size overflow".into()))?;
    if end > bytes.len() {
        return Err(AppError::Validation(
            "VTF thumbnail data is truncated".into(),
        ));
    }

    let rgb = decode_dxt1(&bytes[start..end], low_res_w, low_res_h);
    Ok((low_res_w, low_res_h, rgb))
}

fn blocks_for(dim: u32) -> u32 {
    dim.div_ceil(4).max(1)
}

/// Decode DXT1-compressed data into a row-major RGB888 buffer.
fn decode_dxt1(data: &[u8], width: u32, height: u32) -> Vec<u8> {
    let mut out = vec![0u8; (width * height * 3) as usize];
    let blocks_x = blocks_for(width);
    let blocks_y = blocks_for(height);

    for by in 0..blocks_y {
        for bx in 0..blocks_x {
            let block_index = (by * blocks_x + bx) as usize;
            let offset = block_index * 8;
            let c0 = read_u16(data, offset);
            let c1 = read_u16(data, offset + 2);
            let lookup = read_u32(data, offset + 4);

            let palette = build_palette(c0, c1);

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

fn build_palette(c0: u16, c1: u16) -> [(u8, u8, u8); 4] {
    let (r0, g0, b0) = rgb565_to_888(c0);
    let (r1, g1, b1) = rgb565_to_888(c1);
    let mut palette = [(0u8, 0u8, 0u8); 4];
    palette[0] = (r0, g0, b0);
    palette[1] = (r1, g1, b1);
    if c0 > c1 {
        palette[2] = (
            mix(r0, r1, 2, 1),
            mix(g0, g1, 2, 1),
            mix(b0, b1, 2, 1),
        );
        palette[3] = (
            mix(r0, r1, 1, 2),
            mix(g0, g1, 1, 2),
            mix(b0, b1, 1, 2),
        );
    } else {
        palette[2] = (
            mix(r0, r1, 1, 1),
            mix(g0, g1, 1, 1),
            mix(b0, b1, 1, 1),
        );
        // 1-bit alpha mode: index 3 is transparent black.
        palette[3] = (0, 0, 0);
    }
    palette
}

/// Weighted average: (a*wa + b*wb) / (wa + wb).
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

    // BITMAPFILEHEADER
    out.extend_from_slice(b"BM");
    out.extend_from_slice(&file_size.to_le_bytes());
    out.extend_from_slice(&0u16.to_le_bytes()); // reserved1
    out.extend_from_slice(&0u16.to_le_bytes()); // reserved2
    out.extend_from_slice(&offset.to_le_bytes());

    // BITMAPINFOHEADER
    out.extend_from_slice(&info_header.to_le_bytes());
    out.extend_from_slice(&(width as i32).to_le_bytes());
    out.extend_from_slice(&(height as i32).to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // planes
    out.extend_from_slice(&24u16.to_le_bytes()); // bpp
    out.extend_from_slice(&0u32.to_le_bytes()); // compression BI_RGB
    out.extend_from_slice(&pixel_data_size.to_le_bytes());
    out.extend_from_slice(&2835i32.to_le_bytes()); // x ppm (~72 dpi)
    out.extend_from_slice(&2835i32.to_le_bytes()); // y ppm
    out.extend_from_slice(&0u32.to_le_bytes()); // colors used
    out.extend_from_slice(&0u32.to_le_bytes()); // important colors

    // Pixel data, bottom-up, BGR.
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
    fn dxt1_solid_block_decodes() {
        // Two identical white endpoints, all indices 0 => 4x4 white block.
        let mut block = Vec::new();
        block.extend_from_slice(&0xFFFFu16.to_le_bytes());
        block.extend_from_slice(&0x0000u16.to_le_bytes());
        block.extend_from_slice(&0x00000000u32.to_le_bytes());
        let rgb = decode_dxt1(&block, 4, 4);
        assert_eq!(rgb.len(), 4 * 4 * 3);
        assert_eq!(&rgb[0..3], &[255, 255, 255]);
    }

    #[test]
    fn bmp_header_is_well_formed() {
        let rgb = vec![10u8; 4 * 4 * 3];
        let bmp = encode_bmp_24(4, 4, &rgb);
        assert_eq!(&bmp[0..2], b"BM");
        // 14 + 40 header + 4*4*3 pixels (no padding needed for width 4)
        assert_eq!(bmp.len(), 14 + 40 + 4 * 4 * 3);
    }

    #[test]
    fn rejects_non_vtf() {
        assert!(thumbnail_data_url(b"not a vtf file at all 0123456789").is_err());
    }
}
