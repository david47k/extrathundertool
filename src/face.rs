//  face.rs - watch face file structure and methods
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


// iter::chain, concat
// struct-to-iter: bevy_reflect
// casting: bytemuck

//#![allow(unused_variables)]
//#![allow(dead_code)]


use serde::{Serialize, Deserialize};
use crate::util::{*};
use crate::img_data::{ImgData, DumpFormat};
use crate::elements::{*};
use crate::digits::Digits;


// FACEN STARTS HERE

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FaceN 
{
    pub type_str: String,
    pub rev: u16,
    pub tpls: u16,
    pub api_ver: u16,
    pub unknown: u16,
    pub preview_img_data: ImgData,
    pub digits: Vec<Digits>,
    pub elements: Vec<Element>,
}

impl FaceN 
{
    pub fn from_bin(file_data: &[u8]) -> FaceN {
        let mut f = FaceN {
            type_str: "extrathunder watchface".to_string(),
            rev: 0,
            tpls: 0,
            api_ver:   get_u16(file_data, 0),
            unknown:   get_u16(file_data, 2),
            preview_img_data: ImgData::from_owh(file_data, 4),
            digits: Vec::new(),
            elements: Vec::new(),
        };
        let d_offset = get_u16(file_data, 12);
        let e_offset = get_u16(file_data, 14);

        // read digits, if they exist
        if d_offset != 0 {
            // we start at d_offset
            let mut offset: usize = d_offset.into();
            let digits_size: usize = std::mem::size_of::<crate::binary_face_n::DigitsHeader>();
                
            // Read the introduction to the digit section 0x0101
            let dss = get_u16(file_data, offset);
            if dss != 0x0101 {
                println!("WARNING: Unknown start to digits section: 0x{:04X}", dss);
            }
            offset += 2;
            let mut digits_count = 0;
            while offset < e_offset.into() {
                let digits = Digits::from_bin(file_data, offset, digits_count);
                f.digits.push(digits);
                digits_count += 1;
                offset += digits_size;
            }
        }

        // read elements
        let mut offset = e_offset as usize;
        loop {
        	let one = file_data[offset];
        	let e_type = file_data[offset+1];
            if one == 0 {
                // End of header section        
                break;
            }
            // print!("Loading e_type {} ... ", e_type);
            let e = Element::from_bin(file_data, offset);            
            if e == Element::Unknown {
                panic!("ERROR: Unknown Element {}", e_type);
            }
            offset += e.bin_size();
            f.elements.push(e);
            // println!("done.");
        }
        f // return the FaceN struct
    }

    pub fn to_bin(&self) -> Vec<u8> {
        let digits_header_size = if self.digits.len() > 0 {
            2 + self.digits.len() * 83
        } else { 0 };
        let mut data: Vec<u8> = Vec::new();
        data.extend(self.api_ver.to_le_bytes());
        data.extend(self.unknown.to_le_bytes());
        data.extend([0, 0, 0, 0]);                          // we will fill this in later once we know the offset
        data.extend(self.preview_img_data.w.to_le_bytes());
        data.extend(self.preview_img_data.h.to_le_bytes());
        let dh_offset: u16 = if self.digits.len() > 0 { 16 } else { 0 };
        data.extend(dh_offset.to_le_bytes());
        let bh_offset = 16 + digits_header_size as u16;
        data.extend(bh_offset.to_le_bytes());

        // we cant calculate offsets, until we know the size of all the headers
        let mut total_header_size = 16;                     // size of the main header
        total_header_size += digits_header_size;            // size of the digits section
        total_header_size += self.elements.iter().map(|el| el.bin_size()).sum::<usize>();   // size of all the elements
        total_header_size += 2;                             // there are two zero bytes that mark the end of the elements section
                
        let mut blob_data: Vec<u8> = Vec::new();
        let header_align = get_align_diff(total_header_size as u32);
        let mut blob_offset: u32 = total_header_size as u32 + header_align as u32;        // align it to 32-bit

        // just save zeros for the digits headers for now...
        data.extend(vec![0; digits_header_size]);

        // create each of the elements binary headers and push their image data
        for el in self.elements.iter() {
            let expected_size = el.bin_size();
            let el_data = el.to_bin(&mut blob_data, &mut blob_offset);
            if expected_size != el_data.len() {
                panic!("Size of binary element generated does not match expected size!");
            } 
            data.extend( el_data );
            pad_it(&mut blob_data);
            align_it(&mut blob_offset);
        }
        data.extend([0, 0]);       // this ends the elements header section

        // confirm data size matches expected
        if data.len() != total_header_size {
            panic!("header sizes mismatch!");
        }

        // go back and fill in the digits section
        if self.digits.len() > 0 {
            let mut dh_offset: usize = dh_offset as usize;
            put_u16(&mut data, dh_offset, 0x0101);
            dh_offset += 2;     //data.extend([1, 1]);       // this is an introductory sequence to the digits section!
            
        
            for (n, d) in self.digits.iter().enumerate() {
                let mut dh: Vec<u8> = Vec::new();
                dh.push(n as u8);
                for id in d.img_data.iter() {
                    let mut bd = id.to_bin();
                    pad_it(&mut bd);
                    dh.extend(blob_offset.to_le_bytes());       // offset of blob u32
                    dh.extend(id.w.to_le_bytes());              // width of blob u16
                    dh.extend(id.h.to_le_bytes());              // height of blob u16
                    blob_offset += bd.len() as u32;
                    blob_data.extend(bd);                
                }
                dh.extend(d.unknown.to_le_bytes());
                if dh.len() != 83 {
                    panic!("Digits header size is not 83 bytes! Too few digits?");
                }
                
                // copy the digits header into the main header
                for (i,b) in dh.iter().enumerate() {
                    data[dh_offset+i] = *b;
                }
                dh_offset += 83;
            }
        }

        // save the preview image data, and store the offset in the file header
        blob_data.extend(self.preview_img_data.to_bin());
        pad_it(&mut blob_data);
        put_u32(&mut data, 4, blob_offset);

        // return allllll the binary data of the file
        data.extend(vec![0; header_align as usize]);   // align the start of the blob
        data.extend(blob_data);
        data
    }

    fn gen_name(prefix: &str, numbers: &[usize], format: &DumpFormat) -> String {
        let ext = match format {
            DumpFormat::BIN => ".bin",
            DumpFormat::RAW => ".raw",
            DumpFormat::BMP => ".bmp",
        };
        let mut file_name: String = format!("{}", prefix);
        for n in numbers {
            file_name += "_";
            file_name += &n.to_string();
        }
        file_name += ext;
        return file_name;
    }

    pub fn generate_file_names(&mut self, format: &DumpFormat) {
        let overwrite: bool = true;

        // write the preview filename
        self.preview_img_data.set_file_name( &Self::gen_name("preview", &[], format), overwrite );

        // write the digits filenames
        for (n,ds) in self.digits.iter_mut().enumerate() {
            for (i,id) in ds.img_data.iter_mut().enumerate() {
                id.set_file_name( &Self::gen_name("digit", &[n, i], format), overwrite );
            }
        }        
        // write the elements filenames
        let mut image_counter: usize = 0;
        let mut day_name_counter: usize = 0;
        let mut battery_fill_counter: usize = 0;
        for el in self.elements.iter_mut() {
            match el {
                Element::Image(e) => {          // one image, can be multiple Images
                    e.img_data.set_file_name( &Self::gen_name("image", &[image_counter], format), overwrite );
                    image_counter += 1;
                },
                Element::TimeNum(_) => {},      // no images
                Element::DayName(e) => {        // seven images
                    for i in 0..e.img_data.len() {                        
                        e.img_data[i].set_file_name( &Self::gen_name("day_name", &[day_name_counter, i], format), overwrite );
                    }
                    day_name_counter += 1;
                },
                Element::BatteryFill(e) => {    // three images
                    e.img_data.set_file_name(  &Self::gen_name("battery_fill", &[battery_fill_counter, 0], &format), overwrite );
                    e.image_data1.set_file_name( &Self::gen_name("battery_fill", &[battery_fill_counter, 1], &format), overwrite );
                    e.image_data2.set_file_name( &Self::gen_name("battery_fill", &[battery_fill_counter, 2], &format), overwrite );
                    battery_fill_counter += 1;
                }, 
                Element::HeartRateNum(_) => {}, // no images
                Element::StepsNum(_) => {},     // no images
                Element::KCalNum(_) => {},      // no images
                Element::TimeHand(e) => {       // one image, h_type in filename                    
                    e.img_data.set_file_name( &Self::gen_name("time_hand", &[e.h_type as usize], &format), overwrite );
                }, 
                Element::DayNum(_) => {},       // no images
                Element::MonthNum(_) => {},     // no images
                Element::BarDisplay(e) => {     // variable images
                    for i in 0..e.img_data.len() {                        
                        e.img_data[i].set_file_name( &Self::gen_name("bar_display", &[e.b_type as usize, i], &format), overwrite );
                    }
                }, 
                Element::Weather(e) => {        // variable images
                    for i in 0..e.img_data.len() {                        
                        e.img_data[i].set_file_name( &Self::gen_name("weather", &[i], &format), overwrite );
                    }
                }, 
                Element::Unknown29(_) => {},    // no images
                Element::Dash(e) => {      // one image
                    e.img_data.set_file_name( &Self::gen_name("dash", &[], &format), overwrite );
                },
                _ => panic!("ERROR: Unknown type requested in FaceN::generate_file_names()!"),
            };
        }
    }    

    pub fn read_imgs(&mut self, folder_name: &str) {    // read in the image files
        // read in the preview image
        self.preview_img_data.read_img(folder_name);
        
        // read in the digit image files
        for d in self.digits.iter_mut() {
            for i in 0..10 {
                d.img_data[i].read_img(folder_name);
            }
        }
        // read in the element image files
        for el in self.elements.iter_mut() {
            match el {
                Element::Image(e) => {          // one image, can be multiple Images
                    e.img_data.read_img(folder_name);
                },
                Element::TimeNum(_) => {},      // no images
                Element::DayName(e) => {        // seven images
                    for id in e.img_data.iter_mut() {                        
                        id.read_img(folder_name);
                    }
                },
                Element::BatteryFill(e) => {    // three images
                    e.img_data.read_img(folder_name);
                    e.image_data1.read_img(folder_name);
                    e.image_data2.read_img(folder_name);
                }, 
                Element::HeartRateNum(_) => {}, // no images
                Element::StepsNum(_) => {},     // no images
                Element::KCalNum(_) => {},      // no images
                Element::TimeHand(e) => {       // one image, h_type in filename                    
                    e.img_data.read_img(folder_name);
                }, 
                Element::DayNum(_) => {},       // no images
                Element::MonthNum(_) => {},     // no images
                Element::BarDisplay(e) => {     // variable images
                    for id in e.img_data.iter_mut() {                        
                        id.read_img(folder_name);
                    }
                }, 
                Element::Weather(e) => {        // variable images
                    for id in e.img_data.iter_mut() {                        
                        id.read_img(folder_name);
                    }
                }, 
                Element::Unknown29(_) => {},    // no images
                Element::Dash(e) => {      // one image
                    e.img_data.read_img(folder_name);
                },
                _ => panic!("ERROR: Unknown type found in FaceN::read_imgs()!"),
            };
        }
    }

    pub fn write_imgs(&self, folder_name: &str, format: &DumpFormat) {
        // write the preview image
        self.preview_img_data.write_img(folder_name, format);

        // write the digits
        for d in self.digits.iter() {
            for i in 0..10 {
                d.img_data[i].write_img(folder_name, format);
            }
        }        
        // write the elements
        for el in self.elements.iter() {
            match el {
                Element::Image(e) => {          // one image, can be multiple Images
                    e.img_data.write_img(folder_name, format);
                },
                Element::TimeNum(_) => {},      // no images
                Element::DayName(e) => {        // seven images
                    for id in e.img_data.iter() {                        
                        id.write_img(folder_name, format);
                    }
                },
                Element::BatteryFill(e) => {    // three images
                    e.img_data.write_img(folder_name, format);
                    e.image_data1.write_img(folder_name, format);
                    e.image_data2.write_img(folder_name, format);
                }, 
                Element::HeartRateNum(_) => {}, // no images
                Element::StepsNum(_) => {},     // no images
                Element::KCalNum(_) => {},      // no images
                Element::TimeHand(e) => {       // one image, h_type in filename                    
                    e.img_data.write_img(folder_name, format);
                }, 
                Element::DayNum(_) => {},       // no images
                Element::MonthNum(_) => {},     // no images
                Element::BarDisplay(e) => {     // variable images
                    for id in e.img_data.iter() {                        
                        id.write_img(folder_name, format);
                    }
                }, 
                Element::Weather(e) => {        // variable images
                    for id in e.img_data.iter() {                        
                        id.write_img(folder_name, format);
                    }
                }, 
                Element::Unknown29(_) => {},    // no images
                Element::Dash(e) => {      // one image
                    e.img_data.write_img(folder_name, format);
                },
                _ => panic!("ERROR: Unknown type found in FaceN::write_imgs()!"),
            };
        }
    }    
}

