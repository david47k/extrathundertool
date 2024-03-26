//  main.rs - program entry
//
// 	ExtraThunder WatchFace Tool
// 	for Mo Young / Da Fit binary watch face files.
//
// 	Copyright 2022-4 David Atkinson
// 	Author: David Atkinson <dav!id47k@d47.co> (remove the '!')
// 	License: GNU General Public License version 2 or any later version (GPL-2.0-or-later)


use std::fs;
use std::path::PathBuf;

mod util;
mod bmp_format;
mod binary_face_n;
mod img;
mod digits;
mod face;
mod xy;
mod img_data;
mod elements;
mod sane_file_name;

use crate::img_data::DumpFormat;


fn main() {
    let mut file_name = "";
    let mut folder_name = "dump";
    let mut format = DumpFormat::BMP;
    let mut dump = false;
    let mut pack = false;
    let mut show_help = false;
    let mut debug_level: u8 = 1;

    // find executable name
    let basename = "extrathundertool";
    let argv: Vec<String> = std::env::args().collect();

    // read command-line parameters
    for i in 1..argv.len() {
        if argv[i] == "--raw" {
            format = DumpFormat::RAW;
        } else if argv[i] == "--bin" {
            format = DumpFormat::BIN;
        } else if argv[i].starts_with("--dump") {
            dump = true;
            if argv[i].len() >= 8 && argv[i].as_bytes()[6] == b'=' {
                folder_name = &argv[i][7..];
            }
        } else if argv[i].starts_with("--pack") {
            pack = true;
            if dump && pack {
                panic!("ERROR: Can't dump and pack at the same time!");
            }
            if argv[i].len() >= 8 && argv[i].as_bytes()[6] == b'=' {
                folder_name = &argv[i][7..];
            }
        } else if argv[i].starts_with("--debug") {
            debug_level = 3;
            if argv[i].len() >= 9 && argv[i].as_bytes()[7] == b'=' {
                debug_level = argv[i][8..].parse().unwrap();
            }
        } else if argv[i].starts_with("--help") {
            show_help = true;
        } else if argv[i].starts_with("--") {
            eprintln!("ERROR: Unknown option: {}", argv[i]);
            show_help = true;
        } else {
            // must be file_name
            if file_name == "" {
                file_name = &argv[i];
            } else {
                eprintln!("WARNING: Ignored unknown parameter: {}", argv[i]);
            }
        }
    }

    // display basic program header
    eprintln!("\nextrathunder watchface tool: For 'new' Mo Young / Da Fit binary watch face files.\n");

    // display help
    if argv.len() < 2 || show_help {
        eprintln!("Usage:   {} [OPTIONS] FILENAME\n", basename);
        eprintln!("  OPTIONS");
        eprintln!("    --dump=FOLDERNAME    Dump data to folder. Folder name defaults to 'dump'.");
        eprintln!("    --bin                When dumping, dump binary (compressed) files.");
        eprintln!("    --raw                When dumping, dump raw (decompressed raw bitmap) files.");
        eprintln!("    --debug=LEVEL        Print more debug info. Range 0 to 3.");
        eprintln!("  FILENAME               Binary watch face file for input.");
        eprintln!("\n");
        std::process::exit(0);
    }

    if !pack {
        print!("Reading '{}' ... ", file_name);

        // Open the binary input file
        let fdata: Vec<u8> = fs::read(file_name).expect("ERROR: Failed to read file into memory.");

        // Load the binary watch face file
        let mut f = face::FaceN::from_bin(&fdata);

        println!("done.");

        if debug_level >= 3 {
            // Print debug info
            eprintln!("api_ver          {}", f.api_ver);
            eprintln!("unknown          0x{:04X}", f.unknown);
            eprintln!("digits.len       {}", f.digits.len());
            eprintln!("elements.len     {}", f.elements.len());

            for i in 0..f.digits.len() {
                eprintln!("\nDIGIT SET {}\n", i);
                eprintln!("{:#?}", f.digits[i]);
            }
            for i in 0..f.elements.len() {
                eprintln!("\nELEMENT {}: E_TYPE {}\n", i, f.elements[i].e_type());
                eprintln!("{:#?}", f.elements[i]);
            }
        }

        if dump {
            // create folder if it doesn't exist
            let path = PathBuf::from(folder_name);
            if !path.is_dir() {
                match std::fs::create_dir_all(path.clone()) {
                    Ok(_) => {},
                    Err(e) => { println!("ERROR: Unable to create folder '{}': {}", folder_name, e); return; },
                };
            }
            
            // generate image filenames
            f.generate_file_names(&format);
            
            // save the images
            print!("Saving images ... ");
            f.write_imgs(&folder_name, &format);
            println!("done.");

            // file to save json data to        
            let output_file_name = "watchface.json";
            let path: PathBuf = [ folder_name, output_file_name ].iter().collect();

            let json_data = match serde_json::to_string_pretty(&f) {
                Ok(s) => s,
                Err(e) => { println!("ERROR: Unable to serialize watchface: {}", e); return; },
            };

            // save the json data
            print!("Saving '{}' ... ", output_file_name);
            match fs::write(path, json_data) {
                Ok(_) => {},
                Err(e) => { println!("ERROR: Unable to save '{}': {}", output_file_name, e); return; },
            };
            println!("done.");
        }
    } else {    // PACK
        // read in the json file
        let path = PathBuf::from(folder_name);
        if !path.is_dir() {
            panic!("ERROR: path provided for --pack is not a folder")
        }

        print!("Reading 'watchface.json' ... ");
        let path: PathBuf = [ folder_name, "watchface.json" ].iter().collect();
        let file_data = match std::fs::read(path) {
            Ok(fd) => fd,
            Err(e) => { println!("ERROR: Unable to read file: {}", e); return; }
        };

        let mut face: face::FaceN = match serde_json::from_slice(&file_data) {
            Ok(fd) => fd,
            Err(e) => { println!("ERROR: Unable to understand JSON file: {}", e); return; }
        };

        println!("done.");

        // Read in the images
        println!("Reading in bitmaps ...");
        face.read_imgs(folder_name);
        println!("... done.");

        // Print debug info
        // println!("{:#?}", face);

        // Get binary data
        let bin_data = face.to_bin();

        // Write to output file
        println!("Saving '{}' ... ", file_name);
        match fs::write(file_name, bin_data) {
            Ok(_) => {},
            Err(e) => { println!("ERROR: Unable to save '{}': {}", file_name, e); return; },
        };
        println!("... done.");

    }
}
