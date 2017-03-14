extern crate elma;
#[macro_use]
extern crate clap;

use elma::lev::{ObjectType, Level};
use elma::OBJECT_RADIUS;
use clap::App;

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct Settings<'a> {
    ground: &'a str,
    sky: &'a str,
    apple: &'a str,
    flower: &'a str,
    killer: &'a str,
    player: &'a str,
    stroke: u32,
    scale: u32,
    pad: u32,
    complexity: &'a str
}

fn main () {
    // Take care of command line arguments.
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

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
    let stroke = matches.value_of("stroke").unwrap().parse::<u32>().unwrap();
    let scale = matches.value_of("scale").unwrap().parse::<u32>().unwrap();
    let pad = matches.value_of("pad").unwrap().parse::<u32>().unwrap();
    let complexity = matches.value_of("svg").unwrap();

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
        if settings.complexity == "svg-mix" || settings.complexity == "svg-lo" {
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
        if settings.complexity == "svg-mix" || settings.complexity == "svg-hi" {
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
