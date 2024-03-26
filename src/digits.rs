//  digits.rs - the digits that are used on the watchface
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


// most of the digits handling is done directly from FaceN


use serde::{Serialize, Deserialize};
use crate::img_data::ImgData;
use crate::util::{*};


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Digits 
{
    pub img_data: Vec<ImgData>,       // size 10
    pub unknown: u16,
}

impl Digits 
{
    pub fn from_bin(file_data: &[u8], offset: usize, expected_set: usize) -> Digits {
        let set: u8 = file_data[offset];
        assert_eq!(set as usize, expected_set);         // ensure that the digits array index matches the specified digit set number
        let mut img_data: Vec<ImgData> = Vec::new();
        for i in 0..10 {            
            img_data.push( ImgData::from_owh(file_data, offset + 1 + 8 * i) );
        }
        let unknown: u16 = get_u16(file_data, offset + 81);
        Digits {
            img_data,
            unknown,
        }
    }
}
