use regex::Regex;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

pub fn partone(input: &str) -> usize {
    outer_bags_for_color("shiny gold", &process_rules(input)).len()
}

pub fn parttwo(input: &str) -> usize {
    inner_bags_for_color("shiny gold", &process_rules(input)).len() // fixme
}

fn process_rules(rules: &str) -> HashMap<String, HashSet<String>> {
    rules
        .lines()
        .map(|line| rules_for_bag(line))
        .fold(HashMap::new(), |mut s, (k, v)| {
            s.insert(k, v);
            return s;
        })
}

struct Rule {
    count: usize,
    color: String,
}

impl PartialEq for Rule {
    fn eq(&self, other: &Rule) -> bool {
        self.color == other.color
    }
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.color.hash(state);
    }
}

impl Borrow<String> for Rule {
    fn borrow(&self) -> &String {
        &self.color
    }
}

fn rules_for_bag(input: &str) -> (String, HashSet<String>) {
    let re = Regex::new(r"([0-9]) (.*) bag").unwrap();
    let x = input.split(" bags contain ").collect::<Vec<&str>>();
    let rules = x[1]
        .split(",")
        .filter(|&r| !r.contains("no other bags"))
        .map(|r| {
            let count = re
                .captures(r)
                .unwrap()
                .get(1)
                .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
            let color = re.captures(r).unwrap().get(2).map_or("", |m| m.as_str());
            // Rule {
            //     count,
            //     color: String::from(color),
            // }
            String::from(color)
        })
        .collect::<HashSet<String>>();
    (String::from(x[0]), rules)
}

fn outer_bags_for_color(color: &str, rules: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    rules.into_iter().fold(HashSet::new(), |mut s, (c, r)| {
        if r.contains(color) {
            s.insert(c.to_string());
            return outer_bags_for_color(c.as_str(), rules)
                .union(&s)
                .map(|it| it.to_string())
                .collect();
        }
        return s;
    })
}

fn inner_bags_for_color(color: &str, rules: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    HashSet::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(4, partone(rules));
    }

    // #[test]
    fn test_part_two() {
        let r1 = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(126, parttwo(r1));
        let r2 = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        assert_eq!(32, parttwo(r2));
    }
}
