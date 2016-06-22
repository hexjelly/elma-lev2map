extern crate elma;
extern crate clap;

use elma::lev::Level;
use clap::{Arg, App};

use std::fs::File;
use std::io::Write;

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
                            .arg(Arg::with_name("svg")
                                .long("svg")
                                .help("Specify SVG as output type [Default]"))
                            .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let level = Level::load(input_file).unwrap();

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

    let mut buffer = vec![];
    buffer.extend_from_slice(format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">\n",
                                    (max_x + min_x.abs()) * 20_f64, (max_y + min_y.abs()) * 20_f64).as_bytes());
    buffer.extend_from_slice(format!("\t<rect width=\"100%\" height=\"100%\" style=\"fill: {};\" />\n", "#181048").as_bytes());
    buffer.extend_from_slice(format!("\t<path fill-rule=\"evenodd\" fill=\"{}\" d=\"", "#3078bc").as_bytes());
    for polygon in &level.polygons {
        if !polygon.grass {
            for (n, vertex) in polygon.vertices.iter().enumerate() {
                if n == 0 { buffer.extend_from_slice(b"M"); }
                else { buffer.extend_from_slice(b"L"); }
                let pos = format!("{} {} ", (vertex.x + min_x.abs()) * 20_f64, (vertex.y + min_y.abs()) * 20_f64);
                buffer.extend_from_slice(pos.as_bytes());
            }
        }
    }
    buffer.extend_from_slice(b"Z\" />\n</svg>");

    let mut file = File::create("test.svg").unwrap();
    file.write_all(&buffer).unwrap();
}
