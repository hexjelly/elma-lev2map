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
    // TODO: put into external yaml file
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
    // TODO: use clap default thing doh
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

    // import objects, textures, pictures
    buffer.extend_from_slice(include_bytes!("assets/ground.def"));
    buffer.extend_from_slice(include_bytes!("assets/killer.def"));
    buffer.extend_from_slice(include_bytes!("assets/apple.def"));
    buffer.extend_from_slice(include_bytes!("assets/flower.def"));

    buffer.extend_from_slice(b"</defs>\r\n\t<rect width=\"100%\" height=\"100%\" fill=\"url(#ground)\" />\r\n");

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
        if settings.complexity == "mix" || settings.complexity == "complex" {
            match object.object_type {
                ObjectType::Apple { .. } => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#apple\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
                ObjectType::Killer => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#killer\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
                ObjectType::Exit => buffer.extend_from_slice(format!("<use x=\"{}\" y=\"{}\" xlink:href=\"#flower\" />\r\n", x - OBJECT_RADIUS * settings.scale as f64, y - OBJECT_RADIUS * settings.scale as f64).as_bytes()),
                _ => {}
            };
        }
    }

    buffer.extend_from_slice(b"</svg>");

    // Write buffer to file.
    // TODO: remove unwraps doh
    let mut file = File::create(&output).unwrap();
    file.write_all(&buffer).unwrap();
    println!("Wrote SVG file: {:?}", output);
}
