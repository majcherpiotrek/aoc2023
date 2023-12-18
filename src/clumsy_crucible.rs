use std::collections::{BinaryHeap, HashMap, HashSet};

type NodeAddress = (usize, usize);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    coordinates: NodeAddress,
    heat_loss_factor: u8,
    shortest_distance: usize,
    previous_node: Option<NodeAddress>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    node_address: NodeAddress,
    accumulated_heat_loss: usize,
    direction: Direction,
    moves_in_direction: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .accumulated_heat_loss
            .cmp(&self.accumulated_heat_loss)
            .then_with(|| {
                other
                    .node_address
                    .0
                    .cmp(&self.node_address.0)
                    .then_with(|| other.node_address.1.cmp(&self.node_address.1))
            })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn find_shortest_path(file: &str) -> Option<usize> {
    let (nodes, mut nodes_map) = parse_input(file);

    let start_node_address: NodeAddress = (0, 0);
    let rows_number = nodes.len();
    let columns_number = nodes.first().unwrap().len();
    let end_node_address: NodeAddress = (rows_number - 1, columns_number - 1);

    let mut heap = BinaryHeap::new();

    heap.push(State {
        node_address: start_node_address,
        accumulated_heat_loss: 0,
        direction: Direction::Up,
        moves_in_direction: 0,
    });

    let mut results: Vec<State> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    while let Some(current_state) = heap.pop() {
        //println!("State: {:?}", current_state);

        if current_state.node_address == end_node_address {
            results.push(current_state);
            continue;
        }

        let seen_key = build_seen_key(&current_state);
        if seen.contains(&seen_key) {
            continue;
        } else {
            seen.insert(seen_key);
        }

        let neighbours = if current_state.node_address == start_node_address {
            vec![(Direction::Right, (0, 1)), (Direction::Down, (1, 0))]
        } else {
            get_possible_neighbours(&current_state, &rows_number, &columns_number)
        };

        //println!("Checking neighbours: {:?}", neighbours);
        for neighbour in neighbours.iter() {
            let neighbour_data = nodes_map.get_mut(&build_key(neighbour.1)).unwrap();
            //println!("Current neighbour data: {:?}", neighbour_data);

            let next = State {
                node_address: neighbour_data.coordinates,
                accumulated_heat_loss: current_state.accumulated_heat_loss
                    + neighbour_data.heat_loss_factor as usize,
                direction: neighbour.0,
                moves_in_direction: if neighbour.0 == current_state.direction {
                    current_state.moves_in_direction + 1
                } else {
                    1
                },
            };

            //println!("Next: {:?}", next);
            let neighbour_seen_key = build_seen_key(&next);

            if seen.contains(&neighbour_seen_key) {
                continue;
            } else {
                heap.push(next.clone());
            }

            if next.accumulated_heat_loss < neighbour_data.shortest_distance {
                //println!("Updating path");
                neighbour_data.shortest_distance = next.accumulated_heat_loss;
                neighbour_data.previous_node = Some(current_state.node_address);
            }
            //println!("\n");
        }

        //println!("Heap at the end: {:?}", heap);
        //println!("\n");
    }

    //println!("Results {:?}", results);

    nodes_map
        .get(&build_key(end_node_address))
        .map(|r| r.shortest_distance)
}

pub fn find_shortest_path2(file: &str) -> Option<usize> {
    let (nodes, mut nodes_map) = parse_input(file);

    let start_node_address: NodeAddress = (0, 0);
    let rows_number = nodes.len();
    let columns_number = nodes.first().unwrap().len();
    let end_node_address: NodeAddress = (rows_number - 1, columns_number - 1);

    let mut heap = BinaryHeap::new();

    heap.push(State {
        node_address: start_node_address,
        accumulated_heat_loss: 0,
        direction: Direction::Up,
        moves_in_direction: 0,
    });

    let mut results: Vec<State> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    while let Some(current_state) = heap.pop() {
        //println!("State: {:?}", current_state);

        if current_state.node_address == end_node_address {
            results.push(current_state);
            continue;
        }

        let seen_key = build_seen_key(&current_state);
        if seen.contains(&seen_key) {
            continue;
        } else {
            seen.insert(seen_key);
        }

        let neighbours = if current_state.node_address == start_node_address {
            vec![(Direction::Right, (0, 1)), (Direction::Down, (1, 0))]
        } else {
            get_possible_neighbours2(&current_state, &rows_number, &columns_number)
        };

        //println!("Checking neighbours: {:?}", neighbours);
        for neighbour in neighbours.iter() {
            let neighbour_data = nodes_map.get_mut(&build_key(neighbour.1)).unwrap();
            //println!("Current neighbour data: {:?}", neighbour_data);

            let next = State {
                node_address: neighbour_data.coordinates,
                accumulated_heat_loss: current_state.accumulated_heat_loss
                    + neighbour_data.heat_loss_factor as usize,
                direction: neighbour.0,
                moves_in_direction: if neighbour.0 == current_state.direction {
                    current_state.moves_in_direction + 1
                } else {
                    1
                },
            };

            //println!("Next: {:?}", next);
            let neighbour_seen_key = build_seen_key(&next);

            if seen.contains(&neighbour_seen_key) {
                continue;
            } else {
                heap.push(next.clone());
            }

            if next.accumulated_heat_loss < neighbour_data.shortest_distance {
                //println!("Updating path");
                neighbour_data.shortest_distance = next.accumulated_heat_loss;
                neighbour_data.previous_node = Some(current_state.node_address);
            }
            //println!("\n");
        }

        //println!("Heap at the end: {:?}", heap);
        //println!("\n");
    }

    //println!("Results {:?}", results);

    nodes_map
        .get(&build_key(end_node_address))
        .map(|r| r.shortest_distance)
}

fn build_seen_key(state: &State) -> String {
    format!(
        "{:?}-{:?}-{}",
        state.node_address, state.direction, state.moves_in_direction
    )
}

fn get_possible_neighbours(
    state: &State,
    rows_number: &usize,
    columns_number: &usize,
) -> Vec<(Direction, NodeAddress)> {
    let (row, column) = state.node_address;

    let mut forbidden_directions = Vec::new();

    let opposite_direction = match state.direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    };

    forbidden_directions.push(opposite_direction);

    if state.moves_in_direction == 3 {
        forbidden_directions.push(state.direction);
    }

    let mut neighbours: Vec<(Direction, NodeAddress)> = Vec::new();

    let is_up_forbidden = forbidden_directions.contains(&Direction::Up);
    let is_right_forbidden = forbidden_directions.contains(&Direction::Right);
    let is_down_forbidden = forbidden_directions.contains(&Direction::Down);
    let is_left_forbidden = forbidden_directions.contains(&Direction::Left);

    if !is_down_forbidden && row + 1 < *rows_number {
        let neighbour = (row + 1, column);
        neighbours.push((Direction::Down, neighbour));
    }

    if !is_up_forbidden && row > 0 {
        let neighbour = (row - 1, column);

        neighbours.push((Direction::Up, neighbour));
    }

    if !is_right_forbidden && column + 1 < *columns_number {
        let neighbour = (row, column + 1);
        neighbours.push((Direction::Right, neighbour));
    }

    if !is_left_forbidden && column > 0 {
        let neighbour = (row, column - 1);
        neighbours.push((Direction::Left, neighbour));
    }

    neighbours
}
fn get_possible_neighbours2(
    state: &State,
    rows_number: &usize,
    columns_number: &usize,
) -> Vec<(Direction, NodeAddress)> {
    let (row, column) = state.node_address;

    if state.moves_in_direction < 4 {
        let next = move_in_direction(&state.node_address, &state.direction, rows_number, columns_number);
        
        return match next {
           None => vec![],
            Some(n) => vec![(state.direction, n)]
        }
    }

    let mut forbidden_directions = Vec::new();

    let opposite_direction = match state.direction {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    };

    forbidden_directions.push(opposite_direction);

    if state.moves_in_direction >= 10 {
        forbidden_directions.push(state.direction);
    }

    let mut neighbours: Vec<(Direction, NodeAddress)> = Vec::new();

    let is_up_forbidden = forbidden_directions.contains(&Direction::Up);
    let is_right_forbidden = forbidden_directions.contains(&Direction::Right);
    let is_down_forbidden = forbidden_directions.contains(&Direction::Down);
    let is_left_forbidden = forbidden_directions.contains(&Direction::Left);

    if !is_down_forbidden && row + 1 < *rows_number {
        let neighbour = (row + 1, column);
        neighbours.push((Direction::Down, neighbour));
    }

    if !is_up_forbidden && row > 0 {
        let neighbour = (row - 1, column);

        neighbours.push((Direction::Up, neighbour));
    }

    if !is_right_forbidden && column + 1 < *columns_number {
        let neighbour = (row, column + 1);
        neighbours.push((Direction::Right, neighbour));
    }

    if !is_left_forbidden && column > 0 {
        let neighbour = (row, column - 1);
        neighbours.push((Direction::Left, neighbour));
    }

    neighbours
}

fn move_in_direction(
    node_address: &NodeAddress,
    direction: &Direction,
    rows_number: &usize,
    columns_number: &usize,
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if node_address.0 > 0 {
                Some((node_address.0 - 1, node_address.1))
            } else {
                None
            }
        }
        Direction::Right => {
            if node_address.1 + 1 < *columns_number {
                Some((node_address.0, node_address.1 + 1))
            } else {
                None
            }
        }
        Direction::Down => {
            if node_address.0 + 1 < *rows_number {
                Some((node_address.0 + 1, node_address.1))
            } else {
                None
            }
        }
        Direction::Left => {
            if node_address.1 > 0 {
                Some((node_address.0, node_address.1 - 1))
            } else {
                None
            }
        }
    }
}

fn arrived_from(current_node: &NodeAddress, prev_node: &NodeAddress) -> Direction {
    if current_node.0 == prev_node.0 {
        if current_node.1 > prev_node.1 {
            Direction::Right
        } else {
            Direction::Left
        }
    } else if current_node.1 == prev_node.1 {
        if current_node.0 > prev_node.0 {
            Direction::Down
        } else {
            Direction::Up
        }
    } else {
        panic!("illegal situation");
    }
}

fn parse_input(file: &str) -> (Vec<Vec<u8>>, HashMap<String, Node>) {
    let mut nodes: Vec<Vec<u8>> = Vec::new();
    let mut nodes_map: HashMap<String, Node> = HashMap::new();

    let lines = file.split("\n").filter(|l| !l.is_empty());
    for (i, line) in lines.enumerate() {
        let mut numbers: Vec<u8> = Vec::new();

        for (j, c) in line.chars().enumerate() {
            let num = c.to_string().parse::<u8>().unwrap();
            let key = build_key((i, j));
            let node = Node {
                coordinates: (i, j),
                heat_loss_factor: num,
                shortest_distance: if i == 0 && j == 0 { 0 } else { std::usize::MAX },
                previous_node: None,
            };

            numbers.push(num);
            nodes_map.insert(key, node);
        }

        nodes.push(numbers);
    }

    (nodes, nodes_map)
}

fn build_key(node_address: NodeAddress) -> String {
    format!("{}-{}", node_address.0, node_address.1)
}
