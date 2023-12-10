pub fn steps_to_farthest_loop_end(file: &str) -> usize {
    let maze = Maze::parse(file);
    let shortest_loop_length = find_shortest_loop_in_maze(&maze).expect("No loops found in maze");

    shortest_loop_length / 2
}

fn find_shortest_loop_in_maze(maze: &Maze) -> Option<usize> {
    let path_lengths = vec![
        maze.try_path(&maze.maze_start, Direction::North),
        maze.try_path(&maze.maze_start, Direction::East),
        maze.try_path(&maze.maze_start, Direction::South),
        maze.try_path(&maze.maze_start, Direction::West),
    ]
    .into_iter()
    .filter_map(|elem| elem)
    .collect::<Vec<usize>>();
    path_lengths.iter().min().copied()
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

    pub fn try_path(&self, start: &Position, direction: Direction) -> Option<usize> {
        println!("Trying path from start at {:?} in direction {:?}", start, direction);
        let mut path_length = 1;
        let mut current_position = start.clone();
        let mut current_direction = direction;
        loop {
            println!("\nPath length: {path_length}");
            println!("Position: {:?}", current_position);
            println!("Direction: {:?}", current_direction);
            let maybe_destination =
                self.get_relative_element(&current_position, &current_direction.clone());
            println!("Maybe destination: {:?}", maybe_destination);

            match maybe_destination {
                None => break None,
                Some((pos, pipe)) => {
                    let direction_in = current_direction.opposite();
                    println!("Entering the pipe from {:?}", direction_in);
                    let maybe_next = pipe
                        .go_through_pipe(&direction_in)
                        .map(|direction_out| (pos, direction_out));
                    println!("Maybe next: {:?}", maybe_next);
                    match maybe_next {
                        Some((next_position, next_direction)) => {
                            current_position = next_position;
                            current_direction = next_direction;
                            path_length += 1;
                        }
                        None => {
                            if pos == *start {
                                println!("Reached start!\n");
                                break Some(path_length);
                            } else {
                                println!("Pipe led to nowhere!\n");
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
        println!("Entering a pipe {:?} from {:?}", self, enter_from);
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
