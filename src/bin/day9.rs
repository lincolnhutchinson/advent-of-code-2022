fn main() {
    println!("Hello Day 9!");

    let input = std::fs::read_to_string("src/data/day9.txt").unwrap();
    let result = count_multi_joint_tail_positions_visited(&input, 10);
    println!("Visited {result} locations!");
}

fn count_distinct_locations_visited(input: &str) -> usize {
    let head_positions = build_head_position_list(input);
    let mut visited_coords = build_tail_pos_from_head(&head_positions);

    visited_coords.sort();
    visited_coords.dedup();

    visited_coords.len()
}

fn build_head_position_list(input: &str) -> Vec<(i32, i32)> {
    let mut head_pos = (0, 0);
    let mut visited_coords = vec![(0, 0)];

    input.trim().split("\n").for_each(|line| {
        let mut split = line.split_whitespace();
        let dir = split.next().unwrap();
        let count = split.last().unwrap().parse::<i32>().unwrap();

        let delta = match dir {
            "U" => (0, -1),
            "D" => (0, 1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!(),
        };

        (0..count).for_each(|_| {
            head_pos.0 += delta.0;
            head_pos.1 += delta.1;
            visited_coords.push(head_pos);
        });
    });

    visited_coords
}

fn build_tail_pos_from_head(head: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut tail_pos = (0, 0);
    let mut visited_coords = vec![];

    for head_pos in head {
        let x_diff: i32 = head_pos.0 - tail_pos.0;
        let y_diff: i32 = head_pos.1 - tail_pos.1;
        if x_diff.abs() > 1 {
            let dir = x_diff.signum();
            tail_pos.0 += dir;

            let y_dir = y_diff.signum();
            tail_pos.1 += y_dir;
        } else if y_diff.abs() > 1 {
            let dir = y_diff.signum();
            tail_pos.1 += dir;

            let x_dir = x_diff.signum();
            tail_pos.0 += x_dir;
        }

        visited_coords.push(tail_pos);
    }

    visited_coords
}

fn count_multi_joint_tail_positions_visited(input: &str, tail_segments: usize) -> usize {
    let head_positions = build_head_position_list(input);

    let mut furthest_tail = head_positions.clone();
    (1..tail_segments).for_each(|i| {
        println!("Building tail {i}");
        let new_tail = build_tail_pos_from_head(&furthest_tail);
        furthest_tail = new_tail;
    });

    furthest_tail.sort();
    println!("Length before dedup: {}", furthest_tail.len());
    furthest_tail.dedup();
    furthest_tail.len()
}

fn visualize_multi_jointed_tail(input: &str, tail_segments: usize) {
    let head_positions = build_head_position_list(input);
    let mut vis_positions = vec![head_positions.clone()];

    let mut furthest_tail = head_positions.clone();
    (1..tail_segments).for_each(|i| {
        let new_tail = build_tail_pos_from_head(&furthest_tail);
        furthest_tail = new_tail;
        vis_positions.push(furthest_tail.clone());
    });

    let mut lowest_y = 9999;
    let mut highest_y = -9999;
    let mut lowest_x = 9999;
    let mut highest_x = -9999;

    head_positions.iter().for_each(|(x, y)| {
        lowest_x = lowest_x.min(*x);
        highest_x = highest_x.max(*x);

        lowest_y = lowest_y.min(*y);
        highest_y = highest_y.max(*y);
    });

    let height = (highest_y - lowest_y + 1) * 2;
    let width = (highest_x - lowest_x + 1) * 2;

    (0..head_positions.len()).for_each(|idx| {
        let mut row = Vec::new();
        row.resize(width as usize, '.');

        let mut grid = Vec::new();
        grid.resize(height as usize, row);

        vis_positions
            .iter()
            .enumerate()
            .rev()
            .for_each(|(segment_idx, points)| {
                let c = if segment_idx == 0 {
                    'H'
                } else {
                    char::from_digit(segment_idx as u32, 10).unwrap()
                };

                let pos = points[idx];
                let upos = ((pos.0 + width / 2) as usize, (pos.1 + height / 2) as usize);

                grid[upos.1][upos.0] = c;
            });

        grid.iter().for_each(|row| {
            let s: String = row.iter().collect();
            println!("{s}");
        });
        println!("------------------");
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_example() {
        let actual = count_distinct_locations_visited(EXAMPLE);
        let expected = 13;

        assert_eq!(actual, expected);
    }

    const VISIT_ONE_MORE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
U 2
";

    #[test]
    fn test_visit_one_more() {
        let actual = count_distinct_locations_visited(VISIT_ONE_MORE);
        let expected = 14;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_joint_tail_initial_example() {
        let actual = count_multi_joint_tail_positions_visited(EXAMPLE, 10);
        let expected = 1;

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_multi_joint_different_lengths() {
        let actual = count_multi_joint_tail_positions_visited(EXAMPLE, 4);
        let expected = 4;
        assert_eq!(actual, expected);
    }

    const LARGE_EXAMPLE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_large_multi_jointed_tail_example() {
        let actual = count_multi_joint_tail_positions_visited(LARGE_EXAMPLE, 10);
        let expected = 36;

        assert_eq!(actual, expected);
    }

    const DOUBLE_BACK: &str = "R 20
        L 20";

    #[test]
    fn test_double_back() {
        let actual = count_multi_joint_tail_positions_visited(DOUBLE_BACK, 10);
        let expected = 12;

        assert_eq!(actual, expected);
    }

    const UNMOVING_SIXTH: &str = "R 4
        U 4";

    #[test]
    fn test_unmoving_6th_segment_edge_case() {
        let actual = count_multi_joint_tail_positions_visited(UNMOVING_SIXTH, 7);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_visualization() {
        visualize_multi_jointed_tail(EXAMPLE, 10);
        //assert!(false)
    }
}
