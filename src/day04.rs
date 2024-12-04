use aoc2024::{print_part_solution, read_lines, print_day_title};
use color_eyre::Result;
use std::collections::HashMap;
use std::marker::PhantomData;

const TEST_INPUT: &str = "assets/test_input_day04";
const INPUT: &str = "assets/input_day04";

struct Part_1;
struct Part_2;

#[derive(Debug)]
struct Input<Part> {
    data: Vec<Vec<char>>,
    target: Vec<Vec<char>>,
    _marker: PhantomData<Part>,
}

pub fn main() -> Result<()> {
    print_day_title(4);

    let matches = Input::<Part_1>::default().from_file(INPUT, 1)?.parse()?;
    print_part_solution(1, "Total of matches:", matches);

    let matches = Input::<Part_2>::default().from_file(INPUT, 1)?.parse()?;
    print_part_solution(2, "Total of matches:", matches);

    Ok(())
}

impl<Part> Input<Part> {
    fn from_file(mut self, file: &str, part: i8) -> Result<Self> {
        let lines = read_lines(file)?;

        lines.map_while(Result::ok).for_each(|line| {
            let line: Vec<char> = line.chars().collect();
            self.add_line(line);
        });

        Ok(self)
    }

    fn add_line(&mut self, line: Vec<char>) {
        self.data.push(line)
    }

    fn loop_through_input<F, G>(&self, continue_condition: G, action: F) -> Result<i32>
    where
        G: Fn(char) -> bool,
        F: Fn(&Self, (usize, usize)) -> Result<i32>,
    {
        let mut matches = 0;

        for (y, row) in self.data.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if continue_condition(col) {
                    continue;
                }

                matches += action(&self, (x, y))?;
            }
        }

        Ok(matches)
    }

    fn get_at_position(&self, position: (usize, usize)) -> Option<char> {
        let (x, y) = position;
        self.data.get(y)?.get(x).copied()
    }
}

impl Input<Part_1> {
    fn parse(&self) -> Result<i32> {
        self.loop_through_input(|char| char != 'X', Self::check_at_x)
    }

    fn check_at_x(&self, position: (usize, usize)) -> Result<i32> {
        let (x, y) = position;

        let directions = [
            (1, 0),   // forward: x++
            (-1, 0),  // backward: x--
            (0, -1),  // up: y--
            (0, 1),   // down y++
            (-1, 1),  // backward/down: x--, y++
            (-1, -1), // backward/up: x--, y--
            (1, -1),  // forward/up: x++,y--
            (1, 1),   // forward/down: x++, y++
        ];

        let mut matches = 0;
        for (dr, dc) in directions {
            if self.check_direction(position, (dr, dc)) {
                matches += 1;
            }
        }

        Ok(matches)
    }

    fn check_direction(&self, start: (usize, usize), direction: (i8, i8)) -> bool {
        let (mut x, mut y) = start;
        let (dx, dy) = direction;
        let length = self.target[0].len();
        let mut i = 0;

        while let Some(value) = self.get_at_position((x, y)) {
            if self.target[0][i] != value {
                return false;
            }

            if i >= length - 1 {
                return true;
            }

            x = match checked_move(x, dx) {
                Some(new) => new,
                None => break,
            };
            y = match checked_move(y, dy) {
                Some(new) => new,
                None => break,
            };

            i += 1;
        }

        false
    }
}

impl Input<Part_2> {
    fn parse(&self) -> Result<i32> {
        self.loop_through_input(|char| char != 'M' && char != 'S', Self::check_at_m)
    }

    fn check_at_m(&self, position: (usize, usize)) -> Result<i32> {
        // Relevant:
        // (x,y)    | ---------- | (x+2, y)   |
        //  ------- | (x+1, y+1) | ---------- |
        // (x, y+2) | ---------- | (x+2, y+2) |
        let (x, y) = position;
        let mut matches = true;

        let top_left: char = match self.get_at_position((x, y)) {
            Some(char) => char,
            None => return Ok(0),
        };
        let top_right: char = match self.get_at_position((x + 2, y)) {
            Some(char) => char,
            None => return Ok(0),
        };
        let middle_middle: char = match self.get_at_position((x + 1, y + 1)) {
            Some(char) => char,
            None => return Ok(0),
        };
        let bottom_left: char = match self.get_at_position((x, y + 2)) {
            Some(char) => char,
            None => return Ok(0),
        };
        let bottom_right: char = match self.get_at_position((x + 2, y + 2)) {
            Some(char) => char,
            None => return Ok(0),
        };

        // A in the middle
        if middle_middle != 'A' {
            return Ok(0);
        }

        // Top right needs to be checked seperatly
        if top_right != 'S' && top_right != 'M' {
            return Ok(0);
        }

        // If top left is M, bottom right needs to be S
        if top_left == 'S' && bottom_right != 'M' {
            return Ok(0);
        }

        // If top left is S, bottom right needs to be M
        if top_left == 'M' && bottom_right != 'S' {
            return Ok(0);
        }

        // If top right is M, bottom left needs to be S
        if top_right == 'S' && bottom_left != 'M' {
            return Ok(0);
        }

        // If top right is S, bottom left needs to be M
        if top_right == 'M' && bottom_left != 'S' {
            return Ok(0);
        }

        Ok(1)
    }
}

impl Default for Input<Part_1> {
    fn default() -> Self {
        Input {
            data: vec![],
            target: vec![vec!['X', 'M', 'A', 'S']],
            _marker: PhantomData,
        }
    }
}

impl Default for Input<Part_2> {
    fn default() -> Self {
        Input {
            data: vec![],
            target: vec![],
            _marker: PhantomData,
        }
    }
}

fn checked_move(origin: usize, offset: i8) -> Option<usize> {
    if offset > 0 {
        origin.checked_add(offset as usize)
    } else {
        origin.checked_sub((-offset) as usize)
    }
}
