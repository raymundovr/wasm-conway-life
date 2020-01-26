mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;
use js_sys::Math;
use web_sys;
// use fixedbitset::FixedBitSet;

macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($ ($t)*).into());
    };
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)] // Each cell is represented as single byte
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        };
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

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let n_row = (row + delta_row) % self.height;
                let n_col = (column + delta_col) % self.width;
                let idx = self.get_index(n_row, n_col);
                count += self.cells[idx] as u8;
            }
        }

        count
    }

    fn random_init(size: u32) -> Vec<Cell> {
        (0..size).map(|_| {            
            if Math::random() < 0.5 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect()
    }

    fn all_dead_init(size: u32) -> Vec<Cell> {
        (0..size).map(|_| { Cell::Dead }).collect()
    }

    /* fn bitset_init(size: u32) -> FixedBitSet {
        let size = size as usize;
        let mut cells = FixedBitSet::with_capacity(size);
        for i in 0..size {
            cells.set(i, i%2 == 0 || i%7 == 0)
        }

        cells
    } */
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match(cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
                /* next.set(idx, match(cell, live_neighbors) {
                    (true, x) if x < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, x) if x > 3 => false,
                    (false, 3) => true,
                    (otherwise, _) => otherwise,
                }); */
                // log!("Testing log...");
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = Universe::random_init(width * height);
        //let cells = Universe::bitset_init(width * height);

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn all_dead(self) -> Universe {
        let cells = Universe::all_dead_init(self.width * self.height);

        Universe {
            width: self.width,
            height: self.height,
            cells
        }
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

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    pub fn draw_glider(&mut self, row: u32, column: u32) {        
        let n_row = (row - 1) % self.height;
        let n_col = (column - 1) % self.width;
        let idx = self.get_index(n_row, n_col);
        self.cells[idx] = Cell::Dead;

        let n_row = (row - 1) % self.height;
        //let n_col = (column - 1) % self.width;
        let idx = self.get_index(n_row, column);
        self.cells[idx] = Cell::Alive;
        //
        
        let n_row = (row - 1) % self.height;
        let n_col = (column + 1) % self.width;
        let idx = self.get_index(n_row, n_col);
        self.cells[idx] = Cell::Dead;

        //let n_row = (row - 1) % self.height;
        let n_col = (column - 1) % self.width;
        let idx = self.get_index(row, n_col);
        self.cells[idx] = Cell::Dead;

        let idx = self.get_index(row, column);
        self.cells[idx] = Cell::Dead;

        let n_row = (row + 1) % self.height;
        let n_col = (column + 1) % self.width;
        let idx = self.get_index(n_row, n_col);
        self.cells[idx] = Cell::Alive;
        
        let n_col = (column + 1) % self.width;
        let idx = self.get_index(row, n_col);
        self.cells[idx] = Cell::Alive;

        let n_row = (row + 1) % self.height;
        let n_col = (column - 1) % self.width;
        let idx = self.get_index(n_row, n_col);
        self.cells[idx] = Cell::Alive;

        for i in [0, 1].iter().cloned() {            
            let n_col = (column + i) % self.width;
            let idx = self.get_index(n_row, n_col);
            self.cells[idx] = Cell::Alive;
        }
    }

    /* pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    } */
}

impl Universe {
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

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Alive {'☹'} else {'☻'};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}