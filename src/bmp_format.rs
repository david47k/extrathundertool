//  bmp_format.rs - bitmap file format
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)

//----------------------------------------------------------------------------
//  BMP FILE FORMAT HEADERS
//----------------------------------------------------------------------------

#[repr(packed,C)]
pub struct BMPHeaderClassic {
    pub sig: u16,               // "BM"                                              BITMAPFILEHEADER starts here
    pub file_size: u32,          // unreliable - size in bytes of file
    pub reserved1: u16,         // 0
    pub reserved2: u16,         // 0
    pub offset: u32,            // offset to start of image data                    BITMAPFILEHEADER ends here
    pub dib_header_size: u32,     // 40 = size of BITMAPINFOHEADER                    BITMAPINFOHEADER starts here, BITMAPINFO starts here, BITMAPV4HEADER starts here
    pub width: i32,             // pixels
    pub height: i32,            // pixels
    pub planes: u16,            // 1
    pub bpp: u16,               // 16
    pub compression_type: u32,   // 0=BI_RGB. 3=BI_BITFIELDS. Must be set to BI_BITFIELDS for RGB565 format.
    pub image_data_size: u32,     // including padding - unreliable
    pub hres: u32,              // pixels per metre
    pub vres: u32,              // pixels per meter
    pub clr_used: u32,           // colors in image, or 0
    pub clr_important: u32,      // colors in image, or 0                            BITMAPINFOHEADER ends here
    pub bmi_colors: [u32; 3],    // masks for R, G, B components (for 16bpp)
}

pub const BASIC_BMP_HEADER_SIZE: usize = std::mem::size_of::<BMPHeaderClassic>() - 12;

#[repr(packed,C)]
pub struct BMPHeaderV4 {
    pub sig: u16,               // "BM"                                              BITMAPFILEHEADER starts here
    pub file_size: u32,          // unreliable - size in bytes of file
    pub reserved1: u16,         // 0
    pub reserved2: u16,         // 0
    pub offset: u32,            // offset to start of image data                    BITMAPFILEHEADER ends here
    pub dib_header_size: u32,     // 108 for BITMAPV4HEADER                            BITMAPINFOHEADER starts here, BITMAPINFO starts here, BITMAPV4HEADER starts here
    pub width: i32,             // pixels
    pub height: i32,            // pixels
    pub planes: u16,            // 1
    pub bpp: u16,               // 16
    pub compression_type: u32,   // 3=BI_BITFIELDS. Must be set to BI_BITFIELDS for RGB565 format.
    pub image_data_size: u32,     // including padding - unreliable
    pub hres: u32,              // pixels per metre
    pub vres: u32,              // pixels per meter
    pub clr_used: u32,           // colors in image, or 0
    pub clr_important: u32,      // colors in image, or 0                            BITMAPINFOHEADER ends here
    pub rgba_masks: [u32; 4],    // masks for R,G,B,A components (if BI_BITFIELDS)    BITMAPINFO ends here
    pub cs_type: u32,
    pub bv4_endpoints: [u32; 9],
    pub gammas: [u32; 3],
}

#[repr(packed,C)]
pub struct BMPHeaderV5 {
    pub sig: u16,               // "BM"                                             BITMAPFILEHEADER starts here
    pub file_size: u32,          // unreliable - size in bytes of file
    pub reserved1: u16,         // 0
    pub reserved2: u16,         // 0
    pub offset: u32,            // offset to start of image data                    BITMAPFILEHEADER ends here
    pub dib_header_size: u32,     // 124 for BITMAPV5HEADER                            BITMAPINFOHEADER starts here, BITMAPINFO starts here, BITMAPV5HEADER starts here
    pub width: i32,             // pixels
    pub height: i32,            // pixels
    pub planes: u16,            // 1
    pub bpp: u16,               // 16
    pub compression_type: u32,   // 3=BI_BITFIELDS. Must be set to BI_BITFIELDS for RGB565 format.
    pub image_data_size: u32,     // including padding - unreliable
    pub hres: u32,              // pixels per metre
    pub vres: u32,              // pixels per meter
    pub clr_used: u32,           // colors in image, or 0
    pub clr_important: u32,      // colors in image, or 0                              BITMAPINFOHEADER ends here
    pub rgba_masks: [u32; 4],    // masks for R,G,B,A components (if BI_BITFIELDS)     BITMAPINFO ends here
    pub intent: u32,
    pub profile_data: u32,
    pub profile_size: u32,
    pub reserved: u32,          //                                                     BITMAPV5HEADER ends here
    pub cs_type: u32,
    pub bv4_endpoints: [u32; 9],
    pub gammas: [u32; 3],
}

impl BMPHeaderV5 {
    pub fn new(width: u32, height: u32, bpp: u8) -> Self {
        const V5SIZE: usize = 138;
        if V5SIZE != std::mem::size_of::<BMPHeaderV5>() {
            panic!("BMPHeaderV5 is not 138 bytes!");
        }
        let row_size = (((bpp as u32 / 8) * width) + 3) & 0xFFFFFFFC;
        let mut dest = BMPHeaderV5 {
            reserved1: 0,
            reserved2: 0,
            clr_used: 0,
            clr_important: 0,
            intent: 0,
            profile_data: 0,
            profile_size: 0,
            reserved: 0,
            cs_type: 0,
            bv4_endpoints: [ 0, 0, 0, 0, 0, 0, 0, 0, 0, ],
            gammas: [ 0, 0, 0 ],
            sig: 0x4D42,
            offset: V5SIZE as u32,
            dib_header_size: 124,       // V5SIZE - 14
            width: width as i32,
            height: -(height as i32),
            planes: 1,
            bpp: bpp as u16,
            compression_type: 0,
            rgba_masks: [0; 4],
            image_data_size: 0,
            file_size: 0,
            hres: 2835,
            vres: 2835,
        };
        if bpp == 16 {
            dest.compression_type = 3;
            dest.rgba_masks[0] = 0xF800;
            dest.rgba_masks[1] = 0x07E0;
            dest.rgba_masks[2] = 0x001F;
        } else if bpp == 32 {
            dest.compression_type = 3;
            dest.rgba_masks[0] = 0xFF0000;
            dest.rgba_masks[1] = 0x00FF00;
            dest.rgba_masks[2] = 0x0000FF;
            dest.rgba_masks[3] = 0xFF000000;
        } else if bpp == 24 {
            dest.compression_type = 0;
        }
        dest.image_data_size = row_size * height;
        dest.file_size = dest.image_data_size + V5SIZE as u32;
        dest.hres = 2835;
        dest.vres = 2835;
        return dest;
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        const V5SIZE: usize = 138;
        if V5SIZE != std::mem::size_of::<BMPHeaderV5>() {
            panic!("BMPHeaderV5 is not 138 bytes!");
        }
        let mut bytes: Vec<u8> = vec![0; V5SIZE];
        unsafe {
            let header_bytes = std::mem::transmute::<&BMPHeaderV5,&[u8; V5SIZE]>(self);
            (0..V5SIZE).for_each(|i| bytes[i] = header_bytes[i]);
        }
        bytes
    }
}
