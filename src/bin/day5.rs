fn main() {
    println!("Hello Day 5!");

    let data = std::fs::read_to_string("src/data/day5.txt").unwrap();

    let (crate_text, instruction_text) = split_crates_and_instructions(&data);

    let initial = extract_initial_crates(crate_text);

    println!("Starting with crates: {initial:?}");

    let instructions = parse_instructions(instruction_text);

    println!("Following instructions: {instructions:?}");

    let final_crates = apply_instructions(initial, instructions).unwrap();

    println!("The final crats are: {final_crates:?}");

    let firsts: Vec<char> = final_crates.iter().map(|v| *v.last().unwrap()).collect();

    println!("The top of each column: {firsts:?}");
}

fn split_crates_and_instructions(input: &str) -> (&str, &str) {
    input
        .split_once("\n\n")
        .expect("Could not find empty line to split at.")
}

fn extract_initial_crates(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect();
    let width: usize = lines
        .last()
        .unwrap()
        .rmatches(char::is_numeric)
        .next()
        .unwrap()
        .parse()
        .unwrap();

    println!("Crates has a width of {width}");

    let mut result = Vec::new();
    result.resize(width, Vec::new());

    lines.iter().rev().skip(1).for_each(|s| {
        (0..width)
            .map(|i| 1 + (i * 4))
            .enumerate()
            .for_each(|(column, i)| {
                if let Some(s) = s.get(i..i + 1) {
                    let c = s.chars().next().unwrap();
                    if c.is_alphabetic() {
                        result[column].push(c);
                    }
                }
            })
    });

    result
}

#[derive(Debug, PartialEq)]
struct Instruction {
    quantity: usize,
    from: usize,
    to: usize,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            println!("Parsing instruction: {line}");
            parse_instruction(line)
        })
        .collect()
}

fn parse_instruction(line: &str) -> Instruction {
    let nums: Vec<usize> = line
        .chars()
        .filter(|c| !c.is_alphabetic())
        .collect::<String>()
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    Instruction {
        quantity: nums[0],
        from: nums[1] - 1,
        to: nums[2] - 1,
    }
}

fn apply_instructions(
    initial_crates: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
) -> Result<Vec<Vec<char>>, String> {
    let mut result = initial_crates.clone();
    instructions.iter().for_each(|instruction| {
        let from_column = instruction.from;
        let to_column = instruction.to;

        let from_idx = result[from_column].len() - instruction.quantity;
        let mut slice = result[from_column].split_off(from_idx);

        result[to_column].append(&mut slice);
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    fn example_initial_crates() -> Vec<Vec<char>> {
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
    }

    fn example_final_crates() -> Vec<Vec<char>> {
        vec![vec!['M'], vec!['C'], vec!['P', 'Z', 'N', 'D']]
    }

    fn example_instructions() -> Vec<Instruction> {
        vec![
            Instruction {
                quantity: 1,
                from: 1,
                to: 0,
            },
            Instruction {
                quantity: 3,
                from: 0,
                to: 2,
            },
            Instruction {
                quantity: 2,
                from: 1,
                to: 0,
            },
            Instruction {
                quantity: 1,
                from: 0,
                to: 1,
            },
        ]
    }

    #[test]
    fn test_extract_example_initial_crates() {
        let expected: Vec<Vec<char>> = example_initial_crates();

        let (crates, _) = split_crates_and_instructions(EXAMPLE_DATA);
        let actual = extract_initial_crates(crates);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_extract_example_instructions() {
        let expected = example_instructions();
        let (_, instructions) = split_crates_and_instructions(EXAMPLE_DATA);

        let actual = parse_instructions(instructions);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_apply_example_instructions() {
        let expected = example_final_crates();
        let actual = apply_instructions(example_initial_crates(), example_instructions()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_can_parse_multidigit_columns_in_instructions() {
        let line = "move 10 from 6 to 9";
        let actual = parse_instruction(line);
        let expected = Instruction {
            quantity: 10,
            from: 5,
            to: 8,
        };

        assert_eq!(actual, expected);
    }
}
