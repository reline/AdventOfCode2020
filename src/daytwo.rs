pub fn partone(passwords: &[String]) -> i32 {
    let mut valid_count: i32 = 0;
    for line in passwords {
        let input: Vec<&str> = line
            .split(|c| c == ' ' || c == ':')
            .filter(|s| !s.is_empty())
            .collect();

        let policy_occurrences_range: Vec<&str> = input[0].split('-').collect();
        let min_occurrences = policy_occurrences_range[0].parse::<i32>().unwrap();
        let max_occurrences = policy_occurrences_range[1].parse::<i32>().unwrap();
        let policy_required = input[1];
        let password = input[2];
        let occurrences = password.match_indices(policy_required).count();
        let is_valid =
            occurrences >= min_occurrences as usize && occurrences <= max_occurrences as usize;
        if is_valid {
            valid_count += 1;
        }
    }
    return valid_count;
}

pub fn parttwo(passwords: &[String]) -> i32 {
    let mut valid_count: i32 = 0;
    for line in passwords {
        let input: Vec<&str> = line
            .split(|c| c == ' ' || c == ':')
            .filter(|s| !s.is_empty())
            .collect();

        let policy_occurrences: Vec<&str> = input[0].split('-').collect();
        let first_index = policy_occurrences[0].parse::<i32>().unwrap();
        let second_index = policy_occurrences[1].parse::<i32>().unwrap();
        let policy_required = input[1];
        let password = input[2];
        let occurrences = password
            .match_indices(policy_required)
            .map(|m| m.0 + 1)
            .filter(|&i| i == first_index as usize || i == second_index as usize)
            .count();
        let is_valid = occurrences == 1;
        if is_valid {
            valid_count += 1;
        }
    }
    return valid_count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let passwords = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        assert_eq!(partone(&passwords), 2);
    }

    #[test]
    fn test_part_two() {
        let passwords = vec![
            String::from("1-3 a: abcde"),
            String::from("1-3 b: cdefg"),
            String::from("2-9 c: ccccccccc"),
        ];
        assert_eq!(parttwo(&passwords), 1);
    }
}
