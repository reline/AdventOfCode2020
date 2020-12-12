use std::collections::HashSet;
use std::iter::FromIterator;

pub fn partone(g: &[String]) -> usize {
    g.into_iter()
        .enumerate()
        .scan(HashSet::new(), |state, (i, x)| {
            for x in x.chars() {
                state.insert(x);
            }
            if x.is_empty() || i == g.len() - 1 {
                let s = state.len();
                state.clear();
                return Some(s);
            }
            return Some(0);
        })
        .sum()
}

pub fn parttwo(input: &[String]) -> usize {
    let i: String = input.join("\n");
    i.split("\n\n")
        .map(|g| {
            g.lines()
                .map(|l| l.chars())
                .map(HashSet::<char>::from_iter)
                .reduce(|a, b| a.intersection(&b).copied().collect())
                .map(|it| it.len())
                .unwrap()
        })
        .sum()
}

trait ReduceExt: Iterator {
    fn reduce<F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item;
}

impl<I: Iterator> ReduceExt for I {
    fn reduce<F>(mut self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        let first = self.next()?;
        Some(self.fold(first, f))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let g = "abcx\nabcy\nabcz"
            .lines()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"
            .lines()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        assert_eq!(partone(&g), 6);
        assert_eq!(partone(&input), 11);
    }

    #[test]
    fn test_part_two() {
        let input = "abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb"
            .lines()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        assert_eq!(parttwo(&input), 6);
    }
}
