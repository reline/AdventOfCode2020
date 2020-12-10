use string_builder::Builder;
/**
byr (Birth Year) - four digits; at least 1920 and at most 2002.
iyr (Issue Year) - four digits; at least 2010 and at most 2020.
eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
hgt (Height) - a number followed by either cm or in:
If cm, the number must be at least 150 and at most 193.
If in, the number must be at least 59 and at most 76.
hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pid (Passport ID) - a nine-digit number, including leading zeroes.
cid (Country ID) - ignored, missing or not.
**/

const BYR: &str = "byr";
const IYR: &str = "iyr";
const EYR: &str = "eyr";
const HGT: &str = "hgt";
const HCL: &str = "hcl";
const ECL: &str = "ecl";
const PID: &str = "pid";

fn required_fields<'a>() -> Vec<&'a str> {
    vec![BYR, IYR, EYR, HGT, HCL, ECL, PID]
}

pub fn partone(passports: &[String]) -> i32 {
    count_valid_passports(passports, |s| is_valid(s))
}

pub fn parttwo(passports: &[String]) -> i32 {
    count_valid_passports(passports, |s| is_valid_strict(s))
}

fn count_valid_passports(passports: &[String], validator: impl Fn(&str) -> bool) -> i32 {
    let mut valid_count = 0;
    let mut builder = Builder::default();
    for (i, line) in passports.iter().enumerate() {
        if !line.is_empty() {
            builder.append(" ");
            builder.append(line.as_str());
        }
        if line.is_empty() || i == passports.len() - 1 {
            if validator(builder.string().unwrap().as_str()) {
                valid_count += 1;
            }
            builder = Builder::default();
        }
    }
    return valid_count;
}

fn is_valid(passport: &str) -> bool {
    required_fields()
        .iter()
        .all(|&field| passport.contains(field))
}

fn is_valid_strict(passport: &str) -> bool {
    let birth_year = find_int_field(passport, BYR);
    if birth_year.is_none() || !is_valid_byr(birth_year.unwrap()) {
        return false;
    }

    let issue_year = find_int_field(passport, IYR);
    if issue_year.is_none() || !is_valid_iyr(issue_year.unwrap()) {
        return false;
    }

    let expiration_year = find_int_field(passport, EYR);
    if expiration_year.is_none() || !is_valid_eyr(expiration_year.unwrap()) {
        return false;
    }

    if !has_valid_hgt(passport) {
        return false;
    }

    let hair_color = find_string_field(passport, HCL);
    if hair_color.is_none() || !is_valid_hcl(&hair_color.unwrap()) {
        return false;
    }

    let eye_color = find_string_field(passport, ECL);
    if eye_color.is_none() || !is_valid_ecl(&eye_color.unwrap()) {
        return false;
    }

    let passport_id = find_string_field(passport, PID);
    if passport_id.is_none() || passport_id.unwrap().len() != 9 {
        return false;
    }

    true
}

fn find_int_field(passport: &str, field: &str) -> Option<i64> {
    let f = find_string_field(passport, field);
    if f.is_none() {
        return None;
    }
    let v = f.unwrap().parse::<i64>().unwrap();
    Some(v)
}

fn find_string_field(passport: &str, field: &str) -> Option<String> {
    let o = passport.find(field);
    if o.is_none() {
        return None;
    }
    let start = o.unwrap() + field.len() + 1; // :
    let v = passport
        .chars()
        .enumerate()
        .filter(|&(i, _)| i >= start)
        .take_while(|&(_, c)| c != ' ')
        .map(|(_, c)| c)
        .collect();
    Some(v)
}

fn is_valid_byr(byr: i64) -> bool {
    1920 <= byr && byr <= 2002
}

fn is_valid_iyr(iyr: i64) -> bool {
    2010 <= iyr && iyr <= 2020
}

fn is_valid_eyr(eyr: i64) -> bool {
    2020 <= eyr && eyr <= 2030
}

fn has_valid_hgt(passport: &str) -> bool {
    let o = find_string_field(passport, HGT);
    if o.is_none() {
        return false;
    }
    let u = o.unwrap();
    let (hgt, system) = u.split_at(u.len() - 2);
    let height = hgt.parse::<i32>().unwrap();
    if system == "cm" {
        return 150 <= height && height <= 193;
    }
    if system == "in" {
        return 59 <= height && height <= 76;
    }
    false
}

fn is_valid_hcl(hcl: &str) -> bool {
    hcl.chars().enumerate().all(|(i, c)| {
        if i == 0 {
            c == '#'
        } else {
            "0123456789abcdef".contains(c)
        }
    })
}

fn is_valid_ecl(ecl: &str) -> bool {
    "amb blu brn gry grn hzl oth".contains(ecl)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let l = vec![
            String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
            String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
            String::from(""),
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        ];
        assert_eq!(partone(&l), 2);
    }

    #[test]
    fn test_part_two_invalid() {
        let passports = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
            .lines()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        assert_eq!(parttwo(&passports), 0);
    }

    #[test]
    fn test_part_two_valid() {
        let passports = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
            .lines()
            .map(|s| String::from(s))
            .collect::<Vec<String>>();
        assert_eq!(parttwo(&passports), 4);
    }

    #[test]
    fn test_find_string_field() {
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert_eq!(find_string_field(passport, "pid").unwrap(), "087499704");
        assert_eq!(find_string_field(passport, "hgt").unwrap(), "74in");
        assert_eq!(find_string_field(passport, "ecl").unwrap(), "grn");
        assert_eq!(find_string_field(passport, "iyr").unwrap(), "2012");
        assert_eq!(find_string_field(passport, "eyr").unwrap(), "2030");
        assert_eq!(find_string_field(passport, "byr").unwrap(), "1980");
        assert_eq!(find_string_field(passport, "hcl").unwrap(), "#623a2f");
    }

    #[test]
    fn test_find_int_field() {
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert_eq!(find_int_field(passport, "iyr").unwrap(), 2012);
        assert_eq!(find_int_field(passport, "eyr").unwrap(), 2030);
        assert_eq!(find_int_field(passport, "byr").unwrap(), 1980);
    }

    #[test]
    fn test_is_valid_hcl() {
        assert!(is_valid_hcl("#623a2f"));
    }

    #[test]
    fn test_has_valid_hgt() {
        let passport = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";
        assert!(has_valid_hgt(passport))
    }
}
