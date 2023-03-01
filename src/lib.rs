mod utils;

// use core::fmt;
use std::fmt;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..(width*height))
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

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

                /*
                    row = unsigned int

                    what we want for values of delta = [-1,0,1]
                    by setting the first value to width - 1, we then add the row
                    row + width = (k * width) + row, due to overflow (if you can think of width as the max value)
                    row + (width - 1) = (k * width) + row - 1, which is what we want. we take the modulo of the value
                    by width, which removes the (k*width) section, leaving row-1. this avoids any 0-1 issues
                    with unsigned ints

                 */

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.width;
                let index = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[index] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {

                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbours = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbours) {
                    /*
                        Rule 1 - any live cell with less than 2 live neighbours dies
                        due to underpopulation
                     */
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    /*
                        Rule 2 - any live cell with 2 or 3 live neighbour cells lives
                        on to the next generation
                     */
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    /*
                        Rule 3 - any live cell with greater than 3 live neighbours dies
                        due to overpopulation
                     */ 
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    /*
                        Rule 4 - any dead cell with exactly 3 live neighbour cells becomes
                        a live cell, due to reproduction
                     */
                    (Cell::Dead, 3) => Cell::Alive,
                    /*
                        All other cells remain in the same state
                     */
                    (otherwise, _) => otherwise
                };

                next[index] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Dead => '◻',
                    Cell::Alive => '◼',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?; 
        }

        Ok(())
    }
}