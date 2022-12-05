fn main() {
    println!("Hello Day 3!");

    let raw_data = std::fs::read_to_string("src/data/day3.txt").unwrap();

    let (count, result) = calculate_badge_priority(&raw_data);

    println!("Looked through {count} rucksacks.");
    println!("The total priority of the rucksacks is {result}");
}

fn calculate_total_priority(input: &str) -> (usize, i32) {
    let mut count = 0;
    let result: i32 = input
        .trim()
        .split("\n")
        .map(|s| {
            count += 1;

            let len = s.trim().len();
            let (left, right) = s.trim().split_at(len / 2);
            let shared_items = find_shared_items(left, right);

            let result = shared_items
                .iter()
                .map(|c| calculate_priority(*c))
                .sum::<i32>();

            result
        })
        .sum();

    (count, result)
}

fn calculate_badge_priority(input: &str) -> (usize, i32) {
    let mut count = 0;
    let result: i32 = input
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|group| {
            count += 3;

            let first = group[0];
            let second = group[1];
            let third = group[2];
            let badge_item = first
                .chars()
                .find(|c| second.contains(*c) && third.contains(*c))
                .unwrap();

            calculate_priority(badge_item)
        })
        .sum();

    (count, result)
}

fn calculate_priority(c: char) -> i32 {
    let mut lowercase = c;
    lowercase.make_ascii_lowercase();
    let value = lowercase as u8 - 'a' as u8;

    if c.is_uppercase() {
        value as i32 + 26 + 1
    } else {
        value as i32 + 1
    }
}

fn find_shared_items(left: &str, right: &str) -> Vec<char> {
    let mut result = left
        .chars()
        .filter(|c| right.contains(*c))
        .collect::<Vec<char>>();

    result.sort();
    result.dedup();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_example_priorities() {
        let data = [
            ('p', 16),
            ('L', 38),
            ('P', 42),
            ('v', 22),
            ('t', 20),
            ('s', 19),
        ];

        data.iter().for_each(|(c, expected)| {
            let actual = calculate_priority(*c);
            println!("Calculating priority for character {c}. Expecting {expected}. Got {actual}.");
            assert_eq!(actual, *expected);
        });
    }

    #[test]
    fn test_find_shared_items_example() {
        const LEFT: &str = "vJrwpWtwJgWr";
        const RIGHT: &str = "hcsFMMfFFhFp";

        let actual = find_shared_items(LEFT, RIGHT);
        let expected = ['p'].to_vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_find_shared_items_but_prune_duplicates() {
        const LEFT: &str = "jqHRNqRjqzjGDLGL";
        const RIGHT: &str = "rsFMfFZSrLrFZsSL";

        let actual = find_shared_items(LEFT, RIGHT);
        let expected = ['L'].to_vec();

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_data() {
        const EXAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let (count, result) = calculate_total_priority(EXAMPLE);

        const EXPECTED_COUNT: usize = 6;
        const EXPECTED_RESULT: i32 = 157;
        assert_eq!(count, EXPECTED_COUNT);
        assert_eq!(result, EXPECTED_RESULT);
    }
}
