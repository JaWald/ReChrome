# ReChrome
ReChrome is a command-line tool that recolors images using predefined color palettes.
It applies Bayer ordered dithering to reduce banding, then maps each pixel to the nearest color in the selected palette and saves the result.

It loads an image from the specified input path, applies dithering, performs palette quantization, and writes the processed output file in the desired format.
## Palettes
|[Original](https://unsplash.com/de/fotos/ein-schwarzer-hintergrund-mit-einem-rosa-und-blauen-design-L8fXJgMk5jc)|[Everforest](https://github.com/sainnhe/everforest)|[Gruvbox](https://github.com/morhetz/gruvbox)|[Kanagawa](https://github.com/rebelot/kanagawa.nvim)|
|:--:|                                 :---:                                     |                    :---:                    |                      :---:                         |
|<img src="images/color.jpg" width="300" alt="Original">|<img src="images/color_everforest_bayer16-64.jpg" width="300" alt="Everforest">|<img src="images/color_gruvbox_bayer16-64.jpg" width="300" alt="Gruvbox">|<img src="images/color_kanagawa_bayer16-64.jpg" width="300" alt="Kanagawa">|
||[Molokai](https://github.com/UtkarshVerma/molokai.nvim)|[Papercut](https://github.com/NLKNguyen/papercolor-theme)|[Solarized](https://github.com/solarized/xresources)|
||<img src="images/color_molokai_bayer16-64.jpg" width="300" alt="Molokai">|<img src="images/color_papercut_bayer16-64.jpg" width="300" alt="Papercut">|<img src="images/color_solarized_bayer16-64.jpg" width="300" alt="Solarized">|

## Help
```c++
Usage: rechrome.exe [OPTIONS] --input <INPUT>

Options:
  -p, --palette <PALETTE>  Color Palette    [possible values: atomone, catppuccin, darcula, everforest, gruvbox, kanagawa, monokai, nord, papercut, solarized, synthwave]
  -d, --dither <DITHER>    Dithering Modes  [default: bayer8] [possible values: raw, bayer2, bayer4, bayer8, bayer16]
  -a, --ampl <AMPL>        Bayer Amplitude  [default: bayerX * 4, <256] 
                           
  -i, --input <INPUT>      Input file path  
  -f, --format <FORMAT>    Output file type [default: jpg] [possible values: png, jpg, jpeg]
  -o, --output <OUTPUT>    Output file path [default: <input>_<palette>_<dither>.<format>]
                           
  -s, --showcase <SIZE>    Show in-terminal (recommended value: <= 20)
  -r, --runtime            Show runtime performance
                           
  -t, --test <TEST>        Test arguments   [possible values: none, palette, dither, amplitude, all]
  -h, --help               Print help
  -V, --version            Print version
```
### Example usage
Simple conversion\
` <correctDir>.\rechrome.exe -i "C:\Users\<user>\Downloads\flower.jpeg" -p kanagawa`

Conversion with preview\
` <correctDir>.\rechrome.exe -i "C:\Users\<user>\Downloads\flower.jpeg" -p kanagawa -s 15`

Manual dithering (watercolor effect)\
` <correctDir>.\rechrome.exe -i "C:\Users\<user>\Downloads\flower.jpeg" -p kanagawa -d bayer2 -b 128`

In all these examples, ReChrome saves the image to the input directory, appending palette and dithering parameter to the file name.

## Install
Download the most recent release under the "Releases" Section on the right hand side, then unzip it and use it in your terminal

Unfortunately, the binary might be flagged by Windows Defender as a Virus\
To circumvent this, please follow these steps:
- Create a folder in a root directive
- Exclude said folder from Windows Defender
- Unzip ReChrome to that folder
