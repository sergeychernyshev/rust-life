use rand;

pub mod shapes;

pub struct GameOfLife {
    width: usize,
    height: usize,
    cells: Vec<bool>,
    prev_cells: Vec<bool>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            width,
            height,
            cells: vec![false; width * height],
            prev_cells: vec![false; width * height],
        }
    }

    pub fn from_shape(shape: shapes::LifeShape) -> GameOfLife {
        GameOfLife {
            width: shape.width,
            height: shape.height,
            cells: shape.cells,
            prev_cells: vec![false; shape.width * shape.height],
        }
    }

    pub fn add(&mut self, shape: shapes::LifeShape, x: usize, y: usize) {
        for sy in 0..shape.height {
            for sx in 0..shape.width {
                let idx = (y + sy) * self.width + (x + sx);
                self.cells[idx] = shape.cells[sy * shape.width + sx];
            }
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> bool {
        self.cells[y * self.width + x]
    }

    pub fn randomize(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = rand::random();
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.get_cell(x, y) { "🦠" } else { "⬛" });
            }
            println!();
        }
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                result.push_str(if self.get_cell(x, y) { "🦠" } else { "⬛" });
            }
            result.push_str("\n");
        }
        result
    }

    pub fn calculate_next_step(&mut self) {
        // @TODO replace this with swapping the pointers instead of copying all the data over
        self.prev_cells.clone_from_slice(&self.cells);

        for y in 0..self.height {
            for x in 0..self.width {
                // calculate number of neighbors
                let mut alive_neighbours = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        // don't count itself
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        // don't count cells outside the grid
                        if nx < 0
                            || nx >= self.width as isize
                            || ny < 0
                            || ny >= self.height as isize
                        {
                            continue;
                        }
                        if self.prev_cells[ny as usize * self.width + nx as usize] {
                            alive_neighbours += 1;
                        }
                    }
                }

                // set new state based on the number of alive neighbours
                let idx = y * self.width + x;
                if self.prev_cells[idx] {
                    self.cells[idx] = alive_neighbours == 2 || alive_neighbours == 3;
                } else {
                    self.cells[idx] = alive_neighbours == 3;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let game = GameOfLife::new(10, 10);
        assert_eq!(game.width, 10);
        assert_eq!(game.height, 10);
        assert_eq!(game.cells.len(), 100);
    }
}
