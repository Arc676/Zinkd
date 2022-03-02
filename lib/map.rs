// MIT/Apache 2.0 dual license
// Apache 2.0
// Copyright 2022 Arc676/Alessandro Vinciguerra <alesvinciguerra@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// MIT
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation the
// rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
// sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use crate::items;
use rand::Rng;

type Direction = u8;
pub const NORTH: u8 = 1 << 0;
pub const SOUTH: u8 = 1 << 1;
pub const LONGITUDINAL: u8 = NORTH | SOUTH;
pub const EAST: u8 = 1 << 2;
pub const WEST: u8 = 1 << 3;
pub const LATITUDINAL: u8 = EAST | WEST;
pub const OMNIDIRECTIONAL: u8 = LONGITUDINAL | LATITUDINAL;

pub enum GridCell {
    Wall,
    Path(Direction, items::PossibleItem),
    Goal,
}

#[derive(Copy, Clone)]
pub struct Coordinates(pub usize, pub usize);

type Grid = Vec<Vec<GridCell>>;
pub struct Map {
    grid: Grid,
    goal: Coordinates,
    starting_points: Vec<Coordinates>,
}

impl Map {
    pub fn generate_random_map(
        map_width: usize,
        map_height: usize,
        players: u32,
        travel_distance: usize,
    ) -> Self {
        let mut grid = Grid::with_capacity(map_height);
        for row in 0..map_height {
            grid.push(Vec::with_capacity(map_width));
            for _ in 0..map_width {
                grid[row].push(GridCell::Wall);
            }
        }

        let mut map = Map {
            grid,
            goal: Coordinates(0, 0),
            starting_points: vec![],
        };

        // Randomly place goal
        let goal = map.get_random_cell();
        map.goal = goal;
        map.set_cell(goal, GridCell::Goal);

        // Set random starting positions for players
        for _ in 0..players {
            let start = map.get_random_cell_with_distance(goal, travel_distance);
            map.connect_cells(start, goal);

            map.starting_points.push(start);
        }

        map
    }

    fn set_cell(&mut self, coordinates: Coordinates, cell: GridCell) {
        let Coordinates(x, y) = coordinates;
        self.grid[y][x] = cell;
    }

    fn supplement_cell(&mut self, coordinates: Coordinates, direction: Direction) {
        let Coordinates(x, y) = coordinates;
        match &mut self.grid[y][x] {
            GridCell::Wall => self.set_cell(coordinates, GridCell::Path(direction, None)),
            GridCell::Path(existing, _) => {
                *existing |= direction;
            }
            GridCell::Goal => panic!("Cannot supplement goal cell"),
        }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn get_random_cell(&self) -> Coordinates {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..self.width());
        let y = rng.gen_range(0..self.height());
        Coordinates(x, y)
    }

    fn get_random_cell_with_distance(&self, target: Coordinates, distance: usize) -> Coordinates {
        let Coordinates(x0, y0) = target;
        let mut rng = rand::thread_rng();
        let x_low = if x0 < distance { 0 } else { x0 - distance };
        let x = rng.gen_range(x_low..=(x0 + distance).min(self.width() - 1));
        let dx = x0.max(x) - x0.min(x);
        let dy = distance - dx;
        if y0 + dy >= self.height() {
            Coordinates(x, y0 - dy)
        } else if y0 < dy {
            Coordinates(x, y0 + dy)
        } else if rng.gen_bool(0.5) {
            Coordinates(x, y0 - dy)
        } else {
            Coordinates(x, y0 + dy)
        }
    }

    fn connect_cells(&mut self, start: Coordinates, end: Coordinates) {
        let Coordinates(x0, y0) = start;
        let Coordinates(x1, y1) = end;

        let mut corner = true;

        if x0 != x1 {
            let range = if x0 < x1 { (x0 + 1)..x1 } else { (x1 + 1)..x0 };
            self.straight_path(range, true, y0);
        } else {
            corner = false;
        }

        if y0 != y1 {
            let range = if y0 < y1 { (y0 + 1)..y1 } else { (y1 + 1)..y0 };
            self.straight_path(range, false, x1);
        } else {
            corner = false;
        }

        if corner {
            self.supplement_cell(Coordinates(x1, y0), NORTH | EAST | SOUTH | WEST);
        }
    }

    fn straight_path<R>(&mut self, range: R, x_range: bool, fixed_coord: usize)
    where
        R: IntoIterator<Item = usize>,
    {
        for coord in range {
            let node = if x_range {
                Coordinates(coord, fixed_coord)
            } else {
                Coordinates(fixed_coord, coord)
            };
            let direction = if x_range { EAST | WEST } else { NORTH | SOUTH };
            self.supplement_cell(node, direction);
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Coordinates, &GridCell)> {
        self.grid.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, cell)| (Coordinates(x, y), cell))
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::map::*;

    #[test]
    fn generate_map() {
        let map = Map::generate_random_map(10, 10, 3, 5);
        let mut render = [[' '; 10]; 10];
        for (position, cell) in map.iter() {
            let Coordinates(x, y) = position;
            render[y][x] = match *cell {
                GridCell::Wall => '.',
                GridCell::Path(direction, _) => match direction {
                    NORTH => '^',
                    SOUTH => 'v',
                    EAST => '>',
                    WEST => '<',
                    _ => {
                        if direction == NORTH | SOUTH {
                            '|'
                        } else if direction == EAST | WEST {
                            '-'
                        } else if direction == NORTH | EAST | SOUTH | WEST {
                            '+'
                        } else {
                            '?'
                        }
                    }
                },
                GridCell::Goal => '*',
            };
        }

        for (i, start) in map.starting_points.iter().enumerate() {
            let Coordinates(x, y) = *start;
            render[y][x] = (b'1' + i as u8) as char;
        }

        let rendered = render.iter().fold(String::new(), |mut text, row| {
            text.push('@');
            text.push_str(row.iter().collect::<String>().as_str());
            text.push_str("@\n");
            text
        });
        println!("@@@@@@@@@@@@\n{}@@@@@@@@@@@@", rendered);
    }
}
