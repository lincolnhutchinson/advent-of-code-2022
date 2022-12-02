use std::fs;

pub fn solve() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let raw_data = fs::read_to_string("src/day1/data.txt")?;

    let result = find_top_calorie_elves(&raw_data, 1);
    println!("The highest calorie elf has: {result} calories.");
    let result_two = find_top_calorie_elves(&raw_data, 3);
    println!("The top three elves together have: {result_two} calories");

    Ok(())
}

fn find_top_calorie_elves(input_string: &str, number: usize) -> u32 {
    let split_by_elf: Vec<&str> = input_string.split("\n\n").collect();

    let mut data: Vec<u32> = split_by_elf
        .iter()
        .map(|elf_s| {
            elf_s
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    data.sort();

    data.as_slice()
        .rchunks(number)
        .next()
        .unwrap()
        .iter()
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let data = "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            ";

        const EXPECTED: u32 = 24000;
        let result = find_top_calorie_elves(data, 1);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn test_gets_different_answers() {
        let data = "
            1000
            2000
            3000

            4000

            5000
            6000

            10000
            20000
            30000

            7000
            8000
            9000

            10000
            ";

        const EXPECTED: u32 = 60000;
        let result = find_top_calorie_elves(data, 1);
        assert_eq!(result, EXPECTED);
    }

    #[test]
    fn test_getting_top_three() {
        let data = "
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
            ";

        const EXPECTED: u32 = 45000;
        let result = find_top_calorie_elves(data, 3);
        assert_eq!(result, EXPECTED);
    }
}
