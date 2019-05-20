extern crate log;
extern crate wasm_bindgen;
extern crate wasm_logger;
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod term_buf;
mod c51;
mod common;
mod d51;
mod logo;
mod train;

use crate::c51::C51;
use crate::common::*;
use wasm_bindgen::prelude::*;
// use crate::d51::D51;
// use crate::logo::Logo;
use crate::train::{Config, Train};

#[wasm_bindgen]
pub fn num_cols() -> i32 {
    term_buf::num_cols()
}

#[wasm_bindgen]
pub fn num_lines() -> i32 {
    term_buf::num_lines()
}

#[wasm_bindgen]
pub fn display_one_sl(x: i32) -> String {
    let conf = Config {
        accident: false,
        fly: false,
        smoke: false,
        smoke_state: smoke::SmokeState::new(),
    };

    let mut logo = C51::new(conf);
    logo.update(x);

    scr_buf!()
        .iter()
        .map(|l| l.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

#[wasm_bindgen(start)]
pub fn wasm_init() {
    wasm_logger::init_with_level(log::Level::Trace)
        .expect("Failed to initialize logger");
}

/// Print the command line usage
// fn print_usage(program: &str, opts: &Options) {
// info!("{}", opts.usage(&format!("Usage:\n {} [options]", program)));
// }

/// Non-wasm main function
fn main() {
    wasm_logger::init_with_level(log::Level::Trace)
        .expect("Failed to initialize logger");

    // let args: Vec<String> = env::args().collect();
    // let program = args[0].clone();

    // let mut opts = Options::new();
    // opts.optflag("l", "logo", "select logo");
    // opts.optflag("c", "c51", "select C51");
    // opts.optflag("F", "fly", "enable fly mode");
    // opts.optflag("a", "accident", "enable accident mode");
    // opts.optflag("s", "no-smoke", "disable smoke mode");
    // opts.optflag("", "help", "show this usage message.");
    // let matches = match opts.parse(&args[1..]) {
    // Ok(m) => m,
    // Err(_) => {
    // print_usage(&program, &opts);
    // return;
    // }
    // };
    // if matches.opt_present("help") {
    // print_usage(&program, &opts);
    // return;
    // }
    // let sl_type = if matches.opt_present("logo") {
    // SLType::Logo
    // } else if matches.opt_present("c51") {
    // SLType::C51
    // } else {
    // SLType::D51
    // };

    // let conf = Config {
    // accident: matches.opt_present("accident"),
    // fly: matches.opt_present("fly"),
    // smoke: !matches.opt_present("no-smoke"),
    // smoke_state: smoke::SmokeState::new(),
    // };
    // match sl_type {
    // SLType::Logo => Logo::new(conf).run(),
    // SLType::C51 => C51::new(conf).run(),
    // SLType::D51 => D51::new(conf).run(),
    // };
}
