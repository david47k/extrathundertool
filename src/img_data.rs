//  img_data.rs: binary rle-compressed image data
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


// ImgData represents the BINARY COMPRESSED WATCHFACE form of the image
// It also stores the file name, for import and export of the image


use serde::{Serialize, Deserialize};
use crate::util::{*};
use crate::sane_file_name::{*};
use std::path::PathBuf;
use std::fs;
use std::fmt;
use crate::img::{*};


// IMAGE DUMP FORMAT

#[derive(PartialEq)]
pub enum DumpFormat {
        BIN = 0,
        RAW = 1,
        BMP = 2,
}

// IMAGEDATA STARTS HERE

#[derive(Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ImgData {
    // we get an OffsetWidthHeight from the file, read it in as some u8 bytes
    #[serde(skip)]          // don't save (or read) the data field to json
    pub header: Vec<u8>,
    #[serde(skip)]          // don't save (or read) the data field to json
    pub data: Vec<u8>,
    pub w: u16,
    pub h: u16,
    pub file_name: Option<String>,
}

impl ImgData {
    // get the size of the rle compressed image data
    fn get_data_size(file_data: &[u8], base_offset: usize, height: usize) -> usize {
        let header_size = (height as usize) * 4;
        let last_header_entry = base_offset + header_size - 4;
        let mut last_offset = get_u16(&file_data, last_header_entry) as usize;       // this offset is from start of image data
        let mut last_size   = get_u16(&file_data, last_header_entry + 2) as usize;
        last_offset        += (last_size & 0x1F) << 16;        // The lowest 5 bits are the hi part of the offset.
        last_size           = last_size / 32;
        let image_size      = last_offset + last_size;
        // we will return the size of the image blob, excluding the headers (which aren't important to us)
        return image_size - header_size;
    }

    // read in binary data from a file. pass in the offset of the OWH structure. the function will load the image data from the file data.
    pub fn from_owh(file_data: &[u8], owh_offset: usize) -> ImgData {
        // read in an owh structure
        let bin_offset =  get_u32(&file_data, owh_offset) as usize;      // offset is from start of file
        let width =       get_u16(&file_data, owh_offset+4) as usize;
        let height =      get_u16(&file_data, owh_offset+6) as usize;
        
        ImgData::from_bin(file_data, bin_offset, width, height)
    }

    // load binary image data.
    pub fn from_bin(bin_data: &[u8], bin_offset: usize, width: usize, height: usize) -> ImgData {        
        let header_size = height as usize * 4;
        let blob_size = ImgData::get_data_size(&bin_data, bin_offset, height);    // determine the size of the compressed image data blob
        let header: Vec<u8> = bin_data[bin_offset..(bin_offset+header_size)].into();
        let data: Vec<u8> = bin_data[(bin_offset + header_size)..(bin_offset + header_size + blob_size)].into();
        ImgData {
            header,
            data,
            w: width as u16,
            h: height as u16,
            file_name: None,
        }
    }

    pub fn to_bin(&self) -> Vec<u8> {
        // this returns the header and image data
        // it does not include the offset/width/height header, which cannot be calculated from inside
        let mut bin_data = self.header.clone();
        bin_data.extend(self.data.iter());
        // we like it aligned...
        pad_it(&mut bin_data);
        bin_data
    }

    pub fn read_img(&mut self, folder_name: &str) {
        // check we have a filename
        if self.file_name.is_none() || !sane_file_name(&self.file_name.as_ref().unwrap()) {
            println!("WARNING: Not reading image file, as file_name is non-existant or non-sensible.");
            return;
        }
        let file_name = self.file_name.as_ref().unwrap();

        // check file format
        let mut format = DumpFormat::BMP;                
        if file_name.len() > 4 {
            let extn = file_name[file_name.len()-4..file_name.len()].to_lowercase();
            format = match extn.as_str() {
                ".bin" => DumpFormat::BIN,
                ".raw" => DumpFormat::RAW,
                ".bmp" => DumpFormat::BMP,
                _ => { println!("WARNING: Unrecognised file extension '{}', assuming BMP.", extn); DumpFormat::BMP }
            };
        } else {
            println!("WARNING: File name '{}' is very short, it should be a BMP file.", file_name);
        }
        
        // read in the file
        let path: PathBuf = [folder_name, &file_name].iter().collect();
        let file_data: Vec<u8> = match fs::read(path) {
            Ok(fd) => fd,
            Err(e) => panic!("Unable to read file '{}': {}", &file_name, e),
        };

        if format == DumpFormat::BMP || format == DumpFormat::RAW {
            // read it into an Img
            let mut img: Img = match format {
                DumpFormat::BMP => match Img::from_bmp(&file_data) {
                                    Ok(i) => i,
                                    Err(e) => panic!("Unable to understand BMP file '{}': {}", &file_name, e),
                },
                DumpFormat::RAW => Img {
                                    w: self.w as u32,
                                    h: self.h as u32,
                                    format: ImgFormat::Argb8565,
                                    data: file_data,
                                    rle_header: None,
                },
                _ => panic!("unexpected ImgFormat"),
            };

            // convert it to bin format
            img.convert_format(ImgFormat::RleNew);

            // save it to self
            self.w = img.w as u16;
            self.h = img.h as u16;
            self.data = img.data;
            self.header = img.rle_header.unwrap();
        } else if format == DumpFormat::BIN {
            // read it in
            *self = ImgData::from_bin(&file_data, 0, self.w as usize, self.h as usize);
        } else {
            panic!("Unknown ImgFormat!");
        }
    }

    pub fn write_img(&self, folder_name: &str, format: &DumpFormat) {        
        if self.file_name.is_none() {
            println!("ERROR: No file name for ImgData::write_img()!");
            return;
        }        
        let file_name = self.file_name.as_ref().unwrap();
        let path: PathBuf = [folder_name, &file_name].iter().collect();
        let mut img = Img::from_img_data(&self);
        let b: Vec<u8> = match *format {
            DumpFormat::BMP => img.to_bmp(),
            DumpFormat::RAW => {
                img.convert_format(ImgFormat::Argb8565);        // decompress the image data
                img.data
            },
            DumpFormat::BIN => {
                let mut tmp = img.rle_header.unwrap();
                tmp.extend(img.data.iter());
                tmp
            }
        };

        match fs::write(path, b) {
            Ok(_) => {},
            Err(e) => { println!("ERROR: Unable to save '{}': {}", &file_name, e); return; },
        };
    }

    pub fn set_file_name(&mut self, file_name: &str, overwrite: bool) {
        // set the file name, only if it is not already set, or it is dodgy, or we are to overwrite
        if overwrite || self.file_name.is_none() || !sane_file_name(self.file_name.as_ref().unwrap()) {
            self.file_name = Some(file_name.to_string());
        }
    }
}

impl fmt::Debug for ImgData 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ImgData")
         .field("data (size)", &self.data.len())
         .field("w", &self.w)
         .field("h", &self.h)
         .finish()
    }
}