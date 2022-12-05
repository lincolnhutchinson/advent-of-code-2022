fn main() {
    println!("Hello Day 4!");

    let input = std::fs::read_to_string("src/data/day4.txt").unwrap();
    let result = count_containments(&input);

    println!("There were {result} partial containments in the given dataset.");
}

fn count_containments(input: &str) -> usize {
    input
        .trim()
        .split('\n')
        .map(|s| {
            s.split(',')
                .map(|s| {
                    s.split('-')
                        .map(|s| s.trim().parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .filter(|s| {
            let (start_1, end_1) = (s[0][0], s[0][1]);
            let (start_2, end_2) = (s[1][0], s[1][1]);

            check_partial_containment(start_1, end_1, start_2, end_2)
        })
        .count()
}

fn check_partial_containment(start_1: usize, end_1: usize, start_2: usize, end_2: usize) -> bool {
    (start_1 >= start_2 && start_1 <= end_2)
        || (end_1 >= start_2 && end_1 <= end_2)
        || (start_1 <= start_2 && end_1 >= end_2)
        || (start_2 <= start_1 && end_2 >= end_1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_containment() {
        assert!(check_partial_containment(2, 8, 3, 7));
        assert!(check_partial_containment(3, 7, 2, 8));
    }

    #[test]
    fn test_partial_containment() {
        assert!(check_partial_containment(2, 5, 3, 7));
        assert!(check_partial_containment(3, 7, 6, 8));
    }

    #[test]
    fn test_no_containment() {
        assert!(!check_partial_containment(2, 8, 10, 14));
        assert!(!check_partial_containment(3, 7, 0, 2));
    }

    #[test]
    fn test_example_data() {
        const DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        let actual = count_containments(DATA);
        const EXPECTED: usize = 4;

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn test_different_data() {
        const DATA: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
4-4, 2-7
6-6,4-6
2-6,4-8";

        let actual = count_containments(DATA);
        const EXPECTED: usize = 5;

        assert_eq!(actual, EXPECTED);
    }
}
