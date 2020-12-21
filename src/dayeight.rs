use std::collections::HashSet;

const ACC: &str = "acc";
const JMP: &str = "jmp";

pub fn partone(input: &str) -> isize {
    let mut index = 0;
    let mut acc = 0;
    let mut visited = HashSet::<usize>::new();
    let commands = input
        .lines()
        .map(|l| parse_line(l))
        .collect::<Vec<(&str, isize)>>();
    loop {
        if visited.contains(&index) {
            break;
        }
        visited.insert(index);
        let &(cmd, arg) = commands.get(index).unwrap();
        match cmd {
            ACC => {
                acc += arg;
                index += 1;
            }
            JMP => {
                let i = index as isize + arg;
                index = i as usize;
            }
            _ /*NOP*/ => {
                index += 1;
            }
        };
    }
    return acc;
}

fn parse_line(line: &str) -> (&str, isize) {
    let (cmd, num) = line.split_at(3);
    let arg = num.trim().parse::<isize>().unwrap();
    (cmd, arg)
}

pub fn parttwo(input: &str) -> isize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(("acc", -99), parse_line("acc -99"));
        assert_eq!(("jmp", 3), parse_line("jmp +3"));
        assert_eq!(("NOP", 0), parse_line("NOP +0"));
    }

    #[test]
    fn test_part_one() {
        let boot_code = "NOP +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(5, partone(boot_code));
    }

    // #[test]
    fn test_part_two() {
        let boot_code = "NOP +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(8, parttwo(boot_code));
    }
}
