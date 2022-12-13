fn main() {
    println!("Hello day 10!");

    let input = std::fs::read_to_string("src/data/day10.txt").unwrap();
    let total = strengths_at_interval(&input);

    println!("The final strength was {total}");

    draw_to_crt(&input);
}

enum Instruction {
    NoOp,
    AddX(i32),
}

fn get_instructions_from_string(input: &str) -> Vec<Instruction> {
    input
        .trim()
        .split('\n')
        .map(|line| {
            if line.starts_with("noop") {
                Instruction::NoOp
            } else {
                let num = line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                Instruction::AddX(num)
            }
        })
        .collect()
}

fn callback_in_cycles<F>(instructions: Vec<Instruction>, f: F)
where
    F: Fn(i32, i32),
{
    let mut cycle = 0;
    let mut register = 1;
    instructions
        .iter()
        .for_each(|instruction| match *instruction {
            Instruction::NoOp => {
                cycle += 1;

                f(cycle, register);
            }
            Instruction::AddX(amount) => {
                cycle += 1;
                f(cycle, register);

                cycle += 1;
                f(cycle, register);

                register += amount;
            }
        });
}

fn strengths_at_interval(input: &str) -> i32 {
    let mut result = 0;
    let instructions = get_instructions_from_string(input);

    let mut cycle = 0;
    let mut register = 1;
    instructions
        .iter()
        .for_each(|instruction| match *instruction {
            Instruction::NoOp => {
                cycle += 1;

                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    let strength = cycle * register;
                    println!("NOOP Instruction");
                    println!("Strength at cycle {cycle} is {strength}");
                    result += strength;
                }
            }
            Instruction::AddX(amount) => {
                cycle += 1;

                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    let strength = cycle * register;
                    println!("Mid Addx with value {amount}");
                    println!("Strength at cycle {cycle} is {strength}");
                    result += strength;
                }

                cycle += 1;

                if cycle == 20 || (cycle - 20) % 40 == 0 {
                    let strength = cycle * register;
                    println!("After AddX with value {amount}");
                    println!("Strength at cycle {cycle} is {strength}");
                    result += strength;
                }

                register += amount;
            }
        });

    result
}

fn draw_to_crt(input: &str) {
    let instructions = get_instructions_from_string(input);
    callback_in_cycles(instructions, |cycle, register| {
        let line_pos = (cycle - 1) % 40;
        let is_on = (line_pos - register).abs() <= 1;

        if line_pos == 0 {
            print!("\n");
        }

        if is_on {
            print!("#");
        } else {
            print!(".");
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = std::fs::read_to_string("src/data/day10_example.txt").unwrap();
        let actual = strengths_at_interval(&input);
        let expected = 13140;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_alternative() {
        let input = std::fs::read_to_string("src/data/day10_alt.txt").unwrap();
        let actual = strengths_at_interval(&input);
        let expected = 81 * 20;

        assert_eq!(actual, expected);
    }
}
