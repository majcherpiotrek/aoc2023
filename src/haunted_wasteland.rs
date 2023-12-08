use std::collections::HashMap;

#[derive(Debug)]
struct NodePair(String, String);

impl NodePair {
    pub fn left(&self) -> &str {
        &self.0
    }

    pub fn right(&self) -> &str {
        &self.1
    }

    pub fn parse(str: &str) -> Option<NodePair> {
        let cleaned = str.replace(&['(', ')'][..], "");
        let mut split = cleaned.split(", ");
        match (split.next(), split.next()) {
            (Some(l), Some(r)) => Some(NodePair(l.to_string(), r.to_string())),
            _ => None,
        }
    }
}

pub fn find_way(file: &str) -> usize {
    let mut file_split = file.split("\n\n");
    let instructions: Vec<char> = file_split.next().map(|line| line.chars().collect()).unwrap_or(Vec::new());
    let mut nodes_map: HashMap<String, NodePair> = HashMap::new();
   
    
    match file_split.next() {
    
      Some(map_block) => {
    

        for line in map_block.split("\n") {
            let mut line_split = line.split(" = ");
            let maybe_node = line_split.next();
            let maybe_node_pair = line_split.next().and_then(NodePair::parse);

            match (maybe_node, maybe_node_pair) {
                    (Some(node), Some(node_pair)) => {
                    nodes_map.insert(node.to_string(), node_pair);
                    },
                        _ => ()
            }
        }
        },
            None => ()
    }

    println!("Instructions: {:?}", instructions);
    println!("Map: {:?}", nodes_map);
    
    const START_NODE: &str = "AAA";
    const END_NODE: &str = "ZZZ";
    let mut current_node: &str = START_NODE;
    let mut steps: usize = 0;
    
    loop {
        let index = steps % instructions.len();  
        steps = steps + 1;
        println!("\nStep {steps}");
        println!("Instruction index: {index}");
        let direction = instructions.get(index).expect("This should never happen");
        println!("Instruction: {direction}");
        let current_node_pair = nodes_map.get(current_node).expect("This should never happen"); 
        println!("Current node pair {:?}", current_node_pair);
         let next_node = match direction {
            'L' => {
              current_node_pair.left()
            },
                'R' => {
                current_node_pair.right()
            },
                _ => panic!("Incorrect direction")
        };

        println!("Next node: {next_node}");

        if *next_node == *END_NODE {
            break;
        } else {
            current_node = next_node;
        }
    }
    
   steps 
}
