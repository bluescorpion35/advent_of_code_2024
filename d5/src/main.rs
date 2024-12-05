// What a disaster
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
fn main() {
    let file = std::fs::read_to_string("input2").unwrap();
    let parts = file.split("\n\n").collect::<Vec<&str>>();
    let rules_raw = parts[0].split("\n").collect::<Vec<&str>>();
    let mut instructions_raw = parts[1].split("\n").collect::<Vec<&str>>();
    let rules: Vec<Rule> = Rule::convert(rules_raw);
    for rule in &rules {
        println!("{rule:?}")
    }
    instructions_raw.truncate(instructions_raw.len() - 1);

    let mut instructions: Vec<Vec<i32>> = Vec::new();

    for instruction in instructions_raw {
        println!("{instruction:?}");
        let temp = instruction
            .split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        instructions.push(temp);
    }
    println!("{:?}", instructions);
    let mut insts_struct: Vec<Instruction> = Vec::new();
    for instruction in instructions {
        insts_struct.push(Instruction::create(instruction, &rules));
    }

    let mut total = 0;
    for mut instruction in insts_struct {
        println!("{instruction:?}");
        if instruction.is_sorted {
        } else {
            total += instruction.middle_page_unsorted(&rules);
        }
    }
    println!("total: {}", total);
}

impl Rule {
    pub fn convert(rules_raw: Vec<&str>) -> Vec<Rule> {
        let mut output: Vec<Rule> = Vec::new();
        for i in rules_raw {
            let parts = i.split("|").collect::<Vec<&str>>();
            let part1 = parts[0].parse::<i32>().unwrap();
            let part2 = parts[1].parse::<i32>().unwrap();
            output.push(Rule {
                i1: part1,
                i2: part2,
            });
        }
        output
    }
}

struct Rule {
    i1: i32,
    i2: i32,
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}|{}", self.i1, self.i2)
    }
}

#[derive(Debug)]
struct Instruction {
    instructions: Vec<i32>,
    is_sorted: bool,
}

impl Instruction {
    pub fn create(instructions: Vec<i32>, rules: &Vec<Rule>) -> Instruction {
        let mut sorted = true;
        println!("CREATED");
        let x = instructions.iter().position(|i| *i == -1);
        for i in rules {
            if !instructions.contains(&i.i1) || !instructions.contains(&i.i2) {
                sorted = true;
            } else if Some(instructions.iter().position(|&x| x == i.i1))
                < Some(instructions.iter().position(|&x| x == i.i2))
            {
                sorted = true;
                println!("true");
            } else {
                sorted = false;
                println!("false");
                break;
            }
        }
        Instruction {
            instructions: instructions,
            is_sorted: sorted,
        }
    }
    pub fn middle_page(&self) -> i32 {
        println!("test");
        self.instructions[self.instructions.len() / 2]
    }

    pub fn middle_page_unsorted(&self, rules: &Vec<Rule>) -> i32 {
        for i in &self.instructions {
            let pages: (i32, i32) = self.count_pages(*i, rules);
            if pages.0 == pages.1 {
                return *i;
            }
        }

        panic!("Object not found\nThe requested URL was not found on this server. If you entered the URL manually please check your spelling and try again. ")
    }

    pub fn count_pages(&self, instruction: i32, rules: &Vec<Rule>) -> (i32, i32) {
        let mut before: Vec<i32> = Vec::new();
        let mut after: Vec<i32> = Vec::new();
        let mut deps: Vec<i32> = vec![instruction];
        for rule in rules {
            if deps.contains(&rule.i1) && !deps.contains(&rule.i2) {
                after.push(rule.i2)
            } else if deps.contains(&rule.i2) && !deps.contains(&rule.i1) {
                before.push(rule.i1)
            }
            deps.append(&mut before.clone());
            deps.append(&mut after.clone());


        }
        println!("deps: {:?}", deps);
        println!("before: {:?}", before);
        println!("after: {:?}", after);
        (before.len() as i32, after.len() as i32)
    }

    pub fn is_sorted(&self, rules: &Vec<Rule>) -> bool {
        let mut sorted = true;
        println!("CREATED");
        let x = self.instructions.iter().position(|i| *i == -1);
        for i in rules {
            if !self.instructions.contains(&i.i1) || !self.instructions.contains(&i.i2) {
                sorted = true;
            } else if Some(self.instructions.iter().position(|&x| x == i.i1))
                < Some(self.instructions.iter().position(|&x| x == i.i2))
            {
                sorted = true;
                println!("true");
            } else {
                sorted = false;
                println!("false");
                break;
            }
        }
        return sorted;
    }
}

#[cfg(test)]
mod tests {
    use crate::Instruction;
    use crate::Rule;

    #[test]
    fn test_middle_page() {
        let instruction =
            Instruction::create(vec![0, 1, 2, 3, 4, 5, 6], &vec![Rule { i1: 1, i2: 2 }]);
        assert_eq!(3, instruction.middle_page());
    }

    // Helper function to create an instruction with a given set of instructions and rules
    fn create_instruction(instructions: Vec<i32>, rules: &Vec<Rule>) -> Instruction {
        Instruction::create(instructions, &rules)
    }

    // Test case for a sorted instruction list
    #[test]
    fn test_sorted_instructions() {
        let rules = vec![Rule { i1: 1, i2: 2 }, Rule { i1: 2, i2: 3 }];
        let instruction = create_instruction(vec![1, 2, 3], &rules);
        assert!(instruction.is_sorted(&rules));
    }

    // Test case for an unsorted instruction list
    #[test]
    fn test_unsorted_instructions() {
        let rules = vec![Rule { i1: 1, i2: 2 }, Rule { i1: 2, i2: 3 }];
        let instruction = create_instruction(vec![2, 1, 3], &rules);
        assert!(!instruction.is_sorted(&rules));
    }

    // Test case when a rule's elements are not in the instruction list
    #[test]
    fn test_missing_elements_in_instructions() {
        let rules = vec![Rule { i1: 1, i2: 2 }, Rule { i1: 2, i2: 3 }];
        let instruction = create_instruction(vec![4, 5], &rules);
        // Since the rules reference 1 and 2, which are not in the list, it should return true
        assert!(instruction.is_sorted(&rules));
    }

    // Test case for multiple rules
    #[test]
    fn test_multiple_rules() {
        let rules = vec![
            Rule { i1: 1, i2: 2 },
            Rule { i1: 2, i2: 3 },
            Rule { i1: 4, i2: 5 },
        ];
        let instruction = create_instruction(vec![1, 2, 3, 4, 5], &rules);
        assert!(instruction.is_sorted(&rules));
    }

    // Test case for a complex unsorted list
    #[test]
    fn test_complex_unsorted_instructions() {
        let rules = vec![
            Rule { i1: 1, i2: 2 },
            Rule { i1: 2, i2: 3 },
            Rule { i1: 3, i2: 4 },
        ];
        let instruction = create_instruction(vec![4, 3, 2, 1], &rules);
        assert!(!instruction.is_sorted(&rules));
    }

    // Test case for an empty list of instructions
    #[test]
    fn test_empty_instruction_list() {
        let rules = vec![Rule { i1: 1, i2: 2 }, Rule { i1: 2, i2: 3 }];
        let instruction = create_instruction(vec![], &rules);
        // An empty list should be considered sorted as no rules can be violated
        assert!(instruction.is_sorted(&rules));
    }
}
