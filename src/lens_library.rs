use std::collections::HashMap;

fn parse_input(file: &str) -> Vec<&str> {
    file.trim().split(",").collect::<Vec<&str>>()
}

fn calculate_hash(word: &str) -> u16 {
    let ascii_codes = word.chars().map(|c| c as u8).collect::<Vec<u8>>();
    let multiplier: u16 = 17;
    let divider: u16 = 256;

    ascii_codes.iter().fold(0 as u16, |acc, code| {
        let increased = acc + *code as u16;
        let multiplied = increased * multiplier;
        multiplied % divider
    })
}

pub fn calculate_hash_for_sequence(file: &str) -> usize {
    let sequence = parse_input(file);

    sequence.iter().fold(0 as usize, |acc, elem| {
        let hash = calculate_hash(elem);
        acc + hash as usize
    })
}

type Lens = (String, usize);

#[derive(Debug)]
enum Instruction {
    AddLens(String, usize),
    RemoveLens(String),
}

impl Instruction {
    pub fn parse(str: &str) -> Instruction {
        match str {
            add_lens_str if add_lens_str.contains("=") => {
                let mut split = add_lens_str.split("=");
                let label = split.next().expect("Label must be present on instruction");
                let focal_length = split
                    .next()
                    .map(|fl| fl.parse::<usize>().expect("Invalid focal length"))
                    .expect("Focal length must be present on instruction");
                Instruction::AddLens(label.to_string(), focal_length)
            }
            remove_lens_str if remove_lens_str.contains("-") => {
                let label = remove_lens_str
                    .split("-")
                    .nth(0)
                    .expect("Label must be present on instruction");
                Instruction::RemoveLens(label.to_string())
            }
            _ => panic!("Invalid instruction"),
        }
    }

    pub fn get_label(&self) -> &str {
        match self {
            Instruction::AddLens(label, _) => label.as_str(),
            Instruction::RemoveLens(label) => label.as_str(),
        }
    }

    pub fn get_hash(&self) -> u16 {
        let label = self.get_label();
        calculate_hash(label)
    }
}
fn parse_instructions(file: &str) -> Vec<Instruction> {
    parse_input(file)
        .into_iter()
        .map(Instruction::parse)
        .collect::<Vec<Instruction>>()
}

struct Box {
    lenses: Vec<Lens>,
    lenses_index: HashMap<String, usize>,
}

impl Box {
    pub fn add_lens(&mut self, lens: &Lens) -> () {
        let maybe_lens_in_box = self.lenses_index.get(&lens.0);

        match maybe_lens_in_box {
            None => {
                self.lenses.push(lens.clone());
                self.lenses_index.insert(lens.0.clone(), self.lenses.len() - 1);
            }
            Some(lens_index) => {
                self.lenses[*lens_index] = lens.clone();
            }
        }
    }

    pub fn remove_lens(&mut self, label: &String) -> () {
        let maybe_lens_in_box = self.lenses_index.get(label);

        match maybe_lens_in_box {
            None => (),
            Some(lens_index) => {
                self.lenses.remove(*lens_index);
                self.lenses_index.remove(label);

                for (i, lens) in self.lenses.iter().enumerate() {
                    self.lenses_index.insert(lens.0.clone(), i);
                }
            }
        }
    }

    pub fn empty() -> Box {
        Box {
            lenses_index: HashMap::new(),
            lenses: Vec::new(),
        }
    }
}

pub fn put_lenses_in_boxes(file: &str) -> usize {
    let instructions = parse_instructions(file);

    let mut boxes: Vec<Box> = (0..256).map(|_| Box::empty()).collect::<Vec<Box>>();

    for instruction in instructions.iter() {
        match instruction {
            add_lens_instruction @ Instruction::AddLens(label, focal_length) => {
                let hash = add_lens_instruction.get_hash();
                let box_to_update = boxes.get_mut(hash as usize).expect("Box must exist");
                box_to_update.add_lens(&(label.clone(), *focal_length));
            }
            remove_lens_instruction @ Instruction::RemoveLens(label) => {
                let hash = remove_lens_instruction.get_hash();

                let box_to_update = boxes.get_mut(hash as usize).expect("Box must exist");
                box_to_update.remove_lens(label);
            }
        }

        println!("After {:?}", instruction);
        let non_empty_boxes = boxes.iter().enumerate().filter(|(_, b)| !b.lenses_index.is_empty()).collect::<Vec<(usize, &Box)>>();
        for (i, b) in non_empty_boxes.iter() {
          println!("Box {}: {:?}", i, b.lenses);
        } 
        println!("\n");
    }

    boxes.iter().enumerate().fold(0, |acc, (i, b)| {
      let box_number_multiplier = i + 1;
        let sum_lenses = b.lenses.iter().enumerate().fold(0, |acc_lenses, (lens_index, lens)| {
            let lens_strength = box_number_multiplier * (lens_index + 1) * lens.1;
            acc_lenses + lens_strength
        });
        acc + sum_lenses
    })
}
