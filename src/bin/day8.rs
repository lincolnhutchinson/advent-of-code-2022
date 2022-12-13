fn main() {
    println!("Hello day8!");

    let data = std::fs::read_to_string("src/data/day8.txt").unwrap();

    let result = count_visible_trees(&data);
    let scenic = find_high_scenic_score(&data);

    println!("VISIBLE TREES: {result}");
    println!("The highest scenic score is: {scenic}");
}

fn build_height_array(input: &str) -> Vec<Vec<u32>> {
    input
        .split_whitespace()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn build_visibility_array(input: &str) -> Vec<Vec<bool>> {
    let height_array = build_height_array(input);
    let height = height_array.len();
    let width = height_array[0].len();

    let mut vis_array = Vec::new();

    // Build rows and check L/R visibility
    for y in 0..height {
        let mut row = Vec::new();
        if y == 0 || y == height - 1 {
            // Border rows are all visible
            row.resize(width, true);
        } else {
            row.resize(width, false);
        }

        // left and right edges are always visible
        row[0] = true;
        row[width - 1] = true;

        // look from the left
        let mut min_height = height_array[y][0];
        for x in 1..width - 1 {
            let height = height_array[y][x];

            if height > min_height {
                row[x] = true;
                min_height = height;
            }
        }

        // look from the right
        let mut min_height = height_array[y][width - 1];
        for x in (1..width - 1).rev() {
            let height = height_array[y][x];

            if height > min_height {
                row[x] = true;
                min_height = height;
            }
        }

        vis_array.push(row);
    }

    // Go column by column and check UD Visibility
    for x in 0..width {
        // look from top down
        let mut min_height = height_array[0][x];
        for y in 1..height - 1 {
            let height = height_array[y][x];

            if height > min_height {
                vis_array[y][x] = true;
                min_height = height;
            }
        }

        // look from bottom up
        let mut min_height = height_array[height - 1][x];
        for y in (1..height - 1).rev() {
            let height = height_array[y][x];

            if height > min_height {
                vis_array[y][x] = true;
                min_height = height;
            }
        }
    }

    vis_array
}

fn count_visible_trees(input: &str) -> usize {
    let vis_array = build_visibility_array(input);

    vis_array
        .iter()
        .flatten()
        .fold(0, |acc, x| if *x { acc + 1 } else { acc })
}

fn find_high_scenic_score(input: &str) -> u32 {
    let height_array = build_height_array(input);
    let height = height_array.len();
    let width = height_array[0].len();

    let mut highest_score = 0;
    for y in 0..height {
        for x in 0..width {
            let tree_height = height_array[y][x];

            // look right
            let mut dist_right = 0;
            for right_x in x + 1..width {
                dist_right += 1;
                let view_height = height_array[y][right_x];

                if view_height >= tree_height {
                    break;
                }
            }

            // look left
            let mut dist_left = 0;
            for left_x in (0..x).rev() {
                dist_left += 1;
                let view_height = height_array[y][left_x];

                if view_height >= tree_height {
                    break;
                }
            }

            // look up
            let mut dist_up = 0;
            for up_y in (0..y).rev() {
                dist_up += 1;
                let view_height = height_array[up_y][x];

                if view_height >= tree_height {
                    break;
                }
            }

            // look down
            let mut dist_down = 0;
            for down_y in y + 1..height {
                dist_down += 1;
                let view_height = height_array[down_y][x];

                if view_height >= tree_height {
                    break;
                }
            }

            let this_score = dist_up * dist_down * dist_right * dist_left;
            highest_score = highest_score.max(this_score);
        }
    }

    highest_score
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "30373
25512
65332
33549
35390";

    const ALL_BORDER: &str = "1234
        5678";

    #[test]
    fn test_example() {
        const EXPECTED: usize = 21;
        let actual = count_visible_trees(EXAMPLE);

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn test_all_borders() {
        const EXPECTED: usize = 8;
        let actual = count_visible_trees(ALL_BORDER);

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn test_scenic_example() {
        const EXPECTED: u32 = 8;
        let actual = find_high_scenic_score(EXAMPLE);

        assert_eq!(actual, EXPECTED);
    }

    #[test]
    fn test_border_scenicness() {
        const EXPECTED: u32 = 0;
        let actual = find_high_scenic_score(ALL_BORDER);

        assert_eq!(actual, EXPECTED);
    }
}
