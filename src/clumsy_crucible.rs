use std::collections::{BinaryHeap, HashMap};

type NodeAddress = (usize, usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node {
    coordinates: NodeAddress,
    heat_loss_factor: u8,
    shortest_distance: usize,
    previous_node: Option<NodeAddress>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct State {
    node_address: NodeAddress,
    accumulated_heat_loss: usize,
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
    });

    let mut total_heat_loss = 0;
    while let Some(State {
        node_address,
        accumulated_heat_loss,
    }) = heap.pop()
    {
        println!("State: {:?}, {:?}", node_address, accumulated_heat_loss);

        //for entry in nodes_map.values() {
        //    println!("{:?} ->  {:?}, prev {:?}", entry.coordinates, entry.shortest_distance, entry.previous_node)
        //}
        if node_address == end_node_address {
            total_heat_loss = accumulated_heat_loss;
            break;
        }

        let node_data = nodes_map.get(&build_key(node_address)).unwrap();

        // If we already found a better path for the node
        if accumulated_heat_loss > node_data.shortest_distance {
            continue;
        }

        let neighbours =
            get_possible_neighbours(&nodes_map, node_address, &rows_number, &columns_number);

        println!("Checking neighbours: {:?}", neighbours);
        for neighbour in neighbours.iter() {
            let neighbour_data = nodes_map.get_mut(&build_key(*neighbour)).unwrap();
            println!("Current neighbour data: {:?}", neighbour_data);
            let next = State {
                node_address: neighbour_data.coordinates,
                accumulated_heat_loss: accumulated_heat_loss
                    + neighbour_data.heat_loss_factor as usize,
            };

            println!("Next: {:?}", next);

            if next.accumulated_heat_loss < neighbour_data.shortest_distance {
                println!("Updating path");
                heap.push(next);
                neighbour_data.shortest_distance = next.accumulated_heat_loss;
                neighbour_data.previous_node = Some(node_address)
            }
            println!("\n");
        }
    }

    let mut path: HashMap<String, String> = HashMap::new();

    let end_node = nodes_map.get(&build_key(end_node_address)).unwrap();

    let mut current_node: &Node = &end_node;
    loop {
        if let Some(prev_node) = current_node
            .previous_node
            .and_then(|addr| nodes_map.get(&build_key(addr)))
        {
            let direction = arrived_from(&current_node.coordinates, &prev_node.coordinates);
            path.insert(
                build_key(prev_node.coordinates),
                match direction {
                    Direction::Up => "^".to_string(),
                    Direction::Right => ">".to_string(),
                    Direction::Down => "V".to_string(),
                    Direction::Left => "<".to_string(),
                },
            );
            current_node = prev_node;
        } else {
            break;
        }
    }

    let to_draw = nodes
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, elem)| {
                    if let Some(path_element) = path.get(&build_key((i, j))) {
                        path_element.clone()
                    } else {
                        elem.to_string()
                    }
                })
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    println!("\n");
    for row in to_draw.iter() {
        println!("{}", row.join(""));
    }

    Some(total_heat_loss)
}

fn get_possible_neighbours(
    nodes_map: &HashMap<String, Node>,
    node_address: NodeAddress,
    rows_number: &usize,
    columns_number: &usize,
) -> Vec<NodeAddress> {
    let (row, column) = node_address;
    let current = nodes_map.get(&build_key(node_address)).unwrap();
    println!("Current {:?}", current);
    let prev_1 = current
        .previous_node
        .map(|prev| nodes_map.get(&build_key(prev)).unwrap());
    println!("P1 {:?}", prev_1);
    let prev_2 = prev_1.and_then(|node| {
        node.previous_node
            .map(|prev| nodes_map.get(&build_key(prev)).unwrap())
    });
    println!("P2 {:?}", prev_2);
    let prev_3 = prev_2.and_then(|node| {
        node.previous_node
            .map(|prev| nodes_map.get(&build_key(prev)).unwrap())
    });
    println!("P3 {:?}", prev_3);

    let forbidden_direction = match (prev_1, prev_2, prev_3) {
        (Some(p1), Some(p2), Some(p3)) => {
            let arrived_at_current_from = arrived_from(&current.coordinates, &p1.coordinates);
            let arrived_at_p1_from = arrived_from(&p1.coordinates, &p2.coordinates);
            let arrived_at_p2_from = arrived_from(&p2.coordinates, &p3.coordinates);

            if arrived_at_current_from == arrived_at_p1_from
                && arrived_at_p1_from == arrived_at_p2_from
            {
                Some(arrived_at_current_from)
            } else {
                None
            }
        }
        _ => None,
    };

    let mut neighbours: Vec<NodeAddress> = Vec::new();

    let is_up_forbidden = forbidden_direction
        .map(|d| d == Direction::Up)
        .unwrap_or(false);
    let is_right_forbidden = forbidden_direction
        .map(|d| d == Direction::Right)
        .unwrap_or(false);
    let is_down_forbidden = forbidden_direction
        .map(|d| d == Direction::Down)
        .unwrap_or(false);
    let is_left_forbidden = forbidden_direction
        .map(|d| d == Direction::Left)
        .unwrap_or(false);

    if !is_down_forbidden && row + 1 < *rows_number {
        neighbours.push((row + 1, column));
    }

    if !is_up_forbidden && row > 0 {
        neighbours.push((row - 1, column));
    }

    if !is_right_forbidden && column + 1 < *columns_number {
        neighbours.push((row, column + 1))
    }

    if !is_left_forbidden && column > 0 {
        neighbours.push((row, column - 1))
    }

    neighbours
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
