use std::collections::HashMap;
#[derive(Debug)]
struct PassportEntry(HashMap<String, String>);

fn parse_input(input: &str) -> Vec<PassportEntry> {
    input
        .split("\n\n")
        .map(|entry| {
            PassportEntry(
                entry
                    .split_whitespace()
                    .map(|s| {
                        let mut keyval = s.splitn(2, ":");
                        (
                            keyval.next().unwrap().to_string(),
                            keyval.next().unwrap().to_string(),
                        )
                    })
                    .collect(),
            )
        })
        .collect()
}

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const OPTIONAL_KEYS: [&str; 1] = ["cid"];

impl PassportEntry {
    fn valid_part1(&self) -> bool {
        // Check no invalid keys and all required keys present
        self.0
            .keys()
            .all(|k| REQUIRED_KEYS.contains(&k.as_str()) || OPTIONAL_KEYS.contains(&k.as_str()))
            && REQUIRED_KEYS.iter().all(|&k| self.0.get(k).is_some())
    }
    fn valid_part2(&self) -> bool {
        // Check data validation rules
        self.valid_part1() && self.0.iter().all(|(k, v)| {
            match k.as_str() {
                "byr" => {
                    //4-digit year 1920-2002
                    v.parse::<u16>()
                        .ok()
                        .map(|v| v >= 1920 && v <= 2002)
                        .unwrap_or(false)
                }
                "iyr" => {
                    //4-digit year 2010-2020
                    v.parse::<u16>()
                        .ok()
                        .map(|v| v >= 2010 && v <= 2020)
                        .unwrap_or(false)
                }
                "eyr" => {
                    // 4-digit year 2020-2030
                    v.parse::<u16>()
                        .ok()
                        .map(|v| v >= 2020 && v <= 2030)
                        .unwrap_or(false)
                }
                "hgt" => {
                    // 150-193cm OR 59-76in
                    (v.len() == 4 && {
                        let (a, b) = v.split_at(2);
                        b == "in"
                            && a.parse::<u8>()
                                .ok()
                                .map(|v| v >= 59 && v <= 76)
                                .unwrap_or(false)
                    }) || (v.len() == 5 && {
                        let (a, b) = v.split_at(3);
                        b == "cm"
                            && a.parse::<u8>()
                                .ok()
                                .map(|v| v >= 150 && v <= 193)
                                .unwrap_or(false)
                    })
                }
                "hcl" => {
                    // #[0-9a-f]{6}
                    v.chars().count() == 7 && {
                        let val_clone = v.clone();
                        let mut it = val_clone.chars();
                        it.next() == Some('#') && it.all(|f| f.is_ascii_digit() || (f.is_lowercase() && f.is_ascii_hexdigit()))
                    }
                }
                "ecl" => {
                    // One of: amb blu brn gry grn hzl oth
                    v == "amb"
                        || v == "blu"
                        || v == "brn"
                        || v == "gry"
                        || v == "grn"
                        || v == "hzl"
                        || v == "oth"
                }
                "pid" => {
                    // Nine digits (may be leading 0)
                    v.chars().all(|f| f.is_ascii_digit()) && v.chars().count() == 9
                }
                "cid" => {
                    // Always valid
                    true
                }
                _ => {
                    // Unknown field
                    false
                }
            }
        })
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let entries = parse_input(input);
    let valid = entries.iter().filter(|e| e.valid_part1()).count();
    println!("Part 1: {} valid entries", valid);
    let valid2 = entries.iter().filter(|e| e.valid_part2()).count();
    println!("Part 2: {} valid entries", valid2);
}

#[cfg(test)]
mod tests {
    const GOOD_INPUTS: &str = r##"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"##;
    const BAD_INPUTS: &str = r#"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;
    use super::*;

    #[test]
    fn test_inputs() {
        let bad_entries = parse_input(BAD_INPUTS);
        let valid_bad = bad_entries.iter().filter(|e| e.valid_part2());
        assert_eq!(valid_bad.count(), 0);
        let good_entries = parse_input(GOOD_INPUTS);
        assert_eq!(good_entries.iter().filter(|e| e.valid_part2()).count(), 4);
    }
}
