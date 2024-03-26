//  util.rs - basic utility functions
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


// BASIC SLICE TO ARRAY CONVERSION

pub fn clone_into_array<A, T>(slice: &[T]) -> A
where
    A: Default + AsMut<[T]>,
    T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

// BASIC BINARY FUNCTIONS

pub fn get_u16(blob: &[u8], idx: usize) -> u16 
{
    let a: u16 = blob[idx].into();
    let b: u16 = blob[idx+1].into();
    (b << 8) | a
}

pub fn get_u32(blob: &[u8], idx: usize) -> u32 
{
    let a: u32 = blob[idx].into();
    let b: u32 = blob[idx+1].into();
    let c: u32 = blob[idx+2].into();
    let d: u32 = blob[idx+3].into();
    (d << 24) | (c << 16) | (b << 8) | a
}

pub fn put_u16(blob: &mut[u8], idx: usize, val: u16) 
{
    blob[idx+0] = (val & 0xFF) as u8;
    blob[idx+1] = ((val & 0xFF00) >> 8) as u8;
}

pub fn put_u32(blob: &mut[u8], idx: usize, val: u32) 
{
    blob[idx+0] = (val & 0xFF) as u8;
    blob[idx+1] = ((val & 0xFF00) >> 8) as u8;
    blob[idx+2] = ((val & 0xFF0000) >> 16) as u8;
    blob[idx+3] = ((val & 0xFF000000) >> 24) as u8;
}

// ALIGNMENT AND PADDING FUNCTIONS

pub fn get_align_diff(offset: u32) -> u32 {
    match offset % 4 {
        0 => 0 ,
        1 => 3,
        2 => 2,
        3 => 1,
        _ => panic!("mod function failure"),
    }
}

// return FF bytes to pad to maintain 32-bit alignment
pub fn pad_it(v: &mut Vec::<u8>) {    
    v.extend( match v.len() % 4 {
        0 => vec![0xff; 0],
        1 => vec![0xff; 3],
        2 => vec![0xff; 2],
        3 => vec![0xff; 1],
        _ => panic!("mod function failure"),
    } );
}

pub fn align_it(o: &mut u32) {
    *o += match *o % 4 {
        0 => 0,
        1 => 3,
        2 => 2,
        3 => 1,
        _ => panic!("mod function failure"),
    };
}