mod utils;

use core::fmt;
use wasm_bindgen::prelude::*;

extern crate web_sys;
use web_sys::console;

#[allow(unused_macros)]
macro_rules! log {
    ( $( $t:tt)* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

pub struct Timer<'a> {
    name: &'a str,
}

impl<'a> Timer<'a> {
    pub fn new(name: &'a str) -> Timer<'a> {
        console::time_with_label(name);
        Timer { name }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        console::time_end_with_label(self.name);
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Alive => Cell::Dead,
            Cell::Dead => Cell::Alive,
        }
    }

    fn set_alive(&mut self) {
        *self = Cell::Alive
    }

    fn set_dead(&mut self) {
        *self = Cell::Dead
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let cells = (0..width * height)
            .map(|_i| {
                // if i % 2 == 0 || i % 7 == 0 {
                if js_sys::Math::random() < 0.3 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn empty(width: u32, height: u32) -> Universe {
        utils::set_panic_hook();

        let cells = (0..width * height).map(|_i| Cell::Dead).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                // log!(
                //     "cell[{}, {}] is initally {:?} and has {} live neighbours",
                //     row,
                //     col,
                //     cell,
                //     live_neighbours,
                // );

                let next_cell = match (cell, live_neighbours) {
                    // Rule 1: Live cell with fewer than 2 live neighbours
                    // dies.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Live cell with two or three live neighbours
                    // lives.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Live cell with more than 3 live neighbours
                    // dies.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Dead cell with exactly 3 neighbours is alive.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All else will remain same
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }

    pub fn insert_glider(&mut self, row: u32, col: u32) {
        // o__
        // _oo
        // oo_

        // row & col is the middle cell
        let previous_row = (row + self.height - 1) % self.height;
        let previous_col = (col + self.width - 1) % self.width;
        let next_row = (row + 1) % self.height;
        let next_col = (col + 1) % self.width;

        let idx = self.get_index(previous_row, previous_col);
        self.cells[idx].set_alive();
        let idx = self.get_index(previous_row, col);
        self.cells[idx].set_dead();
        let idx = self.get_index(previous_row, next_col);
        self.cells[idx].set_dead();

        let idx = self.get_index(row, previous_col);
        self.cells[idx].set_dead();
        let idx = self.get_index(row, col);
        self.cells[idx].set_alive();
        let idx = self.get_index(row, next_col);
        self.cells[idx].set_alive();

        let idx = self.get_index(next_row, previous_col);
        self.cells[idx].set_alive();
        let idx = self.get_index(next_row, col);
        self.cells[idx].set_alive();
        let idx = self.get_index(next_row, next_col);
        self.cells[idx].set_dead();
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '☐' } else { '■' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
