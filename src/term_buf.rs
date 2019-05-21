//! Terminal buffer - either a 2D String Vec for wasm, or using ncurses for
//! native

use std::sync::Mutex;

// TODO: Conditional return based on build target
pub fn num_lines() -> i32 {
    39
}

// TODO: Conditional return based on build target
pub fn num_cols() -> i32 {
    178
}

// Static terminal buffer
lazy_static! {
    pub static ref TERM_BUF: Mutex<Vec<Vec<String>>> = Mutex::new(vec![
            vec![' '.to_string(); num_cols() as usize];
            num_lines() as usize
        ]);
}

#[macro_export]
macro_rules! scr_buf {
    () => {
        *crate::term_buf::TERM_BUF.try_lock().unwrap()
    };
    ( $y:expr, $x:expr, $c:expr ) => {
        if $x >= 0 && $x < num_cols() && $y >= 0 && $y < num_lines() {
            TERM_BUF.try_lock().unwrap()[$y as usize][$x as usize] = $c;
        }
    };
}

pub fn place_string(y: i32, mut x: i32, str: &str) -> bool {
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
