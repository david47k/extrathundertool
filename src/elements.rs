//  elements.rs - the different elements displayed on the watch face
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


use serde::{Serialize, Deserialize};
use crate::img_data::ImgData;
use crate::util::{*};
use crate::xy::{*};


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "e_type")]
pub enum Element 
{
    Image(Image),
    TimeNum(TimeNum),
    DayName(DayName),
    TimeHand(TimeHand),
    DayNum(DayNum),
    BatteryFill(BatteryFill),
    HeartRateNum(HeartRateNum),
    StepsNum(StepsNum),
    KCalNum(KCalNum),
    MonthNum(MonthNum),
    BarDisplay(BarDisplay),
    Weather(Weather),
    Unknown29(Unknown29),
    Dash(Dash),
    Unknown,
}

impl Element 
{
    pub fn from_bin(file_data: &[u8], base_offset: usize) -> Element {
        let _one = file_data[base_offset];
        let e_type = file_data[base_offset + 1];
        let offset = base_offset + 2;
        let e: Element = match e_type {
            0 => Element::Image(Image::from_bin(file_data, offset)),
            2 => Element::TimeNum(TimeNum::from_bin(file_data, offset)),
            4 => Element::DayName(DayName::from_bin(file_data, offset)),
            5 => Element::BatteryFill(BatteryFill::from_bin(file_data, offset)),
            6 => Element::HeartRateNum(HeartRateNum::from_bin(file_data, offset)),
            7 => Element::StepsNum(StepsNum::from_bin(file_data, offset)),
            9 => Element::KCalNum(KCalNum::from_bin(file_data, offset)),
            10 => Element::TimeHand(TimeHand::from_bin(file_data, offset)),
            13 => Element::DayNum(DayNum::from_bin(file_data, offset)),
            15 => Element::MonthNum(MonthNum::from_bin(file_data, offset)),
            18 => Element::BarDisplay(BarDisplay::from_bin(file_data, offset)),
            27 => Element::Weather(Weather::from_bin(file_data, offset)),
            29 => Element::Unknown29(Unknown29::from_bin(file_data, offset)),
            35 => Element::Dash(Dash::from_bin(file_data, offset)),
            _ => Element::Unknown,
        };
        return e;
    }
    pub fn bin_size(&self) -> usize {
        // returns the size of the binary file counterpart struct. includes the 'one' and 'e_type' bytes.
        return match self {
            Element::Image(_) => 14,
            Element::TimeNum(_) => 34,
            Element::DayName(_) => 63,
            Element::BatteryFill(_) => 42,
            Element::HeartRateNum(_) => 26,
            Element::StepsNum(_) => 26,
            Element::KCalNum(_) => 19,
            Element::TimeHand(_) => 19,
            Element::DayNum(_) => 12,
            Element::MonthNum(_) => 12,
            Element::BarDisplay(e) => e.bin_size(),
            Element::Weather(e) => e.bin_size(),
            Element::Unknown29(_) => 3,
            Element::Dash(_) => 10,
            _ => panic!("ERROR: Unknown type requested in Element::bin_size()!"),
        };        
    }
    pub fn e_type(&self) -> u8 {
        return match self {
            Element::Image(_) => 0,
            Element::TimeNum(_) => 2,
            Element::DayName(_) => 4,
            Element::BatteryFill(_) => 5,
            Element::HeartRateNum(_) => 6,
            Element::StepsNum(_) => 7,
            Element::KCalNum(_) => 9,
            Element::TimeHand(_) => 10,
            Element::DayNum(_) => 13,
            Element::MonthNum(_) => 15,
            Element::BarDisplay(_) => 18,
            Element::Weather(_) => 27,
            Element::Unknown29(_) => 29,
            Element::Dash(_) => 35,
            _ => panic!("ERROR: Unknown type requested in Element::e_type()!"),
        };        
    }
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec::<u8> {
        // return the binary form of this element
        let mut h = Vec::<u8>::from([ 1, self.e_type() ]);
        h.extend( match self {
            Element::Image(el) =>       el.to_bin(blob_data, blob_offset),
            Element::TimeNum(el) =>     el.to_bin(),
            Element::DayName(el) =>     el.to_bin(blob_data, blob_offset),
            Element::BatteryFill(el) => el.to_bin(blob_data, blob_offset),
            Element::HeartRateNum(el) => el.to_bin(),
            Element::StepsNum(el) =>    el.to_bin(),
            Element::KCalNum(el) =>     el.to_bin(),
            Element::TimeHand(el) =>    el.to_bin(blob_data, blob_offset),
            Element::DayNum(el) =>      el.to_bin(),
            Element::MonthNum(el) =>    el.to_bin(),
            Element::BarDisplay(el) =>  el.to_bin(blob_data, blob_offset),
            Element::Weather(el) =>     el.to_bin(blob_data, blob_offset),
            Element::Unknown29(el) =>   el.to_bin(),
            Element::Dash(el) =>        el.to_bin(blob_data, blob_offset),
            _ => panic!("ERROR: Unknown type requested in Element::e_type()!"),
        });
        h
    }
}

// SPECIFIC ELEMENTS START HERE

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Image 
{                  // e_type 0
    pub x: u16,                         // 0 for background
    pub y: u16,                         // 0 for background
    pub img_data: ImgData,
}

impl Image 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Image {
        Image {
            x: get_u16(file_data, offset),
            y: get_u16(file_data, offset+2),
           img_data: ImgData::from_owh(file_data, offset+4),
        }
    }
    // takes mutable references to blob_data and blob_offset for storing the image data
    // returns a Vec<u8> full of the header data
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        h.extend(blob_offset.to_le_bytes());
        h.extend(self.img_data.w.to_le_bytes());
        h.extend(self.img_data.h.to_le_bytes());
        let bd = self.img_data.to_bin();
        *blob_offset += bd.len() as u32;
        blob_data.extend(bd);
        h
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TimeNum 
{                    // e_type 2
    pub digit_sets: [u8; 4],                // 0 or 1 for example, which digit font set to use for each digit
    pub xys: [XY; 4],                       // x and y position of the four time digits HHMM
    pub unknown: [u8; 12],
}

impl TimeNum 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+32)];
        Self {
            digit_sets: [file_data[offset], file_data[offset+1], file_data[offset+2], file_data[offset+3]],
            xys: [ XY::from_bin(file_data, offset + 4),
                   XY::from_bin(file_data, offset + 4 + 1 * XY::bin_size()),
                   XY::from_bin(file_data, offset + 4 + 2 * XY::bin_size()),
                   XY::from_bin(file_data, offset + 4 + 3 * XY::bin_size()) ],
            unknown: clone_into_array(&r[20..32]),
        }
    }
    // returns a Vec<u8> full of the header data
    pub fn to_bin(&self) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.extend(self.digit_sets);
        for xy in self.xys.iter() {
            h.extend(xy.to_bin());
        }
        h.extend(self.unknown);
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DayName 
{                    // e_type 4
    pub n_type: u8,
    pub x: u16,                             // x and y location
    pub y: u16,
    pub img_data: Vec::<ImgData>,       // size 7
}

impl DayName 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+61)];
        let mut e = Self {
            n_type: r[0],
            x: get_u16(r, 1),
            y: get_u16(r, 3),
            img_data: Vec::new(),
        };
        let mut owh_offset = offset + 5;
        for _ in 0..7 {
            e.img_data.push(ImgData::from_owh(file_data, owh_offset));
            owh_offset += 8;
        }
        e
    }
    // takes mutable references to blob_data and blob_offset for storing the image data
    // returns a Vec<u8> full of the header data
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.n_type);
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        for id in self.img_data.iter() {      // size 7
            h.extend(blob_offset.to_le_bytes());
            h.extend(id.w.to_le_bytes());
            h.extend(id.h.to_le_bytes());
            let bd = id.to_bin();
            *blob_offset += bd.len() as u32;
            blob_data.extend(bd);
        }
        h
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BatteryFill 
{   
    pub x: u16,
    pub y: u16,
    pub img_data: ImgData,      // battery charge background image
    pub x1: u8,                     // subsection for watch to fill, coords from image top left
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
    pub unknown0: u32,
    pub unknown1: u32,
    pub image_data1: ImgData,     // possibly for empty?
    pub image_data2: ImgData,     // possibly for full?
}

impl BatteryFill 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+40)];
        Self {
            x: get_u16(r, 0),
            y: get_u16(r, 2),
            img_data: ImgData::from_owh(file_data, offset + 4),
            x1: r[12],
            y1: r[13],
            x2: r[14],
            y2: r[15],
            unknown0: get_u32(r, 16),
            unknown1: get_u32(r, 20),
            image_data1: ImgData::from_owh(file_data, offset + 24),
            image_data2: ImgData::from_owh(file_data, offset + 32),
        }
    }
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        h.extend(blob_offset.to_le_bytes());
        h.extend(self.img_data.w.to_le_bytes());
        h.extend(self.img_data.h.to_le_bytes());
        let bd = self.img_data.to_bin();
        *blob_offset += bd.len() as u32;
        blob_data.extend(bd);
        h.extend([ self.x1, self.y1, self.x2, self.y2 ]);
        h.extend(self.unknown0.to_le_bytes());
        h.extend(self.unknown1.to_le_bytes());
        
        h.extend(blob_offset.to_le_bytes());
        h.extend(self.image_data1.w.to_le_bytes());
        h.extend(self.image_data1.h.to_le_bytes());
        let bd = self.image_data1.to_bin();
        *blob_offset += bd.len() as u32;
        blob_data.extend(bd);

        h.extend(blob_offset.to_le_bytes());
        h.extend(self.image_data2.w.to_le_bytes());
        h.extend(self.image_data2.h.to_le_bytes());
        let bd = self.image_data2.to_bin();
        *blob_offset += bd.len() as u32;
        blob_data.extend(bd);

        h
    }   
}


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct HeartRateNum 
{ 
    pub digit_set: u8,
    pub align: u8,
    pub x: u16,
    pub y: u16,
    pub unknown: [u8; 18],
}

impl HeartRateNum 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+24)];
        Self {
            digit_set: r[0],
            align: r[1],                                // 0:L, 1:R, 2:C
            x: get_u16(r,2),
            y: get_u16(r,4),
            unknown: clone_into_array(&r[6..24]),       // 0
        }
    }
    // returns a Vec<u8> full of the header data
    pub fn to_bin(&self) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.digit_set);
        h.push(self.align);
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        h.extend(self.unknown);
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct StepsNum 
{
    pub digit_set: u8,
    pub align: u8,
    pub x: u16,
    pub y: u16,
    pub unknown: [u8; 18],
}

impl StepsNum 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+24)];
        Self {
            digit_set: r[0],
            align: r[1],
            x: get_u16(r, 2),
            y: get_u16(r, 4),
            unknown: clone_into_array(&r[6..24]),
        }
    }
    // returns a Vec<u8> full of the header data
    pub fn to_bin(&self) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.digit_set);
        h.push(self.align);
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        h.extend(self.unknown);
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct KCalNum 
{
    pub digit_set: u8,
    pub align: u8,
    pub x: u16,
    pub y: u16,
    pub unknown: [u8; 11],
}

impl KCalNum 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+17)];
        Self {
            digit_set: r[0],
            align: r[1],
            x: get_u16(r, 2),
            y: get_u16(r, 4),
            unknown: clone_into_array(&r[6..17]),
        }
    }
    // returns a Vec<u8> full of the header data
    pub fn to_bin(&self) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.digit_set);
        h.push(self.align);
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        h.extend(self.unknown);
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct TimeHand 
{       // e_type 10
    pub h_type: u8,             // 0 = hour, 1 = minutes, 2 = seconds
    pub unknown_x: u16,
    pub unknown_y: u16,
    pub img_data: ImgData,
    pub x: u16,
    pub y: u16,
}

impl TimeHand 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+17)];
        Self {
            h_type: r[0],
            unknown_x: get_u16(r, 1),
            unknown_y: get_u16(r, 3),
            img_data: ImgData::from_owh(file_data, offset+5),
            x: get_u16(r, 13),
            y: get_u16(r, 15),
        }
    }
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.h_type);
        h.extend(self.unknown_x.to_le_bytes());
        h.extend(self.unknown_y.to_le_bytes());
        h.extend(blob_offset.to_le_bytes());
        h.extend(self.img_data.w.to_le_bytes());
        h.extend(self.img_data.h.to_le_bytes());
        let bd = self.img_data.to_bin();
        *blob_offset += bd.len() as u32;
        blob_data.extend(bd);
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct DayNum 
{         // e_type 13
    pub digit_set: u8,          // number of the digit set to use
    pub align: u8,              // alignment. 0:L, 1:R, 2:C
    pub xys: [XY; 2],           // XY of each digit in the day number
}

impl DayNum 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+10)];
        Self {
            digit_set: r[0],
            align: r[1],
            xys: [ XY::from_bin(file_data, offset + 2),
                   XY::from_bin(file_data, offset + 2 + XY::bin_size())],
        }
    }
    // returns a Vec<u8> full of the header data
    pub fn to_bin(&self) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.digit_set);
        h.push(self.align);
        for xy in self.xys.iter() {
            h.extend(xy.to_bin());
        }
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MonthNum 
{           // e_type 15
    pub digit_set: u8,
    pub align: u8,
    pub xys: [XY; 2],
}

impl MonthNum 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..(offset+10)];
        Self {
            digit_set: r[0],
            align: r[1],
            xys: [ XY::from_bin(file_data, offset + 2),
                   XY::from_bin(file_data, offset + 2 + XY::bin_size())],
        }
    }
    pub fn to_bin(&self) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.digit_set);
        h.push(self.align);
        for xy in self.xys.iter() {
            h.extend(xy.to_bin());
        }
        h
    }       
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BarDisplay 
{                 // e_type: 18
    pub b_type: u8,                     // Data source: 5=HeartRate, 6=Battery, 2=KCal, 0=Steps
    pub count: u8,                      // number of images in the bar display
    pub x: u16,
    pub y: u16,
    pub img_data: Vec<ImgData>,     // there are 'count' images
}

impl BarDisplay 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..];
        let b_type = r[0];
        let count = r[1];
        let c = count as usize;
        let x = get_u16(r, 2);
        let y = get_u16(r, 4);
        let mut img_data = Vec::<ImgData>::new();
        for i in 0..c {
            img_data.push(ImgData::from_owh(file_data, offset + 6 + i * 8));
        }
        Self {
            b_type,
            count,
            x,
            y,
            img_data,
        }
    }
    pub fn bin_size(&self) -> usize {
        return 8 + (self.count as usize) * 8;
    }
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.b_type);
        h.push(self.count);
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        for id in self.img_data.iter() {      // size is 'count'
            h.extend(blob_offset.to_le_bytes());
            h.extend(id.w.to_le_bytes());
            h.extend(id.h.to_le_bytes());
            let bd = id.to_bin();
            *blob_offset += bd.len() as u32;
            blob_data.extend(bd);
        }
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Weather 
{                // e_type: 27
    pub count: u8,
    pub x: u16,
    pub y: u16,
    pub img_data: Vec<ImgData>,     // there are 'count' items
}

impl Weather 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        let r = &file_data[offset..];
        let c = r[0] as usize;
        let mut img_data = Vec::<ImgData>::new();
        for i in 0..c {
            img_data.push(ImgData::from_owh(file_data, offset + 5 + i * 8));
        }
        Self {
            count: r[0],
            x: get_u16(r, 1),
            y: get_u16(r, 3),
            img_data,
        }
    }
    pub fn bin_size(&self) -> usize {
        return 7 + (self.count as usize) * 8;
    }
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.push(self.count);
        h.extend(self.x.to_le_bytes());
        h.extend(self.y.to_le_bytes());
        for id in self.img_data.iter() {      // size 'count'
            h.extend(blob_offset.to_le_bytes());
            h.extend(id.w.to_le_bytes());
            h.extend(id.h.to_le_bytes());
            let bd = id.to_bin();
            *blob_offset += bd.len() as u32;
            blob_data.extend(bd);
        }
        h
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Unknown29 
{              // e_type: 29
    pub unknown: u8,
}

impl Unknown29 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        Self {
            unknown: file_data[offset],
        }
    }
    pub fn to_bin(&self) -> Vec<u8> {
        Vec::from( [self.unknown] )
    }    
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Dash 
{               // e_type: 35
    pub img_data: ImgData,
}

impl Dash 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> Self {
        Self {
            img_data: ImgData::from_owh(file_data, offset),
        }
    }
    pub fn to_bin(&self, blob_data: &mut Vec<u8>, blob_offset: &mut u32) -> Vec<u8> {
        let mut h = Vec::<u8>::new();
        h.extend(blob_offset.to_le_bytes());
        h.extend(self.img_data.w.to_le_bytes());
        h.extend(self.img_data.h.to_le_bytes());
        let bd = self.img_data.to_bin();
        *blob_offset += bd.len() as u32;
        blob_data.extend(bd);
        h
    }    
}
