use std::env;
use std::fs::File;
use std::io::prelude::*;


use clap::{Arg, App, SubCommand};
use serde_json::{Value, Error, json};
use serde;

use serde_derive::{Serialize, Deserialize};

use rust_ffi::{transpose, multiply};

#[derive(Serialize, Deserialize)]
struct LinearMatrix {
    inner: Vec<f64>
}

impl LinearMatrix {
    fn new(s: &[f64]) -> LinearMatrix {
        LinearMatrix {
            inner: s.to_vec(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct InputData {
    a: Vec<f64>,
    b: Vec<f64>,
}

fn main() {
    let matches = App::new("MX - Matrix Multithreaded Multiplication")
        .version("0.1.0")
        .author("Sergey Mishin <sergei.a.mishin@gmail.com>")
        .about("Multiplies matrices in parallel")
        .arg(Arg::with_name("inputfile")
             .short("f")
             .long("file")
             .value_name("FILE")
             .help("Sets a custom input file")
             .takes_value(true))
        .get_matches();

    let filename = matches.value_of("inputfile").unwrap_or("");
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
     .expect("something went wrong reading the file");

    let mut inp: InputData = serde_json::from_str(&contents).unwrap();

    let res = multiply(inp.a.as_mut_slice(), inp.b.as_mut_slice());
    println!("res: {:?}", res);
}
