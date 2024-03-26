//  binary_face_n.rs: structure of the binary watchface file format.
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


// this file is more for reference, we don't use it directly


#![allow(unused_variables)]
#![allow(dead_code)]

use crate::xy::XY;                    // size 4

#[repr(C, packed)]
pub struct OffsetWidthHeight {          // size 8
    pub offset: u32,
    pub width: u16,
    pub height: u16,
}

// #[repr(C, packed)]
// pub struct XY {
//     pub x: u16,
//     pub y: u16,
// }

#[repr(C, packed)]
#[derive(Copy,Clone)]
pub struct FaceHeaderN {                // size 16
    pub api_ver: u16,
    pub unknown: u16,
    pub preview_offset: u32,            // possibly
    pub preview_width: u16,
    pub preview_height: u16,
    pub dh_offset: u16,
    pub bh_offset: u16,
}

#[repr(C, packed)]
pub struct DigitsHeader {               // size 83
    pub digit_set: u8,
    pub owh: [OffsetWidthHeight; 10],
    pub unknown: u16,
}

#[repr(C, packed)]
pub struct ImageHeader {                // e_type 0, size 14
    pub one: u8,
    pub e_type: u8,
    pub x: u16,
    pub y: u16,
    pub offset: u32,
    pub width: u16,
    pub height: u16,
}

#[repr(C, packed)]
pub struct TimeNumHeader {              // e_type 2, size 34
    pub one: u8,
    pub e_type: u8,
    pub digit_set: [u8; 4],
    pub xy: [XY; 4],
    pub padding: [u8; 12],
}

#[repr(C, packed)]
pub struct DayNameHeader {              // e_type 4, size 63
    pub one: u8,
    pub e_type: u8,
    pub n_type: u8,
    pub x: u16,
    pub y: u16,
    pub owh: [OffsetWidthHeight; 7],
}

#[repr(C, packed)]
pub struct BatteryFillHeader {          // e_type 5, size 42
    pub one: u8,
    pub e_type: u8,
    pub x: u16,
    pub y: u16,
    pub owh: OffsetWidthHeight,
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
    pub unknown: u32,
    pub unknown2: u32,
    pub owh1: OffsetWidthHeight,
    pub owh2: OffsetWidthHeight,
}

#[repr(C, packed)]
pub struct HeartRateNumHeader {         // e_type 6, size 26
    pub one: u8,
    pub e_type: u8,
    pub digit_set: u8,
    pub align: u8,
    pub x: u16,
    pub y: u16,
    pub unknown: [u8; 18],
}

#[repr(C, packed)]
pub struct StepsNumHeader {
    pub one: u8,
    pub e_type: u8,
    pub digit_set: u8,
    pub align: u8,
    pub x: u16,
    pub y: u16,
    pub unknown: [u8; 18],
}

#[repr(C, packed)]
pub struct KCalNumHeader {
    pub one: u8,
    pub e_type: u8,
    pub digit_set: u8,
    pub align: u8,
    pub x: u16,
    pub y: u16,
    pub unknown: [u8; 11],
}

#[repr(C, packed)]
pub struct TimeHandHeader {
    pub one: u8,
    pub e_type: u8,
    pub subid: u8,
    pub unknown_xy: XY,
    pub offset: u32,
    pub width: u16,
    pub height: u16,
    pub x: u16,
    pub y: u16,
}

#[repr(C, packed)]
pub struct DayNumHeader {
    pub one: u8,
    pub e_type: u8,
    pub digit_set: u8,
    pub align: u8,
    pub xy: [XY; 2],
}

#[repr(C, packed)]
pub struct MonthNumHeader {
    pub one: u8,
    pub e_type: u8,
    pub digit_set: u8,
    pub align: u8,
    pub xy: [XY; 2],
}

#[repr(C, packed)]
pub struct BarDisplayHeader {
    pub one: u8,
    pub e_type: u8,
    pub subid: u8,
    pub count: u8,
    pub x: u16,
    pub y: u16,
    pub owh: Vec<OffsetWidthHeight>,
}

#[repr(C, packed)]
pub struct WeatherHeader {
    pub one: u8,
    pub e_type: u8,
    pub subid: u8,
    pub x: u16,
    pub y: u16,
    pub owh: [OffsetWidthHeight; 9],
}

#[repr(C, packed)]
pub struct Unknown1D01Header {
    pub one: u8,
    pub e_type: u8,
    pub unknown: u8,
}

#[repr(C, packed)]
pub struct DashHeader {
    pub one: u8,
    pub e_type: u8,
    pub offset: u32,
    pub width: u16,
    pub height: u16,
}
