
//  sane_file_name.rs - check if a file name is sane/sensible
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


// returns TRUE if the file name is sane, returns FALSE if it is dodgy
pub fn sane_file_name(fname: &str) -> bool {
    // fname must have at least one character
    if fname.len() < 1 {
        return false;
    }            
    // must not contain dodgy chars like wildcards or path characters
    let dodgy_chars = "/\\|?*<>:\"";
    if fname.chars().any(|c| dodgy_chars.contains(c)) {
        return false;
    }
    // must not contain chars 0-31
    if fname.chars().any(|c| (c as u32) < 32) {
        return false;
    }
    // must not end in space or dot
    let chs: Vec<char> = fname.chars().collect();
    let c = chs[chs.len()-1];
    if c == ' ' || c == '.' {
        return false;
    }
    // must not be any of the weird windows strings
    let weird_strings = [ "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9" ];
    if weird_strings.iter().any(|s| fname.to_uppercase() == *s) {
        return false;
    }
    return true;
}
