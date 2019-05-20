use crate::common::smoke::*;
use crate::common::*;
use crate::term_buf::place_string;

pub enum SLType {
    Logo,
    C51,
    D51,
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
            place_string(y + i, x, MAN[man_x][i as usize]);
        }
    }

    fn add_smoke(&mut self, y: i32, x: i32) {
        let state = self.get_smoke_state();
        let sum: usize = state.sum;
        let s = &mut state.s;

        if x % 4 == 0 {
            for i in 0..sum {
                let pattern = s[i].ptrn as usize;
                place_string(s[i].y, s[i].x, ERASER[pattern]);
                s[i].y -= DY[pattern];
                s[i].x += DX[pattern];
                let pattern = if pattern < SMOKEPTNS - 1 {
                    s[i].ptrn += 1;
                    s[i].ptrn as usize
                } else {
                    pattern
                };

                place_string(
                    s[i].y,
                    s[i].x,
                    SMOKE[(s[i].kind) as usize][pattern],
                );
            }
            place_string(y, x, SMOKE[sum % 2][0]);
            s[sum].y = y;
            s[sum].x = x;
            s[sum].ptrn = 0;
            s[sum].kind = (sum % 2) as i32;
            state.sum = sum + 1;
        }
    }
}
