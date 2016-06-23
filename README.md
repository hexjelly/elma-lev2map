# elma-lev2map

Converts Elasto Mania level files to images.

## Documentation

```text
USAGE:
    elma-lev2map.exe [FLAGS] [OPTIONS] --input <PATH>

FLAGS:
    -h, --help       Prints help information
        --svg        Specify SVG as output type [default]
    -V, --version    Prints version information

OPTIONS:
    -a, --apple <COLOR>         Apple color: rgb(a), hex or named color [default: red]
    -f, --flower <COLOR>        Flower color: rgb(a), hex or named color [default: white]
    -g, --ground <COLOR>        Ground color: rgb(a), hex or named color [default: #181048]
    -i, --input <PATH>          Path to level file
    -k, --killer <COLOR>        Killer color: rgb(a), hex or named color [default: black]
    -o, --output <PATH>         Path to save image file [default: <input>.svg]
        --pad <UNITS>           Canvas padding [default: 10]
    -p, --player <COLOR>        Player color: rgb(a), hex or named color [default: green]
        --scale <UNITS>         Scale of SVG [default: 20]
    -s, --sky <COLOR>           Sky color: rgb(a), hex or named color [default: #3078bc]
        --stroke <THICKNESS>    Line stroke around objects [default: 0]
```

## Features

-   [x] Basic SVG with customizable scale, padding and colors
-   [ ] Textures, rasterized objects
-   [ ] PNG

## Usage

For a default SVG file:

```text
elma-lev2map -i "Untitled.lev"
```

Showing how you can input arguments in
several various valid ways):

```text
elma-lev2map -i "Untitled.lev" -a "blue" --flower="#efefef" -gpink -s=cyan -k"rgb(200,15,99)"
```
