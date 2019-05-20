#[macro_use]
extern crate log;
extern crate getopts;
extern crate wasm_bindgen;
extern crate wasm_logger;
#[macro_use]
extern crate lazy_static;

mod c51;
mod common;
mod d51;
mod logo;

use std::env;
use std::sync::Mutex;
use std::{thread, time};

use getopts::Options;
use wasm_bindgen::prelude::*;
// use ncurses::*;
// use nix::sys::signal;
// use nix::sys::signal::{sigaction, SigAction, SigHandler, SaFlags, SigSet};
use crate::c51::C51;
use crate::common::*;
use crate::d51::D51;
use crate::logo::Logo;

#[wasm_bindgen]
pub fn num_lines() -> i32 {
    39
}

#[wasm_bindgen]
pub fn num_cols() -> i32 {
    178
}

// Static terminal buffer
lazy_static! {
    pub static ref TERM_BUF: Mutex<Vec<Vec<String>>> =
        Mutex::new(vec![
            vec![' '.to_string(); num_cols() as usize];
            num_lines() as usize
        ]);
}

macro_rules! scr_buf {
    () => {
        TERM_BUF.try_lock().unwrap()
    };
    ( $y:expr, $x:expr, $c:expr ) => {
        if $x >= 0 && $x < num_cols() && $y >= 0 && $y < num_lines() {
            TERM_BUF.try_lock().unwrap()[$y as usize][$x as usize] = $c;
        }
    };
}

pub enum SLType {
    Logo,
    C51,
    D51,
}

pub fn my_mvaddstr(y: i32, mut x: i32, str: &str) -> bool {
    let mut chars = str.chars();
    while x < 0 {
        chars.next();
        x += 1;
    }
    for c in chars {
        scr_buf!(y, x, c.to_string());
        x += 1;
    }

    true
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

    (*scr_buf!())
        .iter()
        .map(|l| l.join(""))
        .collect::<Vec<String>>()
        .join("\n")
}

pub struct Config {
    pub accident: bool,
    pub fly: bool,
    pub smoke: bool,
    pub smoke_state: smoke::SmokeState,
}

pub trait Train {
    fn update(&mut self, x: i32) -> bool;
    fn get_smoke_state(&mut self) -> &mut smoke::SmokeState;

    fn add_man(&self, y: i32, x: i32) {
        for i in 0..2 {
            let man_x = ((SL_LENGTH + x) / 12 % 2) as usize;
            my_mvaddstr(y + i, x, MAN[man_x][i as usize]);
        }
    }

    fn add_smoke(&mut self, y: i32, x: i32) {
        use smoke::*;
        let state = self.get_smoke_state();
        let sum: usize = state.sum;
        let s = &mut state.s;

        if x % 4 == 0 {
            for i in 0..sum {
                let pattern = s[i].ptrn as usize;
                my_mvaddstr(s[i].y, s[i].x, ERASER[pattern]);
                s[i].y -= DY[pattern];
                s[i].x += DX[pattern];
                let pattern = if pattern < SMOKEPTNS - 1 {
                    s[i].ptrn += 1;
                    s[i].ptrn as usize
                } else {
                    pattern
                };

                my_mvaddstr(s[i].y, s[i].x, SMOKE[(s[i].kind) as usize][pattern]);
            }
            my_mvaddstr(y, x, SMOKE[sum % 2][0]);
            s[sum].y = y;
            s[sum].x = x;
            s[sum].ptrn = 0;
            s[sum].kind = (sum % 2) as i32;
            state.sum = sum + 1;
        }
    }
}

#[wasm_bindgen(start)]
pub fn wasm_init() {
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
    // SLType::D51 => D51::new(conf).run()
    // };
}

fn main() {
    wasm_logger::init_with_level(log::Level::Trace)
        .expect("Failed to initialize logger");

    println!("{}", display_one_sl(10));
}
