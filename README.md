# extrathundertool

ExtraThunder watch face tool for 'new' Mo Young / Da Fit binary watch face files.

Allows you to dump and pack the files.

This tool is in alpha stage, some of the code is a bit rough (or even untested)!

The tool for the older watch face files is [here](https://github.com/david47k/dawft).

## Usage

```
Usage:   extrathundertool [OPTIONS] FILENAME

  OPTIONS
    --dump=FOLDERNAME    Dump data to folder. Folder name defaults to 'dump'.
    --pack=FOLDERNAME    Pack data from folder. Folder name defaults to 'dump'.
    --bmp                When dumping, dump BMP (windows bitmap) files. Default.
    --raw                When dumping, dump raw (decompressed raw bitmap) files.
    --bin                When dumping, dump binary (RLE compressed) files.
    --debug=LEVEL        Print more debug info. Range 0 to 3.
  FILENAME               Binary watch face file for input/output.
  ```

## Supported watches

Da Fit watches using MoYoung v2 firmware and the 'new' watchface API should be supported to some extent.  

Tpls | Screen width x height (pixels) |  Example models | Example firmware | Comments 
-----|------------|--------------|----------------|---------
  70 | 	240 x 296 |  GTS 3       | MOY-VSW4-2.0.1       | 

