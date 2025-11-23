# ReChrome

ReChrome is a CLI that recolors images based on predefined palettes.

## Currently available palettes:
- [everforest](https://github.com/sainnhe/everforest/blob/master/palette.md9)
- gray
- [gruvbox](https://github.com/morhetz/gruvbox)
- [kanagawa](https://github.com/rebelot/kanagawa.nvim)
- [molokai](https://github.com/UtkarshVerma/molokai.nvim)
- [papercut](https://github.com/NLKNguyen/papercolor-theme)
- [solarized](https://github.com/solarized/xresources)

## Planned Features
Dithering:
- Bayes
- Floyd-Steinberg

## Usage
```
Usage: rechrome.exe [OPTIONS] --palette <PALETTE> --input <INPUT>

Options:
  -p, --palette <PALETTE>  Available: 
                              > everforest    > gray
                              > gruvbox       > kanagawa
                              > molokai       > papercut
                              > solarized 
  -i, --input <INPUT>      Input  file path
  -o, --output <OUTPUT>    Output file path (optional)
                           
  -s, --showcase <SIZE>    Show preview (optional, recommended < 50)
  -r, --runtime            Show timing measurements
                           
  -h, --help               Print help
  -V, --version            Print version
```
