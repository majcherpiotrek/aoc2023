use std::cmp::Ordering;

pub fn surface_inside_loop(file: &str) -> usize {
    let maze = Maze::parse(file);
    let mut shortest_loop = find_shortest_loop_in_maze(&maze).expect("no loops found in maze");

    println!("Shortest loop {:?}", shortest_loop);
    shortest_loop.sort_by(|a, b| {
        let row_ord = a.0.cmp(&b.0);
        if row_ord == Ordering::Equal {
            row_ord
        } else {
            a.1.cmp(&b.1)
        }
    });

    let max_north = shortest_loop
        .iter()
        .min_by(|a, b| a.0.cmp(&b.0))
        .expect("Failed to calculate bounding square");
    let max_east = shortest_loop
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("Failed to calculate bounding square");
    let max_south = shortest_loop
        .iter()
        .max_by(|a, b| a.0.cmp(&b.0))
        .expect("Failed to calculate bounding square");
    let max_west = shortest_loop
        .iter()
        .min_by(|a, b| a.1.cmp(&b.1))
        .expect("Failed to calculate bounding square");

    let north_limit = max_north.0 + 1;
    let east_limit = max_east.1;
    let south_limit = max_south.0;
    let west_limit = max_west.1 + 1;

    println!("north {north_limit}");
    println!("east {east_limit}");
    println!("south {south_limit}");
    println!("west {west_limit}");

    let mut points_inside: Vec<(Position, MazeElement)> = Vec::new();

    for row in north_limit..=south_limit {
        println!("\nROW {row}");
        for start_col in west_limit..=(east_limit - 1) {
            println!("\nCOL {start_col}");
            let mut intersections: Vec<Position> = Vec::new();
            for col in start_col..=east_limit {
                let point_to_check = Position(row, col);
                let intersects = shortest_loop.contains(&point_to_check);
                if intersects {
                    intersections.push(point_to_check);
                }
            }
            println!("Intersections: {}", intersections.len());

            let mut num_of_unique_intersections: usize = 0;

            for (i, intersection) in intersections.iter().enumerate() {
                if i == 0 {
                    num_of_unique_intersections += 1;
                } else {
                    let previous = intersections.get(i - 1).expect("Unexpected error");
                    if intersection.1 - previous.1 > 1 {
                        num_of_unique_intersections += 1;
                    }
                }
            }
            println!("Unique intersections: {num_of_unique_intersections}");

            if num_of_unique_intersections != 0 && num_of_unique_intersections % 2 != 0 {
                if num_of_unique_intersections == 1 && intersections.len() > 1 {
                    continue;
                }
                let element = maze
                    .get_element(&Position(row, start_col))
                    .expect("No such point in the maze");
                match element {
                    MazeElement::Ground => {
                        points_inside.push((Position(row, start_col), element.clone()));
                    }
                    _ => (),
                }
            }
        }
    }

    println!("Points inside {:?}", points_inside);

    points_inside.len()
}

pub fn steps_to_farthest_loop_end(file: &str) -> usize {
    let maze = Maze::parse(file);
    let shortest_loop = find_shortest_loop_in_maze(&maze).expect("no loops found in maze");

    shortest_loop.len() / 2
}

fn find_shortest_loop_in_maze(maze: &Maze) -> Option<Vec<Position>> {
    let paths = vec![
        maze.try_path(&maze.maze_start, Direction::North),
        maze.try_path(&maze.maze_start, Direction::East),
        maze.try_path(&maze.maze_start, Direction::South),
        maze.try_path(&maze.maze_start, Direction::West),
    ]
    .into_iter()
    .filter_map(|elem| elem)
    .collect::<Vec<Vec<Position>>>();
    paths.into_iter().min_by(|a, b| a.len().cmp(&b.len()))
}

struct Maze {
    maze: Vec<Vec<MazeElement>>,
    maze_start: Position,
}

impl Maze {
    pub fn parse(file: &str) -> Maze {
        let mut maybe_maze_start: Option<Position> = None;
        let mut maze: Vec<Vec<MazeElement>> = Vec::new();
        for (row, line) in file.split("\n").enumerate() {
            let mut maze_row: Vec<MazeElement> = Vec::new();
            for (column, c) in line.chars().enumerate() {
                let maze_element = MazeElement::parse(c);

                if maze_element == MazeElement::StartingPosition {
                    maybe_maze_start = Some(Position(row, column));
                }

                maze_row.push(maze_element);
            }
            maze.push(maze_row);
        }

        Maze {
            maze,
            maze_start: maybe_maze_start.expect("Maze start must be specified"),
        }
    }

    pub fn try_path(&self, start: &Position, direction: Direction) -> Option<Vec<Position>> {
        //println!("Trying path from start at {:?} in direction {:?}", start, direction);
        let mut current_position = start.clone();
        let mut current_direction = direction;
        let mut path = vec![start.clone()];
        loop {
            //println!("\nPath: {:?}", path);
            //println!("Position: {:?}", current_position);
            //println!("Direction: {:?}", current_direction);
            let maybe_destination =
                self.get_relative_element(&current_position, &current_direction.clone());
            //println!("Maybe destination: {:?}", maybe_destination);

            match maybe_destination {
                None => break None,
                Some((pos, pipe)) => {
                    let direction_in = current_direction.opposite();
                    //println!("Entering the pipe from {:?}", direction_in);
                    let maybe_next = pipe
                        .go_through_pipe(&direction_in)
                        .map(|direction_out| (pos, direction_out));
                    //println!("Maybe next: {:?}", maybe_next);
                    match maybe_next {
                        Some((next_position, next_direction)) => {
                            current_position = next_position;
                            current_direction = next_direction;
                            path.push(next_position);
                        }
                        None => {
                            if pos == *start {
                                //println!("Reached start!\n");
                                break Some(path);
                            } else {
                                //println!("Pipe led to nowhere!\n");
                                break None;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_relative_element(
        &self,
        position: &Position,
        direction: &Direction,
    ) -> Option<(Position, MazeElement)> {
        position
            .move_in_direction(direction)
            .and_then(|target_position| {
                self.get_element(&target_position)
                    .map(|element| (target_position, element.clone()))
            })
    }

    pub fn get_element(&self, position: &Position) -> Option<&MazeElement> {
        self.maze
            .get(position.0)
            .and_then(|row| row.get(position.1))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MazeElement {
    Pipe { end_a: Direction, end_b: Direction },
    Ground,
    StartingPosition,
}

impl MazeElement {
    pub fn parse(c: char) -> MazeElement {
        match c {
            '.' => MazeElement::Ground,
            '|' => MazeElement::Pipe {
                end_a: Direction::North,
                end_b: Direction::South,
            },
            '-' => MazeElement::Pipe {
                end_a: Direction::East,
                end_b: Direction::West,
            },
            'L' => MazeElement::Pipe {
                end_a: Direction::North,
                end_b: Direction::East,
            },
            'J' => MazeElement::Pipe {
                end_a: Direction::North,
                end_b: Direction::West,
            },
            '7' => MazeElement::Pipe {
                end_a: Direction::South,
                end_b: Direction::West,
            },
            'F' => MazeElement::Pipe {
                end_a: Direction::South,
                end_b: Direction::East,
            },
            'S' => MazeElement::StartingPosition,
            _ => panic!("Unknown maze element"),
        }
    }

    pub fn go_through_pipe(&self, enter_from: &Direction) -> Option<Direction> {
        //println!("Entering a pipe {:?} from {:?}", self, enter_from);
        match self {
            MazeElement::Pipe { end_a, end_b } => {
                if *enter_from == *end_a {
                    Some(*end_b)
                } else if *enter_from == *end_b {
                    Some(*end_a)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Position(usize, usize);

impl Position {
    pub fn move_in_direction(&self, direction: &Direction) -> Option<Position> {
        match direction {
            Direction::North => {
                if self.0 > 0 {
                    Some(Position(self.0 - 1, self.1))
                } else {
                    None
                }
            }
            Direction::East => Some(Position(self.0, self.1 + 1)),
            Direction::South => Some(Position(self.0 + 1, self.1)),
            Direction::West => {
                if self.1 > 0 {
                    Some(Position(self.0, self.1 - 1))
                } else {
                    None
                }
            }
        }
    }
}
