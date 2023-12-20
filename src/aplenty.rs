use std::collections::HashMap;

pub fn sort_parts(file: &str) -> usize {
    let mut file_split = file.split("\n\n");

    let mut workflows_map: HashMap<String, Workflow> = HashMap::new();

    let worflows_block = file_split.next().unwrap();

    for line in worflows_block.split("\n") {
        if line.is_empty() {
            continue;
        }

        let workflow = Workflow::parse(line);

        workflows_map.insert(workflow.name.clone(), workflow);
    }

    let parts = file_split
        .next()
        .map(|parts_block| {
            parts_block
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(Part::parse)
                .collect::<Vec<Part>>()
        })
        .unwrap();

    let mut accepted_parts: Vec<_> = Vec::new();

    let first_workflow = workflows_map.get("in").unwrap();

    for part in parts.iter() {
        let mut current_workflow = first_workflow;
        let mut workflow_result = current_workflow.process_part(part);

        while let WorkflowResult::AnotherWorkflow(next_workflow_name) = workflow_result {
            current_workflow = workflows_map.get(&next_workflow_name).unwrap();
            workflow_result = current_workflow.process_part(part);
        }

        if workflow_result == WorkflowResult::Accept {
            accepted_parts.push(part);
        }
    }

    accepted_parts
        .iter()
        .fold(0, |acc, part| acc + part.get_properties_sum())
}

pub fn calculate_all_rating_combinations(file: &str) -> usize {
    let mut file_split = file.split("\n\n");

    let mut workflows_map: HashMap<String, Workflow> = HashMap::new();

    let worflows_block = file_split.next().unwrap();

    for line in worflows_block.split("\n") {
        if line.is_empty() {
            continue;
        }

        let workflow = Workflow::parse(line);

        workflows_map.insert(workflow.name.clone(), workflow);
    }
    0
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Workflow {
    name: String,
    conditions: Vec<Condition>,
    default_result: WorkflowResult,
}

impl Workflow {
    pub fn parse(line: &str) -> Workflow {
        let mut split = line.split("{");
        let name = split.next().unwrap();
        split
            .next()
            .map(|str| {
                let parts = &str[..str.len() - 1].split(",").collect::<Vec<&str>>();
                let conditions = parts[..parts.len() - 1]
                    .iter()
                    .map(|cond| Condition::parse(cond))
                    .collect::<Vec<Condition>>();
                let default_result = WorkflowResult::parse(parts.last().unwrap());

                Workflow {
                    name: name.to_string(),
                    conditions,
                    default_result,
                }
            })
            .unwrap()
    }

    pub fn process_part(&self, part: &Part) -> WorkflowResult {
        let mut result = self.default_result.clone();
        for condition in self.conditions.iter() {
            if let Some(condition_result) = condition.verify(part) {
                result = condition_result.clone();
                break;
            }
        }

        result
    }

    //pub fn get_possibilities(&self) -> Vec<_> {
    //    let mut possibilities: Vec<_> = Vec::new();
    //    let mut current_possibilities = 4000;
    //    for cond in self.conditions.iter() {
    //       let condition_pass_possibilities = cond.get_condition_pass_posibilities();
    //       
    //       possibilities.push((cond.on_fulfilled, condition_pass_possibilities));
    //       if current_possibilities = current_possibilities - condition_pass_possibilities;
    //    }
    //}
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Condition {
    property: String,
    comparator: Comparator,
    value_to_compare: usize,
    on_fulfilled: WorkflowResult,
}

impl Condition {
    pub fn parse(str: &str) -> Condition {
        let mut split = str.split(":");

        let (property, comparator, value_to_compare) = split
            .next()
            .map(|cond| {
                let property = &cond[0..1];
                let comparator = Comparator::parse(&cond[1..2]);
                let value_to_compare = *(&cond[2..].parse::<usize>().unwrap());

                (property, comparator, value_to_compare)
            })
            .unwrap();

        let on_fulfilled = split.next().map(WorkflowResult::parse).unwrap();

        Condition {
            property: property.to_string(),
            comparator,
            value_to_compare,
            on_fulfilled,
        }
    }

    pub fn verify(&self, part: &Part) -> Option<WorkflowResult> {
        let value = part.get_property(&self.property);

        let is_fullfilled = match self.comparator {
            Comparator::GreaterThan => value > self.value_to_compare,
            Comparator::LesserThan => value < self.value_to_compare,
        };

        if is_fullfilled {
            Some(self.on_fulfilled.clone())
        } else {
            None
        }
    }

    pub fn get_condition_pass_posibilities(&self) -> usize {
        match self.comparator {
            Comparator::GreaterThan => 4000 - self.value_to_compare,
            Comparator::LesserThan => self.value_to_compare - 1,
        }
    }

    pub fn get_condition_fail_posibilities(&self) -> usize {
        4000 - self.get_condition_pass_posibilities()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Comparator {
    GreaterThan,
    LesserThan,
}

impl Comparator {
    pub fn parse(str: &str) -> Comparator {
        match str {
            ">" => Comparator::GreaterThan,
            "<" => Comparator::LesserThan,
            _ => panic!("Invalid operator"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum WorkflowResult {
    AnotherWorkflow(String),
    Accept,
    Reject,
}

impl WorkflowResult {
    pub fn parse(str: &str) -> WorkflowResult {
        match str {
            "A" => WorkflowResult::Accept,
            "R" => WorkflowResult::Reject,
            workflow_name => WorkflowResult::AnotherWorkflow(workflow_name.to_string()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    pub fn parse(line: &str) -> Part {
        let mut properties: HashMap<_, _> = HashMap::new();

        for part in line[1..line.len() - 1].split(",") {
            let mut split = part.split("=");
            let property_name = split.next().unwrap().to_string();
            let property_value = split.next().and_then(|v| v.parse::<usize>().ok()).unwrap();

            properties.insert(property_name, property_value);
        }

        let x = *properties.get("x").unwrap();
        let m = *properties.get("m").unwrap();
        let a = *properties.get("a").unwrap();
        let s = *properties.get("s").unwrap();

        Part { x, m, a, s }
    }

    pub fn get_property(&self, prop: &str) -> usize {
        match prop {
            "x" => self.x,
            "m" => self.m,
            "a" => self.a,
            "s" => self.s,
            _ => panic!("Invalid part prop!"),
        }
    }

    pub fn get_properties_sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}
