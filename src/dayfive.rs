use std::cmp;
use std::collections::HashSet;
use string_builder::Builder;

pub fn partone(seats: &[String]) -> isize {
    let mut highest = 0;
    for x in seats {
        highest = cmp::max(highest, seat_id(x));
    }
    return highest;
}

pub fn parttwo(seats: &[String]) -> isize {
    let mut open_seats: HashSet<isize> = HashSet::new();
    let mut taken_seats: HashSet<isize> = HashSet::new();
    for x in seats {
        let sid = seat_id(x);
        open_seats.remove(&sid);
        taken_seats.insert(sid);
        if !taken_seats.contains(&(sid + 1)) {
            open_seats.insert(sid + 1);
        }
        if !taken_seats.contains(&(sid - 1)) {
            open_seats.insert(sid - 1);
        }
    }
    *open_seats
        .iter()
        .filter(|&s| taken_seats.contains(&(s + 1)) || taken_seats.contains(&(s - 1)))
        .nth(0)
        .unwrap()
}

fn seat_id(seat: &str) -> isize {
    let (r, c) = seat.split_at(7);
    row(r) * 8 + column(c)
}

fn row(r: &str) -> isize {
    to_bin(r)
}

fn column(c: &str) -> isize {
    to_bin(c)
}

fn to_bin(str: &str) -> isize {
    let mut builder = Builder::default();
    builder.append("0");
    for c in str.chars() {
        match c {
            'F' | 'L' => {
                builder.append('0');
            }
            'B' | 'R' => {
                builder.append('1');
            }
            _ => {}
        }
    }
    let s = builder.string().unwrap();
    isize::from_str_radix(s.as_str(), 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row() {
        let r = "FBFBBFF";
        assert_eq!(44, row(r));
    }

    #[test]
    fn test_column() {
        let col = "RLR";
        assert_eq!(5, column(col));
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(567, seat_id("BFFFBBFRRR"));
        assert_eq!(119, seat_id("FFFBBBFRRR"));
        assert_eq!(820, seat_id("BBFFBBFRLL"));
    }
}
