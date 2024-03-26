//  xy.rs - basic xy struct
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


use serde::{Serialize, Deserialize};
use crate::util::{*};


#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct XY 
{
    pub x: u16,
    pub y: u16,
}

impl XY 
{
    pub fn from_bin(file_data: &[u8], offset: usize) -> XY {
        XY {
            x: get_u16(file_data, offset),
            y: get_u16(file_data, offset + 2),
        }
    }
    pub fn bin_size() -> usize {
        4
    }
    pub fn to_bin(&self) -> [u8; 4] {
        let [a, b] = self.x.to_le_bytes();
        let [c, d] = self.y.to_le_bytes();
        [a, b, c, d]
    }
}
