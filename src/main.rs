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
    max_height: usize,
    complexity: &'a str
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
                            .arg(Arg::with_name("complexity")
                                .long("complexity")
                                .value_name("CHOICE")
                                .help("What complexity to use for objects")
                                .use_delimiter(false).takes_value(true)
                                .default_value("complex")
                                .possible_values(&["simple", "complex", "mix"]))
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
    let complexity = matches.value_of("complexity").unwrap();

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
        max_height: max_height,
        complexity: complexity
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
        if object.position.x - OBJECT_RADIUS < min_x { min_x = object.position.x - OBJECT_RADIUS; }
        if object.position.x > max_x { max_x = object.position.x + OBJECT_RADIUS; }
        if object.position.y - OBJECT_RADIUS < min_y { min_y = object.position.y - OBJECT_RADIUS; }
        if object.position.y > max_y { max_y = object.position.y + OBJECT_RADIUS; }

        match object.object_type {
            ObjectType::Apple { .. } => _apple = true,
            ObjectType::Killer => _killer = true,
            _ => {}
        };
    }

    // Start writing SVG data to buffer.
    let mut buffer = vec![];
    let width = ((max_x + min_x.abs()) * settings.scale as f64) + settings.pad as f64 * 2_f64;
    let height = ((max_y + min_y.abs()) * settings.scale as f64) + settings.pad as f64 * 2_f64;

    buffer.extend_from_slice(br#"<?xml version="1.0" standalone="no"?><!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">"#);
    buffer.extend_from_slice(format!("\r\n<svg xmlns=\"http://www.w3.org/2000/svg\" xmlns:xlink=\"http://www.w3.org/1999/xlink\" viewBox=\"0 0 {} {}\">\r\n<defs>",
                             width, height).as_bytes());

    // ground texture
    buffer.extend_from_slice(br##"
        <pattern id="img1" patternUnits="userSpaceOnUse" x="0" y="0" width="100" height="100" viewBox="0 0 40 31">
        <style type="text/css">
        	#shapes{fill:#191341;stroke:#000000;stroke-miterlimit:10;}
        	#highlights{fill:#210CF5;}
        </style>
        <g id="shapes">
        	<polyline points="17.8,32.9 24.5,27 28,27 28.4,33.4"/>
        	<polygon points="25.3,19.9 21.3,11.2 19.3,12.9 17.7,11.8 14.9,17.2 16.2,23.5 20.1,24"/>
        	<polyline points="8.3,31.9 8,22.9 10.1,20 14.9,17.2 16.2,23.5 20.1,24 25.3,19.9 28.6,23.4 28,25.8 28,27 24.3,27 18.7,32.1"/>
        	<polyline points="19.4,-2.8 13,3.9 5.8,-1"/>
        	<polyline points="-4.4,18.1 10.1,20 8,22.9 8.3,31.3 8.4,33.3 -3.5,32.9"/>
        	<polyline points="-2.4,18.1 10.1,20 14.9,17.2 17.7,11.8 14.5,9.5 13,3.9 5.8,-1 -2.8,-1"/>
        	<polyline points="41.8,25 32.2,22.4 28.6,23.4 25.3,19.9 21.3,11.2 26.8,6.6 34.4,6.9 41.5,2.9"/>
        	<polygon points="41.5,2.9 34.4,6.9 26.8,6.8 28.4,-1 41.3,-1"/>
        	<polygon points="42.9,25.3 32.2,22.4 28.6,23.4 28,25.8 28.4,33.4 41.3,32.7"/>
        	<path d="M41.8,9.8l-4.5,0.3c-0.7,0-1.3,0.4-1.7,0.8l-1.4,1.6c-0.6,0.7-0.7,1.8,0,2.5l0.9,1.1 c0.4,0.5,1,0.8,1.7,1l5.1,0.9V9.8z"/>
        	<polyline points="28.4,-1 26.8,6.6 19.3,12.9 14.5,9.5 13,4 17.8,-1 28.3,-1"/>
        	<polyline points="-1.8,10 4.3,10 5.8,7.5 3.7,5.2 -2.3,4.7"/>
        </g>
        <g id="highlights">
        	<path d="M5.8,2l-2,0C3.6,2,3.5,1.8,3.5,1.6V1.6c0-0.2,0.1-0.3,0.3-0.3h2c0.2,0,0.3,0.1,0.3,0.3v0.1 C6.1,1.8,5.9,2,5.8,2z"/>
        	<path d="M3.8,12h-3c-0.1,0-0.2,0.1-0.2,0.2l0,0c0,0.1,0.1,0.5,0.2,0.5h2.9C4,12.6,4,12.2,4,12.1l0,0 C4,12,3.9,12,3.8,12z"/>
        	<path d="M14.2,12.4c0,0.2-0.3,0.4-0.6,0.4c-0.3,0-0.6-0.2-0.6-0.4c0-0.2,0.3-0.6,0.6-0.6 C13.9,11.8,14.2,12.2,14.2,12.4z"/>
        	<path d="M39.4,12.9h-2c-0.2,0-0.4-0.2-0.4-0.4v-0.1c0-0.2,0.2-0.4,0.4-0.4h2c0.2,0,0.4,0.2,0.4,0.4v0.1 C39.8,12.7,39.6,12.9,39.4,12.9z"/>
        </g>
        </pattern>
        "##);

    // Killer definition.
    buffer.extend_from_slice(br##"
        <g id="killer">
        <style type="text/css">
        	#spikes{fill:#110f0f;}
        	#st1{fill:#941A1D;}
        	#st2{fill:#D91B21;}
        </style>
        <circle id="st1" cx="8" cy="7.5" r="4.9"/>
        <g id="spikes">
            <polygon points="12.7,1.2 10.8,1.6 8.9,2.9 10.4,3.5"/>
            <polygon points="15.3,8.2 13.9,7.5 12.7,7.5 12.5,5.7 14.9,7.5"/>
            <polygon points="11.7,14.4 11.3,12.6 10.6,11.6 11.8,10.6 11.7,12.7"/>
            <polygon points="3.6,13.2 4.9,12.6 5.8,11.7 6.7,12.1 5.1,13"/>
            <polygon points="1.7,7.4 0,6.7 1.5,8.1 3.3,8.6 3.3,7.4"/>
            <polygon points="5.2,0.6 5.2,2.3 5.8,3.5 4.2,4.6 4.1,2.5"/>
            <polygon points="14.2,11.6 13.2,10.2 11.7,9.3 10.5,10.6 9.2,14 8,15.5 8.6,13.9 8.4,11.2 3.2,11.3
            1.3,10.8 3.7,10.5 4.6,9.6 3.4,6.7 2,5 1.6,3.2 3.1,4.8 4.2,5.3 4.8,5.3 5.8,3.9 6.8,1.7 8.6,0 8.3,1.2 8,3.1 8,3.8 13.1,3.8
            15.5,4.3 11.7,5.1 11.5,6.1 14,10.1"/>
        </g>
        <circle id="st2" cx="8" cy="7.5" r="2.2"/>
        </g>"##);

    // Apple definition.
    buffer.extend_from_slice(br##"
        <g id="apple">
        <style type="text/css">
        	#main{fill:#CF0000;}
        	#leaf{fill:#00AA00;}
        	#stem{fill:#3F1D00;}
        </style>

        <path id="main" d="M15.4,6.6c0-0.5-0.1-1.5-0.3-2.2c-0.2-0.6-1.1-1.5-1.7-1.9c-0.5-0.3-1.7-0.7-2.3-0.8C10.2,1.6,8.8,1.4,8,1.4
            c-0.7,0-2.5,0-3.2,0.1c-0.6,0.1-2,0.3-2.5,0.7c-0.4,0.3-1.2,1-1.4,1.4C0.6,3.9,0.2,4.9,0.1,5.4c-0.2,1.1-0.1,1.7,0,2.3
            c0.1,0.7,0.2,1.8,0.6,2.7c0.4,1.1,1,1.9,1.5,2.5c0.5,0.5,1.7,1.5,2.4,1.8c0.7,0.3,2.3,0.7,3.1,0.7c0.9,0,2.7-0.3,3.5-0.7
            c0.6-0.3,1.6-1.2,2.1-1.7C14.8,11.1,15.3,8.3,15.4,6.6z"/>

        <path id="leaf" d="M12.3,0.7c-0.2-0.1-0.8-0.5-1.5-0.3c-0.1,0-0.3,0.1-0.5,0.2C9.9,0.9,9.8,1.1,9.5,1.7C9.2,2.1,9.1,2.3,8.9,2.5
            C8.5,2.8,7.8,3,7.6,2.8C7.3,2.5,7.7,1.6,8,1.1c0.2-0.3,0.6-0.7,1.2-1C10.7-0.4,12.2,0.6,12.3,0.7z"/>

        <path id="stem" d="M7.4,2.8C7.1,2.5,6.5,1.4,6.5,1.4c0-0.1,0-0.2,0.1-0.2l0.2-0.1c0.2-0.1,0.4,0,0.4,0.1c0,0,0.6,1,0.6,1.4
            c0,0-0.1,0.1-0.2,0.2C7.6,2.8,7.4,2.8,7.4,2.8z"/>
        </g>"##);

    // Flower definition
    buffer.extend_from_slice(br##"
    <g id="flower">
    <style type="text/css">
    	#petals{fill:url(#gradientpetals);}
    	#middle{fill:url(#gradientmiddle);}
    </style>
    <linearGradient id="gradientpetals" gradientUnits="userSpaceOnUse" x1="14.8102" y1="11.872" x2="1.1024" y2="3.9578">
		<stop offset="0" style="stop-color:#FFFFFF"/>
		<stop  offset="1" style="stop-color:#E7E7E7"/>
	</linearGradient>
    <linearGradient id="gradientmiddle" gradientUnits="userSpaceOnUse" x1="10.4078" y1="9.4658" x2="5.5449" y2="6.6581">
        <stop offset="0" style="stop-color:#E9C300"/>
        <stop offset="1" style="stop-color:#FFF900"/>
    </linearGradient>
	<path id="petals" d="M9.4,3.3L10,3.5c0,0,0.4-1.1,0.7-1.4c0.2-0.3,0.7-0.7,1-0.8c0.3-0.1,1,0.1,1.2,0.3
		c0.3,0.2,0.5,0.9,0.5,1.2c0,0.4-0.6,1.1-0.8,1.4c-0.2,0.3-1.1,0.9-1.1,0.9l0.3,0.4c0,0,0.8-0.4,1.1-0.6c0.4-0.2,1.4-0.3,1.8-0.2
		c0.3,0.1,0.8,0.7,1,0.9c0.2,0.3,0.3,1.1,0.3,1.5c0,0.4-0.5,1.1-0.9,1.3c-0.3,0.2-1.1,0-1.5,0.1c-0.1,0-0.2,0.1-0.2,0.1s1,0.7,1.2,1
		c0.2,0.3,0.5,0.9,0.6,1.2c0,0.2-0.1,0.7-0.2,0.9c-0.1,0.2-0.6,0.5-0.8,0.6c-0.3,0.1-0.9,0-1.1-0.1c-0.3-0.1-0.8-0.8-0.8-0.8l-0.4,0
		c0,0,0.4,0.9,0.5,1.3c0.1,0.4,0.3,1.2,0.2,1.7c-0.1,0.3-0.4,0.7-0.6,0.9c-0.3,0.2-0.9,0.3-1.3,0.2c-0.3-0.1-0.8-0.5-1-0.8
		C9.2,14.5,9,13.4,9,13.4H8.7c0,0,0,1.5-0.1,1.9C8.5,15.6,8,16,7.7,16.1c-0.3,0.1-1.1-0.2-1.3-0.4c-0.3-0.2-0.4-1-0.4-1.4
		c0-0.4,0.2-1.5,0.2-1.5l-0.4-0.2c0,0-0.2,0.7-0.4,1c-0.2,0.3-0.8,0.8-1.2,0.9c-0.3,0-0.9-0.2-1.1-0.4c-0.2-0.2-0.4-0.6-0.4-0.8
		c0-0.3,0-1,0.3-1.3c0.3-0.4,1.5-1.2,1.5-1.2s0.1-0.3,0-0.4C4.3,10.3,4,10.2,4,10.2s-0.6,0.4-0.9,0.5c-0.3,0.1-0.8,0.4-1.1,0.4
		c-0.3,0-0.9-0.2-1.2-0.3c-0.2-0.2-0.5-0.6-0.6-0.9C0.2,9.6,0,9,0,8.6c0-0.2,0.2-0.9,0.4-1c0.2-0.2,0.7-0.5,1-0.5
		C1.6,7.1,2.3,7,2.3,7l0-0.5c0,0-0.6-0.4-0.8-0.5C1.2,5.8,1,5.4,0.9,5.1c-0.1-0.3,0-1,0.2-1.3C1.3,3.5,2,3.2,2,3.2s0.8,0,1.1,0.1
		c0.2,0.1,0.7,0.4,0.7,0.4l0.2-0.3c0,0-0.4-0.7-0.5-0.9C3.3,2.3,3.3,1.7,3.3,1.4c0.1-0.3,0.4-0.7,0.6-0.9C4.2,0.3,5,0.2,5.4,0.2
		C5.7,0.3,6.1,0.7,6.3,1c0.2,0.4,0.5,1.5,0.5,1.5l0.4,0c0,0-0.1-1.1,0-1.5c0.1-0.3,0.4-0.8,0.7-1c0.3-0.1,1-0.1,1.3,0.1
		c0.2,0.1,0.4,0.5,0.4,0.8C9.7,1.5,9.4,3.3,9.4,3.3z"/>
	<ellipse id="middle" cx="8" cy="8" rx="2.6" ry="3.3"/>
    </g>
    "##);

    buffer.extend_from_slice(b"</defs>\r\n\t<rect width=\"100%\" height=\"100%\" fill=\"url(#img1)\" />\r\n");

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

    // Grass polygons.
    buffer.extend_from_slice(b"\t<path fill=\"none\" stroke=\"green\" stroke-width=\"0.7\" d=\"");
    for polygon in &level.polygons {
        if polygon.grass {
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

    // Objects.
    for object in &level.objects {
        let x = ((object.position.x + min_x.abs()) * settings.scale as f64) + settings.pad as f64;
        let y = ((object.position.y + min_y.abs()) * settings.scale as f64) + settings.pad as f64;

        // Simple circles.
        if settings.complexity == "mix" || settings.complexity == "simple" {
            let color = match object.object_type {
                ObjectType::Apple { .. } => settings.apple,
                ObjectType::Exit => settings.flower,
                ObjectType::Killer => settings.killer,
                ObjectType::Player => settings.player
            };

            buffer.extend_from_slice(format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"{}\" />\r\n",
                                     x, y, OBJECT_RADIUS * settings.scale as f64, "black", settings.stroke, color).as_bytes());
        }

        // SVG objects
        match object.object_type {
            ObjectType::Apple { .. } => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#apple\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
            ObjectType::Killer => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#killer\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
            ObjectType::Exit => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#flower\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
            _ => {}
        };
    }

    buffer.extend_from_slice(b"</svg>");

    // Write buffer to file.
    let mut file = File::create(&output).unwrap();
    file.write_all(&buffer).unwrap();
    println!("Wrote SVG file: {:?}", output);
}
