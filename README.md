# icons

Generates Apple Icon Image format `icns` and Microsoft ICO file format `ico` files from PNG files.

```shell
icons <source_image.png>
```


## Apple Icon Image format

The file format consists of an 8 byte header, followed by any number of icons.

### Header

```
| Magic number|  File size  |
-----------------------------
| 69 63 6e 73 | 00 00 00 00 |
-----------------------------
   i  c  n  s
```

* Magic number
  * must be `icns` (0x69, 0x63, 0x6e, 0x73)
* File size
  * length of file, in bytes


### Body

```
 |   OSType    |  data size  | icon data         |   OSType    |
 ------------------------------------------------------------------
 | 69 63 31 32 | 00 00 00 00 | xx xx xx xx ...   | 69 63 30 37 | ...
 ------------------------------------------------------------------
   i  c  1  2                                      i  c  0  7
```

* OSType:
  * IS32 : 16×16 24-bit icon
  * IL32 : 32x32 24-bit icon
  * IH32 : 48×48 24-bit icon
  * ICP6 : 64x64 32-bit PNG/JP2 icon
  * IC07 : 128x128 32-bit PNG/JP2 icon
  * IC08 : 256×256 32-bit PNG/JP2 icon
  * IC09 : 512×512 32-bit PNG/JP2 icon
  * IC11 : 16x16@2x "retina" 32-bit PNG/JP2 icon
  * IC12 : 32x32@2x "retina" 32-bit PNG/JP2 icon
  * IC13 : 128x128@2x "retina" 32-bit PNG/JP2 icon
  * IC14 : 256x256@2x "retina" 32-bit PNG/JP2 icon
  * IC10 : 512x512@2x "retina" 32-bit PNG/JP2 icon
* data size
  * including type and length, in bytes
* icon data
  * variable icon data


## ICO file format

The ICO file format is an image file format in Microsoft Windows.

little-endian byte order

Application icons and Control Panel items:

The full set includes 16x16, 32x32, 48x48, and 256x256 (code scales between 32 and 256).


### Header

```
| magic | type  | count |
-------------------------
| 00 00 | 01 00 | 04 00 |
-------------------------
```
* magic
  * Reserved. Must always be 0
* type
  * 1:ico
  * 2:cur
* count
  * number of images in the file


### ICONDIRENTRY

```
| w  | h  |pale|    | color |bit per| size of     | offset of   | w  
|    |    |tte |    | plane |pixel  | image data  | data        |
 -----------------------------------------------------------------------
| 00 | 00 | 01 | 00 | 00 00 | 00 00 | 00 00 00 00 | 00 00 00 00 | 00 ...
-----------------------------------------------------------------------
```
* w
  * Image width in pixels
  * Value 0 means image width is 256 pixels
* h
  * Image height in pixels
  * Value 0 means image width is 256 pixels
* palette
  * Number of colors in the color palette
  * Should be 0 if the image does not use a color palette
* color plane
  * color planes
  * Should be 0 or 1 in ico format
* bit per pixel
* size of image data
  * The Size of the image's data in bytes
* offset of data
  * The offset of BMP or PNG data from the beginning of the ICO/CUR file

```
| data of icon 1 | data of icon 2 |
--------------------------------------
| 00 00 ...      | 00 00 ...      |...
--------------------------------------
```


