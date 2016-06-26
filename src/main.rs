extern crate elma;
extern crate clap;

use elma::lev::{ObjectType, Level};
use elma::OBJECT_RADIUS;
use clap::{Arg, App};

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Settings<'a> {
    ground: &'a str,
    sky: &'a str,
    apple: &'a str,
    flower: &'a str,
    killer: &'a str,
    player: &'a str,
    stroke: usize,
    scale: usize,
    pad: usize,
    width: usize,
    height: usize,
    max_width: usize,
    max_height: usize
}

fn main () {
    // Take care of command line arguments.
    let matches = App::new("elma-lev2map")
                            .version(VERSION)
                            .author("Roger Andersen <hexjelly@hexjelly.com>")
                            .about("Converts Elasto Mania level files to images")
                            .arg(Arg::with_name("input")
                                .short("i").long("input")
                                .value_name("PATH")
                                .help("Path to level file")
                                .takes_value(true).use_delimiter(false).required(true))
                            .arg(Arg::with_name("output")
                                .short("o").long("output")
                                .value_name("PATH")
                                .help("Path to save image file [default: <input>.svg]")
                                .use_delimiter(false).takes_value(true))
                            .arg(Arg::with_name("svg")
                                .long("svg")
                                .help("Specify SVG as output type [default]"))
                            .arg(Arg::with_name("ground")
                                .short("g").long("ground")
                                .value_name("COLOR")
                                .help("Ground fill color, in rgb, hex or name")
                                .use_delimiter(false).takes_value(true)
                                .default_value("#181048"))
                            .arg(Arg::with_name("sky")
                                .short("s")
                                .long("sky")
                                .value_name("COLOR")
                                .help("Sky fill color, in rgb, hex or name")
                                .use_delimiter(false)
                                .default_value("#3078bc")
                                .takes_value(true))
                            .arg(Arg::with_name("pad")
                                .long("pad")
                                .value_name("SIZE")
                                .help("Canvas padding")
                                .default_value("10")
                                .use_delimiter(false)
                                .takes_value(true))
                            .arg(Arg::with_name("scale")
                                .long("scale")
                                .value_name("SIZE")
                                .help("Scale of SVG")
                                .default_value("20")
                                .use_delimiter(false)
                                .takes_value(true))
                            .arg(Arg::with_name("apple")
                                .short("a")
                                .long("apple")
                                .value_name("COLOR")
                                .help("Apple color, in rgb, hex or name")
                                .use_delimiter(false)
                                .default_value("red")
                                .takes_value(true))
                            .arg(Arg::with_name("flower")
                                .short("f")
                                .long("flower")
                                .value_name("COLOR")
                                .help("Flower color, in rgb, hex or name")
                                .use_delimiter(false)
                                .default_value("white")
                                .takes_value(true))
                            .arg(Arg::with_name("killer")
                                .short("k")
                                .long("killer")
                                .value_name("COLOR")
                                .help("Killer color, in rgb, hex or name")
                                .use_delimiter(false)
                                .default_value("black")
                                .takes_value(true))
                            .arg(Arg::with_name("player")
                                .short("p")
                                .long("player")
                                .value_name("COLOR")
                                .help("Player color, in rgb, hex or name")
                                .use_delimiter(false)
                                .default_value("green")
                                .takes_value(true))
                            .arg(Arg::with_name("stroke")
                                .long("stroke")
                                .value_name("SIZE")
                                .help("Line stroke around objects")
                                .use_delimiter(false)
                                .default_value("0")
                                .takes_value(true))
                            .arg(Arg::with_name("height")
                                .long("height")
                                .value_name("SIZE")
                                .help("Height")
                                .conflicts_with("max_height")
                                .use_delimiter(false).takes_value(true))
                            .arg(Arg::with_name("width")
                                .long("width")
                                .value_name("SIZE")
                                .help("Width")
                                .conflicts_with("max_width")
                                .use_delimiter(false).takes_value(true))
                            .arg(Arg::with_name("max_height")
                                .long("maxheight")
                                .value_name("SIZE")
                                .help("Max height")
                                .use_delimiter(false).takes_value(true))
                            .arg(Arg::with_name("max_width")
                                .long("maxwidth")
                                .value_name("SIZE")
                                .help("Max width")
                                .use_delimiter(false).takes_value(true))
                            .get_matches();

    let input_file = Path::new(matches.value_of("input").unwrap());
    let mut output_file;
    if let Some(path) = matches.value_of("output") {
        output_file = PathBuf::from(path);
        if output_file.is_dir() {
            if let Some(file_name) = input_file.file_name() {
                output_file.push(file_name);
                output_file.set_extension("svg");
            }
        }
    } else {
        output_file = PathBuf::from(input_file);
        output_file.set_extension("svg");
    }

    let ground = matches.value_of("ground").unwrap();
    let sky = matches.value_of("sky").unwrap();
    let apple = matches.value_of("apple").unwrap();
    let flower = matches.value_of("flower").unwrap();
    let killer = matches.value_of("killer").unwrap();
    let player = matches.value_of("player").unwrap();
    let stroke = matches.value_of("stroke").unwrap().parse::<usize>().unwrap();
    let scale = matches.value_of("scale").unwrap().parse::<usize>().unwrap();
    let pad = matches.value_of("pad").unwrap().parse::<usize>().unwrap();
    let mut width = 0;
    if let Some(val) = matches.value_of("width") {
        width = val.parse::<usize>().unwrap();
    };
    let mut max_width = 0;
    if let Some(val) = matches.value_of("max_width") {
        max_width = val.parse::<usize>().unwrap();
    };
    let mut height = 0;
    if let Some(val) = matches.value_of("height") {
        height = val.parse::<usize>().unwrap();
    };
    let mut max_height = 0;
    if let Some(val) = matches.value_of("max_height") {
        max_height = val.parse::<usize>().unwrap();
    };

    let settings = Settings {
        ground: ground,
        sky: sky,
        apple: apple,
        flower: flower,
        killer: killer,
        player: player,
        stroke: stroke,
        scale: scale,
        pad: pad,
        width: width,
        height: height,
        max_width: max_width,
        max_height: max_height
    };

    make_svg(input_file, settings, &output_file);
}

fn make_svg (input: &Path, settings: Settings, output: &PathBuf) {

    let level = Level::load(input.to_str().unwrap()).unwrap();
    let mut _killer = false;
    let mut _apple = false;

    // Figure out max and min coordinates.
    let mut max_x = 0_f64;
    let mut max_y = 0_f64;
    let mut min_x = 0_f64;
    let mut min_y = 0_f64;

    for polygon in &level.polygons {
        if !polygon.grass {
            for vertex in &polygon.vertices {
                if vertex.x < min_x { min_x = vertex.x; }
                if vertex.x > max_x { max_x = vertex.x; }
                if vertex.y < min_y { min_y = vertex.y; }
                if vertex.y > max_y { max_y = vertex.y; }
            }
        }
    }

    // Checking coordinates for objects as well, as they could be outside of polygons.
    //
    // In addition to that, keep track of what type of objects the level has, in order to add the necessary ones
    // to the <defs> tag, and skip the ones not in the level to make the file size smaller, if only by a tiny fraction.
    for object in &level.objects {
        if object.position.x < min_x { min_x = object.position.x - OBJECT_RADIUS; }
        if object.position.x > max_x { max_x = object.position.x + OBJECT_RADIUS; }
        if object.position.y < min_y { min_y = object.position.y - OBJECT_RADIUS; }
        if object.position.y > max_y { max_y = object.position.y + OBJECT_RADIUS; }

        match object.object_type {
            ObjectType::Apple { .. } => _apple = true,
            ObjectType::Killer => _killer = true,
            _ => break
        };
    }

    // Start writing SVG data to buffer.
    let mut buffer = vec![];
    let width = ((max_x + min_x.abs()) * settings.scale as f64) + settings.pad as f64 * 2_f64;
    let height = ((max_y + min_y.abs()) * settings.scale as f64) + settings.pad as f64 * 2_f64;

    buffer.extend_from_slice(br#"<?xml version="1.0" standalone="no"?><!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">"#);
    buffer.extend_from_slice(format!("\r\n<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" width=\"{}\" height=\"{}\">\r\n<defs>",
                             width, height).as_bytes());
    // Killer definition.
    buffer.extend_from_slice(br##"
        <g id="killer">
        <style type="text/css">
        	.st0 { fill: #231F20; }
        	.st1 { fill: #941A1D; }
        	.st2 { fill: #D91B21; }
        </style>
        	<polygon class="st0" points="20.1,6.6 23.5,7.9 28.8,2.8 24.5,3.7" />
        	<polygon class="st0" points="28.3,13 33.9,17.1 34.8,18.7 31.6,17.1 28.8,17.1" />
        	<polygon class="st0" points="26.7,24 26.6,28.9 26.6,32.6 25.6,28.6 24.1,26.3" />
        	<polygon class="st0" points="15.1,27.5 11.6,29.5 8.2,29.9 11.1,28.6 13.1,26.6" />
        	<polygon class="st0" points="7.4,19.5 7.4,16.8 3.9,16.8 0,15.3 3.4,18.3" />
        	<polygon class="st0" points="9.6,10.4 9.2,5.6 11.7,1.3 11.7,5.3 13.1,7.9" />

        	<circle class="st1" cx="18.1" cy="17.1" r="11" />

        	<polygon class="st0" points="26.1,13.8 31.8,22.9 32.1,26.4 30,23.2 26.6,21.2 23.8,24.1 20.8,31.8 18.1,35.2 19.4,31.6 19.1,25.5
        		7.3,25.6 2.9,24.5 8.3,23.8 10.5,21.7 7.6,15.1 4.6,11.4 3.6,7.3 7.1,10.9 9.5,12.1 10.9,12.1 13.1,8.8 15.4,3.9 19.5,0 18.9,2.8
        		18.1,7.1 18.1,8.6 29.8,8.7 35.1,9.8 26.6,11.6" />

        	<circle class="st2" cx="18.1" cy="17.1" r="5" />
        </g>"##);

    // Apple definition.
    buffer.extend_from_slice(br##"
        <g id="apple">
        <style type="text/css">
        	.st1{fill:#CF0000;}
        	.st2{fill:#00AA00;}
        	.st3{fill:#3F1D00;}
        </style>

        	<path class="st1" d="M38.4,16.6c0.1-1.4-0.2-3.8-0.8-5.5c-0.6-1.4-2.8-3.8-4.1-4.6c-1.3-0.9-4.3-1.8-5.8-2.1
        		c-2.1-0.4-5.6-0.9-7.7-1c-1.8-0.1-6.2,0-7.9,0.2c-1.6,0.2-4.9,0.8-6.3,1.7C4.7,5.9,2.8,7.7,2.2,8.7c-0.7,1.1-1.8,3.5-2,4.8
        		c-0.4,2.8-0.2,4.3,0,5.7c0.2,1.7,0.5,4.4,1.4,6.7c1,2.8,2.4,4.8,3.6,6.1c1.3,1.4,4.3,3.7,6,4.5c1.8,0.8,5.7,1.8,7.7,1.8
        		c2.2,0,6.8-0.7,8.8-1.7c1.5-0.8,4.1-3.1,5.2-4.3C36.9,27.8,38.2,20.8,38.4,16.6z"/>

        	<path class="st2" d="M30.6,1.8c-0.4-0.3-2-1.3-3.8-0.8c-0.2,0.1-0.7,0.2-1.3,0.5c-0.9,0.6-1,1.3-2,2.8c-0.6,0.9-0.9,1.4-1.4,1.8
        		c-1,0.8-2.6,1.3-3.2,0.8c-0.7-0.7,0.2-2.8,1.1-4c0.5-0.6,1.4-1.7,2.9-2.4C26.7-1.1,30.4,1.6,30.6,1.8z"/>

        	<path class="st3" d="M18.4,7c-0.7-0.7-2.1-3.4-2.1-3.4c-0.1-0.2-0.1-0.5,0.1-0.6l0.5-0.3C17.3,2.5,17.8,2.6,18,3
        		c0,0,1.5,2.6,1.5,3.5c0,0-0.3,0.3-0.4,0.4C19,7,18.4,7,18.4,7z"/>
        </g>"##);


    buffer.extend_from_slice(format!("</defs>\r\n\t<rect width=\"100%\" height=\"100%\" style=\"fill: {};\" />\r\n",
                             settings.ground).as_bytes());

    // Polygons.
    buffer.extend_from_slice(format!("\t<path fill-rule=\"evenodd\" fill=\"{}\" d=\"",
                             settings.sky).as_bytes());
    for polygon in &level.polygons {
        if !polygon.grass {
            for (n, vertex) in polygon.vertices.iter().enumerate() {
                if n == 0 { buffer.extend_from_slice(b"M"); }
                else { buffer.extend_from_slice(b"L"); }
                let x = ((vertex.x + min_x.abs()) * settings.scale as f64) + settings.pad as f64;
                let y = ((vertex.y + min_y.abs()) * settings.scale as f64) + settings.pad as f64;
                buffer.extend_from_slice(format!("{} {} ", x, y).as_bytes());
            }
        }
    }
    buffer.extend_from_slice(b"Z\" />\r\n");

    // Objects
    for object in &level.objects {
        let x = ((object.position.x + min_x.abs()) * settings.scale as f64) + settings.pad as f64;
        let y = ((object.position.y + min_y.abs()) * settings.scale as f64) + settings.pad as f64;

        match object.object_type {
            ObjectType::Apple { .. } => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#apple\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
            ObjectType::Killer => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#killer\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
            _ => {}
        };

        // TODO: maybe make a setting to just use simple shapes?
        let color = match object.object_type {
            ObjectType::Apple { .. } => settings.apple,
            ObjectType::Exit => settings.flower,
            ObjectType::Killer => settings.killer,
            ObjectType::Player => settings.player
        };

        buffer.extend_from_slice(format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"{}\" />\r\n",
                                 x, y, OBJECT_RADIUS * settings.scale as f64, "black", settings.stroke, color).as_bytes());
    }

    buffer.extend_from_slice(b"</svg>");

    // Write buffer to file.
    let mut file = File::create(&output).unwrap();
    file.write_all(&buffer).unwrap();
    println!("Wrote SVG file: {:?}", output);
}
