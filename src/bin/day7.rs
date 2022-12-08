const THRESHOLD: usize = 100000;

fn main() {
    println!("Hello Day 7!");

    let input = std::fs::read_to_string("src/data/day7.txt").unwrap();
    let lines = split_input_to_lines(&input);
    let mut get_flat_dirs = flat_dir_sizes_recursive(&lines, 1);
    let total_used_space = get_flat_dirs.last().unwrap();
    let remaining_space = 70000000 - total_used_space;
    let needed_space = 30000000 - remaining_space;

    println!("Total used space: {total_used_space}");
    println!("Remaining Space on Disk: {remaining_space}");
    println!("I need at least: {needed_space} to be freed");

    println!("{get_flat_dirs:?}");
    for size in get_flat_dirs {
        if size >= needed_space {
            println!("{size}");
            return;
        }
    }
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

fn flat_dir_sizes_recursive(lines: &Vec<&str>, start_idx: usize) -> Vec<usize> {
    let mut result = Vec::new();
    let mut nesting_level = 1;
    let mut my_size: usize = 0;

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
                my_size += size;
            }
        } else if line.starts_with("$ cd") {
            if nesting_level == 1 {
                let mut child_dir = flat_dir_sizes_recursive(lines, idx + 1);

                my_size += child_dir.last().unwrap();

                result.append(&mut child_dir);
            }

            nesting_level += 1;
        }
    }

    result.push(my_size);
    result.sort();
    result
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

    const NESTING_DIRS_EXAMPLE: &str = "$ cd /
$ ls
dir h
200 b.txt
400 c.dat
$ cd dir h
$ ls
500 r.dangit
1000 b.dangit
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

    #[test]
    fn test_flat_dir_single() {
        let lines = split_input_to_lines(SINGLE_DIR_EXAMPLE);
        let flat_single_dir = flat_dir_sizes_recursive(&lines, 1);
        let expected = vec![600];

        assert_eq!(flat_single_dir, expected);
    }

    #[test]
    fn test_flat_dir_example() {
        let lines = split_input_to_lines(EXAMPLE_INPUT);
        let flat_example = flat_dir_sizes_recursive(&lines, 1);

        let expected = vec![584, 94853, 24933642, 48381165];

        assert_eq!(flat_example, expected);
    }

    #[test]
    fn test_flat_dir_nesting() {
        let lines = split_input_to_lines(NESTING_DIRS_EXAMPLE);
        let actual = flat_dir_sizes_recursive(&lines, 1);

        let expected = vec![1500, 2100];

        assert_eq!(actual, expected);
    }
}
