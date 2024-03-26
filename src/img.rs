//  img.rs - bitmap functions
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


// Img is concerned mostly with reading and writing to BMP files, and converting between different bit formats


use std::mem;
use crate::bmp_format::{*};
use crate::img_data::{*};


//----------------------------------------------------------------------------
//  RGB FORMAT CONVERSION
//----------------------------------------------------------------------------

fn rgb565_to_888(a: u8, b: u8) -> [u8; 3] {
    let pixel = (b as u16) | ((a as u16) << 8);
    let mut output = [0 as u8, 0 as u8, 0 as u8];
    output[0] = ((pixel & 0x001F) << 3) as u8;
    output[0] |= ((pixel & 0x001C) >> 3) as u8;
    output[1] = ((pixel & 0x07E0) >> 3) as u8;
    output[1] |= ((pixel & 0x0600) >> 9) as u8;
    output[2] = ((pixel & 0xF800) >> 8) as u8;
    output[2] |= ((pixel & 0xE000) >> 13) as u8;
    output
}

fn rgb888_to_565(buf: &[u8]) -> [u8; 2] {
    let mut output: u16 = 0;
    output |= (buf[2] as u16 & 0xF8) >> 3;
    output |= (buf[1] as u16 & 0xFC) << 3;
    output |= (buf[0] as u16 & 0xF8) << 8;
    [ (output & 0xFF) as u8, ((output & 0xFF00) >> 8) as u8 ]
}

//----------------------------------------------------------------------------
//  IMG - STORE BASIC IMAGE DATA
//----------------------------------------------------------------------------

// Different formats supported by Img
#[derive(PartialEq, Copy, Clone)]
pub enum ImgFormat {
 	Argb8888 = 0,				// ARGB8888   4 bytes per pixel
 	Argb8565 = 1,				// ARGB8565   3 bytes per pixel
 	RleNew = 2,				    // Compressed ARGB8565
 }

// Img is a basic image data struct that may contain compressed (RleNew) data or uncompressed bitmap data
#[derive(PartialEq, Clone)]
pub struct Img {
    pub w: u32,
    pub h: u32,
    pub format: ImgFormat,
    pub data: Vec<u8>,
    pub rle_header: Option<Vec<u8>>,
}

impl Img {
    pub fn from_img_data(id: &ImgData) -> Self {
        Self {
            w: id.w as u32,
            h: id.h as u32,
            data: id.data.clone(),
            rle_header: Some(id.header.clone()),
            format: ImgFormat::RleNew,            
        }
    }

    pub fn from_bmp(bytes: &Vec<u8>) -> Result<Img, String> {
        if bytes.len() < BASIC_BMP_HEADER_SIZE {
            return Err("BMP file is too small.".to_string());
        }
        
        let h = unsafe { &*(bytes.as_ptr() as *const BMPHeaderClassic) };
        
        let mut height = h.height;
        let mut top_down = false;
        if h.height < 0 {
            top_down = true;
            height = -h.height;
        }

        if h.sig != 0x4D42 {
            return Err("File is not a BMP bitmap.".to_string());
        }
    
        if h.dib_header_size != 40 && h.dib_header_size != 108 && h.dib_header_size != 124 {
            return Err("BMP header format unrecognised.".to_string());
        }
    
        if h.planes != 1 || h.reserved1 != 0 || h.reserved2 != 0 {
            return Err("BMP is unusual, can't read it.".to_string());
        }
    
        if h.bpp != 16 && h.bpp != 24 && h.bpp != 32 {
            return Err("BMP must be RGB565 or RGB888 or ARGB8888.".to_string());
        }
        
        if h.bpp == 16 && h.compression_type != 3 {
            return Err("16bpp BMP is missing bitfields.".to_string());
        }
        
        if (h.bpp == 24 || h.bpp == 32) && (h.compression_type != 0 && h.compression_type != 3) {
            return Err("BMP must be uncompressed.".to_string());
        }
    
        let mut image_data_size = h.image_data_size as usize;
        let mut row_size = image_data_size as usize / height as usize;
        let bytes_per_pixel = h.bpp as usize / 8;
        if row_size < (h.width as usize * bytes_per_pixel) {
            image_data_size = bytes.len() - h.offset as usize;        // calculate it ourselves, the header could be wrong
            row_size = image_data_size / height as usize;
            if row_size < (h.width as usize * bytes_per_pixel) {
                return Err("BMP image_data_size doesn't make sense!".to_string());
            }
        }
          
        let target_size: usize;
        
        let mut img = Img {
            w: h.width as u32,
            h: height as u32,
            format: ImgFormat::Argb8888,
            data: Vec::new(),
            rle_header: None,
        };

        if h.bpp == 16 {
            img.format = ImgFormat::Argb8565;
            target_size = img.w as usize * img.h as usize * 3;            
        } else {
            target_size = img.w as usize * img.h as usize * 4;
        }       

        img.data = vec![0; target_size];

        if h.bpp == 16 {
            // This pathway is untested
            if h.bmi_colors[0] != 0xF800 || h.bmi_colors[1] != 0x07E0 || h.bmi_colors[2] != 0x001F {
                return Err("16bpp BMP has unusual bitfields, should be RGB565.".to_string());
            }
            for y in 0..img.h as usize {
                let row = if top_down { y } else { img.h as usize - y - 1 };
                let bmp_offset = h.offset as usize + row as usize * row_size as usize;
                for x in 0..img.w as usize {
                    let a = bytes[bmp_offset + 2 * x ];
                    let b = bytes[bmp_offset + 2 * x + 1];
                    let dest_data = [0xFF, a, b];
                    let dest_offset = (y * img.w as usize + x) * 3;
                    img.data[dest_offset as usize..dest_offset as usize + 3].copy_from_slice(&dest_data);
                }
            }
        } else if h.bpp == 32 && h.dib_header_size > 40 {
            let h4 = unsafe { &*(bytes.as_ptr() as *const BMPHeaderV4) };
            if h.compression_type == 3 {
                if h4.rgba_masks[0] != 0x00FF0000 || h4.rgba_masks[1] != 0x0000FF00 || h4.rgba_masks[2] != 0x000000FF || h4.rgba_masks[3] != 0xFF000000 {
                    return Err("32bpp BMP bitfields not ARGB8888.".to_string());
                }
            }
            for y in 0..img.h as usize {
                let row = if top_down { y } else { img.h as usize - y - 1 };
                let bmp_offset = h.offset as usize + row * row_size;
                let dest_offset = y * img.w as usize * 4;
                img.data[dest_offset as usize..dest_offset as usize + img.w as usize * 4].copy_from_slice(&bytes[bmp_offset as usize..bmp_offset as usize + img.w as usize * 4]);
            }
        } else if h.bpp == 24 {
            // This pathway is untested... 
            if h.compression_type == 3 {
                if h.bmi_colors[0] != 0xFF0000 || h.bmi_colors[1] != 0x00FF00 || h.bmi_colors[2] != 0x0000FF {
                    return Err("24bpp BMP bitfields not RGB888.".to_string());
                }
            }
            for y in 0..img.h as usize {
                let row = if top_down { y } else { img.h as usize - y - 1 };
                let bmp_offset = h.offset as usize + row * row_size;
                for x in 0..img.w as usize {
                    let mut pixel: u32 = 0xFF;
                    pixel <<= 8;
                    pixel |= bytes[bmp_offset + x as usize * 3] as u32;
                    pixel <<= 8;
                    pixel |= bytes[bmp_offset + x as usize * 3 + 1] as u32;
                    pixel <<= 8;
                    pixel |= bytes[bmp_offset + x as usize * 3 + 2] as u32;
                    let dest_offset = (y * img.w as usize + x) * 4;
                    img.data[dest_offset as usize..dest_offset as usize + 4].copy_from_slice(&pixel.to_le_bytes());
                }
            }
        } else {
            return Err("32bpp BMP but using older BMP header?!?".to_string());
        }
        Ok(img)
    }
    
    fn argb8565_to_argb8888(&mut self) {
        if self.format != ImgFormat::Argb8565 {
            panic!("Expected Argb8565");
        }
        // increase bit depth
        let mut new_img = Img {
            w: self.w,
            h: self.h,
            format: ImgFormat::Argb8888,
            data: vec![0; self.w as usize * self.h as usize * 4],
            rle_header: None,
        };
        // Go row by row, pixel by pixel
        for y in 0..self.h as usize {
            for x in 0..self.w as usize {
                let offset_s = (self.w as usize * y + x) * 3;
                let p = &self.data[offset_s..offset_s+3];
                let offset_d = (self.w as usize * y + x) * 4;
                let output = &mut new_img.data[offset_d..offset_d+4];
                // Read in 3 bytes, convert to 4 bytes
                // Alpha byte is the same, RGB parts need converting from 565 to 888
                output[3] = p[0];
                let rgb = rgb565_to_888(p[1], p[2]);
                output[0] = rgb[0];
                output[1] = rgb[1];
                output[2] = rgb[2];
            }
        }
        *self = new_img;
    }

    fn argb8888_to_argb8565(&mut self) {
        if self.format != ImgFormat::Argb8888 {
            panic!("Expected Argb8888");
        }
        // reduce bit depth
        let mut new_img = Img {
            w: self.w,
            h: self.h,
            format: ImgFormat::Argb8565,
            data: vec![0; self.w as usize * self.h as usize * 3],
            rle_header: None,
        };
        // Go row by row, pixel by pixel
        for y in 0..self.h as usize {
            for x in 0..self.w as usize {
                let offset_i = (self.w as usize * y + x) * 4;
                let p = &self.data[offset_i..offset_i+4];
                let offset_o = (self.w as usize * y + x) * 3;
                let output = &mut new_img.data[offset_o..offset_o+3];
                // Convert 4 bytes to 3 bytes
                // Alpha byte is the same, rgb parts need converting
                output[0] = p[0];
                let rgb565 = rgb888_to_565(&p[1..4]);
                output[1] = rgb565[0];
                output[2] = rgb565[1];
            }
        }
        *self = new_img;
    }    

    fn argb8565_to_rle_new(&mut self) {         // designed to match the competitor's (inferior?) algorithm
        if self.format != ImgFormat::Argb8565 {
            panic!("Expect Argb8565.");
        }        

        let mut dest_header = Vec::<u8>::with_capacity(self.h as usize * 4);
        let mut dest_data = Vec::<u8>::with_capacity(self.data.len());  // allocate plenty of bytes

        let mut dest_offset = self.h as usize * 4;  // dest pixels start after header

        println!("calculated rle header size {}", dest_offset);
        // compress the data
        // for each row
        let row_width = self.w as usize * 3;
        for y in 0..self.h as usize {
            // get the pixels for this row
            let row_src_data = &self.data[(y * row_width)..((y + 1) * row_width)];
            let mut row_dest_data = Vec::<u8>::new();
            let mut offset = 0;

            while offset < (row_width - 6) {
                let mut pixel_a: Vec<u8> = row_src_data[offset..offset+3].to_vec();
                let mut pixel_b: Vec<u8> = row_src_data[offset+3..offset+6].to_vec();

                if pixel_a == pixel_b {
                    // start a REPEATING block
                    let repeating_pixel = pixel_a.clone();
                    let mut repeating_count = 1;        // at least the first two are repeating, but we're going to count one before starting loop
                    offset += 3;
                    pixel_a = row_src_data[offset..offset+3].to_vec();
                    // count how many pixels are in the REPEATING block
                    while repeating_count < 127 && repeating_pixel == pixel_a {
                        repeating_count += 1;
                        offset += 3;
                        if offset > (row_width - 3) {
                            break;
                        }
                        pixel_a = row_src_data[offset..offset+3].to_vec();
                    }
                    // we have repeating_count pixels
                    // save the repeating pixels
                    row_dest_data.push(0x80 | repeating_count);
                    row_dest_data.extend(&repeating_pixel);
                } else {
                    // start a NON-REPEATING block
                    let nr_start = offset;
                    let mut nr_count = 0;
                    // count how many pixels are in the NON-REPEATING block... 
                    while nr_count < 127 && pixel_a != pixel_b {
                        nr_count += 1;
                        pixel_a = pixel_b;
                        offset += 3;
                        if offset > (row_width - 6) {
                            break;
                        }
                        pixel_b = row_src_data[offset+3..offset+6].to_vec();
                    }
                    // we have nr_count pixels, starting from nr_start
                    row_dest_data.push(nr_count);
                    row_dest_data.extend(row_src_data[nr_start..offset].iter());
                }
            }
            // finish off the row
            // just store whatever pixels are left as non-repeating ?!
            let count = (row_width - offset) / 3;
            row_dest_data.push(count as u8); // non-repeating
            row_dest_data.extend(&row_src_data[offset..row_width]);
            // Save row offset, size to dest_header
            // row_size is 11 bits of a u16, the hi bits of the offset go into the other
            // so the offset is 16+5 = 21 bits
            if row_dest_data.len() > 0x7FF {
                panic!("rle compressed row is too long to be stored!");
            }
            if dest_offset > 0x1FFFFF {
                panic!("rle compressed data is too big to be stored!");
            }
            let row_dest_size = row_dest_data.len() * 32; // preshifted
            let row_dest_size_bits0: u8 = (row_dest_size & 0xFF) as u8;
            let row_dest_size_bits1: u8 = ((row_dest_size & 0xFF00) >> 8) as u8;
            let row_dest_offset_bits0: u8 = (dest_offset & 0xFF) as u8;
            let row_dest_offset_bits1: u8 = ((dest_offset & 0xFF00) >> 8) as u8;
            let row_dest_offset_bits2: u8 = ((dest_offset & 0x1F0000) >> 16) as u8;
            dest_header.push(row_dest_offset_bits0);
            dest_header.push(row_dest_offset_bits1);
            dest_header.push(row_dest_offset_bits2 | row_dest_size_bits0);
            dest_header.push(row_dest_size_bits1);
            
            dest_offset += row_dest_data.len();
            dest_data.extend(&row_dest_data);
        }
                    
        *self = Img {
            w: self.w,
            h: self.h,
            format: ImgFormat::RleNew,
            data: dest_data,
            rle_header: Some(dest_header),
        };
    }

    fn rle_new_to_argb8565(&mut self) {
        if self.format != ImgFormat::RleNew {
            panic!("Expected RleNew");
        }
        // allocate memory for the new image
        let mut new_img = Img {
            w: self.w,
            h: self.h,
            format: ImgFormat::Argb8565,
            data: vec![0; self.w as usize * self.h as usize * 3],
            rle_header: None,
        };

        // decompress the data
        let mut bytes_out = 0;
        let mut bytes_in = 0;
        while bytes_in < self.data.len() {
            let cmd = self.data[bytes_in]; // read a byte
            bytes_in += 1;
            if (cmd & 0x80) != 0 {
                // Repeat the pixel
                let count = (cmd & 0x7F) as usize;
                let data = [
                    self.data[bytes_in],
                    self.data[bytes_in + 1],
                    self.data[bytes_in + 2],
                ];
                bytes_in += 3;
                for _ in 0..count {
                    new_img.data[bytes_out] = data[0];
                    new_img.data[bytes_out + 1] = data[1];
                    new_img.data[bytes_out + 2] = data[2];
                    bytes_out += 3;
                }
            } else {
                // Normal pixel data
                let count = cmd as usize * 3;
                new_img.data[bytes_out..bytes_out + count]
                    .copy_from_slice(&self.data[bytes_in..bytes_in + count]);
                bytes_out += count;
                bytes_in += count;
            }
        }
        *self = new_img;
    }

    pub fn convert_format(&mut self, new_format: ImgFormat) {
        if new_format == ImgFormat::Argb8888 {
            if self.format == ImgFormat::RleNew {                
                self.rle_new_to_argb8565();             // Convert to ARGB8565 first
            }
            if self.format == ImgFormat::Argb8565 {
                self.argb8565_to_argb8888();
            }
        }
        if new_format == ImgFormat::Argb8565 {
            if self.format == ImgFormat::RleNew {
                self.rle_new_to_argb8565();
            } else if self.format == ImgFormat::Argb8888 {
                self.argb8888_to_argb8565();
            }
        }
        if new_format == ImgFormat::RleNew {
            if self.format == ImgFormat::Argb8888 {                
                self.argb8888_to_argb8565();            // reduce bit depth first
            }
            if self.format == ImgFormat::Argb8565 {
                self.argb8565_to_rle_new();             // compress it
            }
        }
    }

    pub fn to_bmp(&self) -> Vec<u8> {
        let header = BMPHeaderV5::new(self.w, self.h, 32);
        let header_size = mem::size_of::<BMPHeaderV5>();
        let mut img = self.clone();
        if img.format != ImgFormat::Argb8888 {
            img.convert_format(ImgFormat::Argb8888);
        }
        let dest_row_size = header.image_data_size as usize / img.h as usize;
        let mut b: Vec<u8> = vec![0; header.file_size as usize];

        let header_bytes = header.to_bytes();
        (0..header_size).for_each(|i| b[i] = header_bytes[i]);

        let mut d_offset = header_size;
        for y in 0..img.h as usize {
            let src_row_data = &img.data[(y * img.w as usize * 4)..((y + 1) * img.w as usize * 4)];
            let src_row_len = img.w as usize * 4;
            (0..src_row_len).for_each(|i| b[d_offset + i] = src_row_data[i]);
            d_offset += dest_row_size;
        }
        b
    }
}
    
