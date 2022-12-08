const THRESHOLD: usize = 100000;

fn main() {
    println!("Hello Day 7!");

    let input = std::fs::read_to_string("src/data/day7.txt").unwrap();
    let lines = split_input_to_lines(&input);
    let result = total_dirs_under_threshold(&lines, THRESHOLD);

    println!("Final Result: {result}");
}

fn split_input_to_lines(input: &str) -> Vec<&str> {
    input.trim().split("\n").collect()
}

struct DirSizeInfo {
    running_total_size: usize,
    children_under_threshold_total_size: usize,
}

impl DirSizeInfo {
    fn zero() -> Self {
        Self {
            running_total_size: 0,
            children_under_threshold_total_size: 0,
        }
    }
}

fn total_dirs_under_threshold(lines: &Vec<&str>, threshold: usize) -> usize {
    let final_result = recursive_size_and_total_under_threshold(lines, threshold, 1);

    final_result.children_under_threshold_total_size
}

fn recursive_size_and_total_under_threshold(
    lines: &Vec<&str>,
    threshold: usize,
    start_idx: usize,
) -> DirSizeInfo {
    println!(
        "Starting check for dir beginning at {start_idx} -- {}",
        lines[start_idx]
    );
    let mut result = DirSizeInfo::zero();
    let mut nesting_level = 1;

    for idx in start_idx..lines.len() {
        let line = lines[idx].trim();

        if line == "$ cd .." {
            nesting_level -= 1;

            if nesting_level == 0 {
                break;
            } else {
                continue;
            }
        }

        if line.chars().next().unwrap().is_numeric() {
            if nesting_level == 1 {
                let size = line
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
                result.running_total_size += size;
            }
        } else if line.starts_with("$ cd") {
            if nesting_level == 1 {
                println!("Entering dir: {line}");
                let child_dir = recursive_size_and_total_under_threshold(lines, threshold, idx + 1);
                result.running_total_size += child_dir.running_total_size;
                result.children_under_threshold_total_size +=
                    child_dir.children_under_threshold_total_size;
            }

            nesting_level += 1;
        }
    }

    println!(
        "Size of dir starting at line {start_idx} is {}",
        result.running_total_size
    );
    if result.running_total_size <= threshold {
        result.children_under_threshold_total_size += result.running_total_size;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    const SINGLE_DIR_EXAMPLE: &str = "$ cd /
$ ls
200 b.txt
400 c.dat
";

    #[test]
    fn test_single_dir() {
        let lines = split_input_to_lines(SINGLE_DIR_EXAMPLE);
        let actual = total_dirs_under_threshold(&lines, THRESHOLD);
        let expected = 600;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example() {
        let lines = split_input_to_lines(EXAMPLE_INPUT);
        let total = total_dirs_under_threshold(&lines, 100000);
        let expected = 95437;

        assert_eq!(total, expected);
    }
}
