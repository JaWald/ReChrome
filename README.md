# ReChrome
ReChrome is a command-line tool that recolors images using predefined color palettes.
It applies Bayer ordered dithering to reduce banding, then maps each pixel to the nearest color in the selected palette and saves the result.

It loads an image from the specified input path, applies dithering, performs palette quantization, and writes the processed output file in the desired format.
## Palettes
|[Original](https://pixabay.com/photos/goldfish-aquarium-fish-tank-pets-8012081/)|[Atom One](https://github.com/Th3Whit3Wolf/one-nvim/tree/main)|[Catppuccin](https://catppuccin.com/palette/)|[Darcula](https://github.com/doums/darcula)|
|:--:|                                 :---:                                     |                    :---:                    |                      :---:                         |
|<img src="images/showcase.jpg" width="300" alt="Original">|<img src="images/showcase_Pal_atomone.jpg" width="300" alt="Atom One">|<img src="images/showcase_Pal_catppuc.jpg" width="300" alt="Catppuccin">|<img src="images/showcase_Pal_darcula.jpg" width="300" alt="Darcula">|
|[Everforest](https://github.com/sainnhe/everforest/blob/master/palette.md)|[Gruvbox](https://github.com/morhetz/gruvbox)|[Kanagawa](https://github.com/rebelot/kanagawa.nvim)|[Monokai](https://github.com/UtkarshVerma/molokai.nvim)|
|<img src="images/showcase_Pal_everforst.jpg" width="300" alt="Everforest">|<img src="images/showcase_Pal_gruvbox.jpg" width="300" alt="Gruvbox">|<img src="images/showcase_Pal_kanagwa.jpg" width="300" alt="Kanagawa">|<img src="images/showcase_Pal_monokai.jpg" width="300" alt="Monokai">|
|[Nord](https://www.nordtheme.com/docs/colors-and-palettes)|[PaperColor](https://github.com/NLKNguyen/papercolor-theme)|[Solarized](https://github.com/solarized/xresources)|[Synthwave](https://github.com/robb0wen/synthwave-vscode)|
|<img src="images/showcase_Pal_nord___.jpg" width="300" alt="Nord">|<img src="images/showcase_Pal_paprcut.jpg" width="300" alt="Papercut">|<img src="images/showcase_Pal_solarizd.jpg" width="300" alt="Solarized">|<img src="images/showcase_Pal_synthwve.jpg" width="300" alt="Synthwave">|

## Help
```c++
Usage: rechrome.exe [OPTIONS] --input <INPUT>

Options:
  -p, --palette <PALETTE>  Color Palette    [possible values: atomone, catppuccin, darcula, everforest, gruvbox, kanagawa, monokai, nord, papercolor, solarized, synthwave]
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
