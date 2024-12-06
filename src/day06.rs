use aoc2024::{print_day_title, print_part_solution, read_lines};
use color_eyre::Result;
use console::{style, Term};
use std::collections::{HashMap, HashSet};
use std::fmt;

const TEST_INPUT: &str = "assets/test_input_day06";
const INPUT: &str = "assets/input_day06";

pub fn main() -> Result<()> {
    print_day_title(6);

    let (mut map, guard) = read_input(INPUT)?;
    let fields = move_guard(&mut map, guard)?;
    print_part_solution(1, "The guard visited unique fields:", fields);

    is_valid_obstacle(&Position { x: 3, y: 8 }, &map);
    print_part_solution(2, "TBD", "-");

    Ok(())
}

fn move_guard(map: &mut Map, mut guard: Guard) -> Result<usize> {
    loop {
        // map.print_state(&guard);
        println!("");
        println!("");
        map.visited.insert(guard.position.clone());
        map.visited_directional.insert(guard.position.clone(), guard.direction.clone());

        let position_ahead = guard.look_ahead();
        // println!("Current position is: {}", guard.position);
        // println!("Next position is: {}", position_ahead);
        //
        if map.is_out_of_bounds(&position_ahead) {
            // println!("Next position out of bounds");
            break;
        }

        if map.is_blocked(&position_ahead) {
            // println!("Next is blocked, turning");
            guard.direction = guard.direction.turn();
            map.add_turn(guard.position.clone());
            continue;
        }

        // println!("Moving to next");
        guard.move_ahead();
        // std::thread::sleep(std::time::Duration::from_millis(50));
    }

    Ok(map.visited.len())
}

fn is_valid_obstacle(position: &Position, map: &Map) -> bool {
    let mut possible_intersections: HashSet<(Position, Position, Position)> = HashSet::new();
    for turn in &map.turns {
        // find turn intersections:
        let same_row = map.y_turns.get(&turn.y).unwrap();
        let same_col = map.x_turns.get(&turn.x).unwrap();

        println!("Same Row: {}", same_row.len());
        println!("Same Col: {}", same_col.len());
        for turn_2 in same_row {
            if turn != turn_2 {
                for turn_3 in same_col {
                    if turn != turn_3 && turn_2 != turn_3 {
                        possible_intersections.insert((turn.clone(), turn_2.clone(), turn_3.clone()));
                    }
                }
            }
        }

        // Ensure there's at least one other turn in the same row and column
        // if same_row.is_some() && same_col.is_some() {
        //     possible_intersections.insert(turn.clone());
        // }
    }

    println!("Possible intersections: {}", possible_intersections.len());

    let mut sum = 0;
    for intersection in possible_intersections {
        let (turn1, turn2, turn3) = intersection;

        if let Some(turn4) = find_fourth_turn(&turn1, &turn2, &turn3) {
        println!("4th turn needs to be done at: {:#?}", turn4);

        if let Some(obstacle) = calculate_obstacle_position(&turn4, &map.visited_directional) {
                if !map.is_out_of_bounds(&obstacle) && obstacle != map.start_position {
                    sum += 1;
                }

            }

        }
    }
    println!("Sum: {}", sum);

    true
}

fn find_fourth_turn(
    turn1: &Position,
    turn2: &Position,
    turn3: &Position,
) -> Option<Position> {
    // Extract coordinates for the turns
    let (x1, y1) = (turn1.x, turn1.y);
    let (x2, y2) = (turn2.x, turn2.y);
    let (x3, y3) = (turn3.x, turn3.y);

    // Determine the possible x and y for the fourth turn
    let x_candidates = [x1, x2, x3];
    let y_candidates = [y1, y2, y3];

    for &x in &x_candidates {
        for &y in &y_candidates {
            let fourth_turn = Position { x, y };

            // Ensure it's not one of the three existing turns
            if fourth_turn != *turn1 && fourth_turn != *turn2 && fourth_turn != *turn3 {
                return Some(fourth_turn);
            }
        }
    }

    None // No valid fourth turn found
}

fn calculate_obstacle_position(
    fourth_turn: &Position,
    visited_directional: &HashMap<Position, Direction>,
) -> Option<Position> {
    // Get the guard's direction at the fourth turn
    let current_direction = visited_directional.get(fourth_turn)?;

    // Calculate the position ahead based on the current direction
    let position_ahead = match current_direction {
        Direction::Up => Position {
            x: fourth_turn.x,
            y: fourth_turn.y + 1,
        },
        Direction::Right => Position {
            x: fourth_turn.x + 1,
            y: fourth_turn.y,
        },
        Direction::Down => Position {
            x: fourth_turn.x,
            y: fourth_turn.y - 1,
        },
        Direction::Left => Position {
            x: fourth_turn.x - 1,
            y: fourth_turn.y,
        },
    };

    // Return the position where the obstacle needs to be placed
    Some(position_ahead)
}

fn has_t_intersection(turns_in_row: &HashSet<Position>, turns_in_col: &HashSet<Position>) -> bool {
    for row_turn in turns_in_row {
        for col_turn in turns_in_col {
            // Check if the intersection of row_turn and col_turn matches
            if row_turn.x == col_turn.x && row_turn.y == col_turn.y {
                // return Some(Position { x: row_turn.x, y: col_turn.y });
                return true;
            }
        }
    }
    // None
    false
}

fn read_input(path: &str) -> Result<(Map, Guard)> {
    let mut lines = read_lines(path)?;

    let mut width = 0;
    let mut obstacles: HashSet<Obstacle> = HashSet::default();
    let mut guard_direction: Direction = Direction::Up;
    let mut guard_position: Position = Position { x: 0, y: 0 };

    let mut x = 0;
    let mut y = 0;
    lines.map(|line| line.unwrap()).for_each(|line| {
        let chars: Vec<char> = line.chars().collect();
        width = chars.len();

        chars.iter().for_each(|char| {
            match *char {
                '#' => {
                    obstacles.insert(Obstacle::new(x, y));
                }
                '^' => {
                    guard_position = Position { x, y };
                    guard_direction = Direction::Up;
                }
                '>' => {
                    guard_position = Position { x, y };
                    guard_direction = Direction::Left;
                }
                'v' => {
                    guard_position = Position { x, y };
                    guard_direction = Direction::Down;
                }
                '<' => {
                    guard_position = Position { x, y };
                    guard_direction = Direction::Right;
                }
                '.' => (),
                _ => {
                    panic!("Unknown char");
                }
            }

            x += 1;
        });

        x = 0;
        y += 1;
    });

    println!("Map has height: {} and width: {}", y, width);

    Ok((
        Map::new(
            y,
            width.try_into().unwrap(),
            obstacles,
            guard_position.clone(),
        ),
        Guard {
            position: guard_position,
            direction: guard_direction,
        },
    ))
}

#[derive(Debug)]
struct Map {
    height: i32,
    width: i32,
    obstacles: HashSet<Obstacle>,
    visited: HashSet<Position>,
    visited_directional: HashMap<Position, Direction>,
    start_position: Position,
    turns: HashSet<Position>,
    x_turns: HashMap<i32, HashSet<Position>>,
    y_turns: HashMap<i32, HashSet<Position>>,
    row_obstacles: HashMap<i32, HashSet<i32>>,
    col_obstacles: HashMap<i32, HashSet<i32>>,
}

impl Map {
    fn add_turn(&mut self, position: Position) {
        self.turns.insert(position.clone());

        self.x_turns
            .entry(position.x)
            .or_insert_with(HashSet::new)
            .insert(position.clone());

        self.y_turns
            .entry(position.y)
            .or_insert_with(HashSet::new)
            .insert(position);
    }
    fn new(
        height: i32,
        width: i32,
        obstacles: HashSet<Obstacle>,
        start_position: Position,
    ) -> Self {
        let mut row_obstacles = HashMap::new();
        let mut col_obstacles = HashMap::new();

        for obstacle in &obstacles {
            row_obstacles
                .entry(obstacle.position.y)
                .or_insert_with(HashSet::new)
                .insert(obstacle.position.x);
            col_obstacles
                .entry(obstacle.position.x)
                .or_insert_with(HashSet::new)
                .insert(obstacle.position.y);
        }

        Map {
            height,
            width,
            obstacles,
            visited: HashSet::new(),
            visited_directional: HashMap::new(),
            start_position,
            turns: HashSet::new(),
            x_turns: HashMap::new(),
            y_turns: HashMap::new(),
            row_obstacles,
            col_obstacles,
        }
    }
    fn is_out_of_bounds(&self, position: &Position) -> bool {
        position.x < 0 || position.x >= self.width || position.y < 0 || position.y >= self.height
    }
    fn is_blocked(&self, position: &Position) -> bool {
        self.obstacles.contains(&Obstacle {
            position: position.clone(),
        })
    }
    fn turn_in_same_row_and_col(&self, position: &Position) -> bool {
        // Check for turns in the same row
        let turn_in_row = self.turns.iter().any(|turn| turn.y == position.y);

        // Check for turns in the same col
        let turn_in_col = self.turns.iter().any(|turn| turn.x == position.x);

        turn_in_row && turn_in_col
    }

    fn turns_in_same_row(&self, position: &Position) -> HashSet<Position> {
        self.turns
            .iter()
            .filter(|turn| turn.y == position.y)
            .cloned()
            .collect()
    }

    fn turns_in_same_col(&self, position: &Position) -> HashSet<Position> {
        self.turns
            .iter()
            .filter(|turn| turn.x == position.x)
            .cloned()
            .collect()
    }

    fn print_state(&self, guard: &Guard) {
        let term = Term::stdout();
        term.clear_screen().unwrap(); // Clear the screen before printing

        println!("");
        for y in (0..self.height).rev() {
            print!(" ");
            // Print from top to bottom
            for x in 0..self.width {
                let position = Position { x, y };

                // Check if this position is where the guard is
                if guard.position == position {
                    let guard_char = match guard.direction {
                        Direction::Up => '^',
                        Direction::Right => '>',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                    };
                    print!("{}", style(guard_char).cyan().bold());
                }
                // Check if the position is an obstacle
                else if self.obstacles.contains(&Obstacle {
                    position: position.clone(),
                }) {
                    print!("{}", style("#").red());
                }
                // Check if the position was visited
                else if self.visited.contains(&position) {
                    print!("{}", style("X").dim());
                }
                // Default representation for unvisited and unobstructed positions
                else {
                    print!("{}", style(".").dim());
                }
            }
            println!(); // Move to the next line
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug)]
struct Guard {
    position: Position,
    // up - 1, left - 2, down - 3, right - 4
    direction: Direction,
}

impl Guard {
    fn look_ahead(&self) -> Position {
        let (dx, dy) = match self.direction {
            Direction::Up => (0, 1),
            Direction::Right => (-1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (1, 0),
        };

        Position {
            x: self.position.x - dx,
            y: self.position.y - dy,
        }
    }
    fn move_ahead(&mut self) {
        let (dx, dy) = match self.direction {
            Direction::Up => (0, 1),
            Direction::Right => (-1, 0),
            Direction::Down => (0, -1),
            Direction::Left => (1, 0),
        };

        self.position = Position {
            x: self.position.x - dx,
            y: self.position.y - dy,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Obstacle {
    position: Position,
}

impl Obstacle {
    fn new(x: i32, y: i32) -> Self {
        Self {
            position: Position { x, y },
        }
    }
}

#[derive(PartialEq, Clone, Debug, Hash, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
