pub fn partone(map: &[String]) -> i64 {
    return determine_arboreal_stop_probability(map, (3, 1));
}

/**
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
**/
pub fn parttwo(map: &[String]) -> i64 {
    determine_arboreal_stop_probability(map, (1, 1))
        * determine_arboreal_stop_probability(map, (3, 1))
        * determine_arboreal_stop_probability(map, (5, 1))
        * determine_arboreal_stop_probability(map, (7, 1))
        * determine_arboreal_stop_probability(map, (1, 2))
}

fn determine_arboreal_stop_probability(map: &[String], slope: (usize, usize)) -> i64 {
    // start at 0,0
    let mut y = 0;
    let mut x = 0;
    // first dimension is y
    let height = map.len() - 1;
    // second dimension is x (chars)
    let width = map[0].len();
    let mut impacts = 0;
    while y < height {
        x += slope.0;
        y += slope.1;
        if x >= width {
            x -= width; // wrap around
        }
        let c = &map[y].chars().nth(x).unwrap();
        if c == &'#' {
            impacts += 1;
        }
    }
    return impacts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let map = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ];
        assert_eq!(partone(&map), 7);
    }

    #[test]
    fn test_part_two() {
        let map = vec![
            String::from("..##......."),
            String::from("#...#...#.."),
            String::from(".#....#..#."),
            String::from("..#.#...#.#"),
            String::from(".#...##..#."),
            String::from("..#.##....."),
            String::from(".#.#.#....#"),
            String::from(".#........#"),
            String::from("#.##...#..."),
            String::from("#...##....#"),
            String::from(".#..#...#.#"),
        ];
        assert_eq!(parttwo(&map), 336);
    }
}
