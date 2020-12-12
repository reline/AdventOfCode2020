use crate::reduce::ReduceExt;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn partone(g: &String) -> usize {
    collect_questionaires(g, &|a, b| a.union(&b).copied().collect())
}

pub fn parttwo(i: &String) -> usize {
    collect_questionaires(i, &|a, b| a.intersection(&b).copied().collect())
}

fn collect_questionaires(
    i: &String,
    reducer: &impl Fn(HashSet<char>, HashSet<char>) -> HashSet<char>,
) -> usize {
    i.split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars())
                .map(HashSet::<char>::from_iter)
                .reduce(reducer)
                .map(|it| it.len())
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let g = "abcx\nabcy\nabcz";
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(partone(&String::from(g)), 6);
        assert_eq!(partone(&String::from(input)), 11);
    }

    #[test]
    fn test_part_two() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb";
        assert_eq!(parttwo(&String::from(input)), 6);
    }
}
