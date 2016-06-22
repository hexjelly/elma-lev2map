extern crate elma;
extern crate clap;

use elma::lev::{ObjectType, Level};
use elma::OBJECT_RADIUS;
use clap::{Arg, App};

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main () {
    // Take care of command line arguments.
    let matches = App::new("elma-lev2map")
                            .version(VERSION)
                            .author("Roger Andersen <hexjelly@hexjelly.com>")
                            .about("Converts Elasto Mania level files to images")
                            .arg(Arg::with_name("input")
                                .short("i")
                                .long("input")
                                .value_name("PATH")
                                .help("Path to level file")
                                .takes_value(true)
                                .required(true))
                            .arg(Arg::with_name("output")
                                .short("o")
                                .long("output")
                                .value_name("PATH")
                                .help("Path to save image file [default: <input>.svg]")
                                .takes_value(true))
                            .arg(Arg::with_name("svg")
                                .long("svg")
                                .help("Specify SVG as output type [default]"))
                            .arg(Arg::with_name("ground")
                                .short("g")
                                .long("ground")
                                .value_name("COLOR")
                                .help("Ground fill color, in rgb, hex or name")
                                .default_value("#181048")
                                .takes_value(true))
                            .arg(Arg::with_name("sky")
                                .short("s")
                                .long("sky")
                                .value_name("COLOR")
                                .help("Sky fill color, in rgb, hex or name")
                                .default_value("#3078bc")
                                .takes_value(true))
                            .arg(Arg::with_name("pad")
                                .long("pad")
                                .value_name("UNITS")
                                .help("Canvas padding")
                                .default_value("10")
                                .takes_value(true))
                            .arg(Arg::with_name("scale")
                                .long("scale")
                                .value_name("UNITS")
                                .help("Scale of SVG")
                                .default_value("20")
                                .takes_value(true))
                            .arg(Arg::with_name("apple")
                                .short("a")
                                .long("apple")
                                .value_name("COLOR")
                                .help("Apple color, in rgb, hex or name")
                                .default_value("red")
                                .takes_value(true))
                            .arg(Arg::with_name("flower")
                                .short("f")
                                .long("flower")
                                .value_name("COLOR")
                                .help("Flower color, in rgb, hex or name")
                                .default_value("white")
                                .takes_value(true))
                            .arg(Arg::with_name("killer")
                                .short("k")
                                .long("killer")
                                .value_name("COLOR")
                                .help("Killer color, in rgb, hex or name")
                                .default_value("black")
                                .takes_value(true))
                            .arg(Arg::with_name("player")
                                .short("p")
                                .long("player")
                                .value_name("COLOR")
                                .help("Player color, in rgb, hex or name")
                                .default_value("green")
                                .takes_value(true))
                            .arg(Arg::with_name("stroke")
                                .long("stroke")
                                .value_name("THICKNESS")
                                .help("Line stroke around objects")
                                .default_value("0")
                                .takes_value(true))
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

    let ground_color = matches.value_of("ground").unwrap();
    let sky_color = matches.value_of("sky").unwrap();
    let apple_color = matches.value_of("apple").unwrap();
    let flower_color = matches.value_of("flower").unwrap();
    let killer_color = matches.value_of("killer").unwrap();
    let player_color = matches.value_of("player").unwrap();
    let stroke = matches.value_of("stroke").unwrap();
    let scale = matches.value_of("scale").unwrap().parse::<usize>().unwrap();
    let pad = matches.value_of("pad").unwrap().parse::<f64>().unwrap();
    let level = Level::load(input_file.to_str().unwrap()).unwrap();

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

    for object in &level.objects {
        if object.position.x < min_x { min_x = object.position.x - OBJECT_RADIUS; }
        if object.position.x > max_x { max_x = object.position.x + OBJECT_RADIUS; }
        if object.position.y < min_y { min_y = object.position.y - OBJECT_RADIUS; }
        if object.position.y > max_y { max_y = object.position.y + OBJECT_RADIUS; }
    }

    // Start writing SVG data to buffer.
    let mut buffer = vec![];
    let width = ((max_x + min_x.abs()) * scale as f64) + pad * 2_f64;
    let height = ((max_y + min_y.abs()) * scale as f64) + pad * 2_f64;
    buffer.extend_from_slice(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
                                    width, height).as_bytes());
    buffer.extend_from_slice(format!("\t<rect width=\"100%\" height=\"100%\" style=\"fill: {};\" />\n", ground_color).as_bytes());

    // Polygons.
    buffer.extend_from_slice(format!("\t<path fill-rule=\"evenodd\" fill=\"{}\" d=\"", sky_color).as_bytes());
    for polygon in &level.polygons {
        if !polygon.grass {
            for (n, vertex) in polygon.vertices.iter().enumerate() {
                if n == 0 { buffer.extend_from_slice(b"M"); }
                else { buffer.extend_from_slice(b"L"); }
                let x = ((vertex.x + min_x.abs()) * scale as f64) + pad;
                let y = ((vertex.y + min_y.abs()) * scale as f64) + pad;
                buffer.extend_from_slice(format!("{} {} ", x, y).as_bytes());
            }
        }
    }
    buffer.extend_from_slice(b"Z\" />\n");

    // Objects
    for object in &level.objects {
        let x = ((object.position.x + min_x.abs()) * scale as f64) + pad;
        let y = ((object.position.y + min_y.abs()) * scale as f64) + pad;
        let color = match object.object_type {
            ObjectType::Apple { .. } => apple_color,
            ObjectType::Exit => flower_color,
            ObjectType::Killer => killer_color,
            ObjectType::Player => player_color
        };
        buffer.extend_from_slice(format!("<circle cx=\"{}\" cy=\"{}\" r=\"{}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"{}\" />\n",
                                        x, y, OBJECT_RADIUS * scale as f64, "black", stroke, color).as_bytes());
    }

    buffer.extend_from_slice(b"</svg>");

    // Write buffer to file.
    let mut file = File::create(output_file).unwrap();
    file.write_all(&buffer).unwrap();
}
