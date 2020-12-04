use regex::Regex;

const REQUIRED_VARIANTS: [&str; 7] = [
    "BirthYear",
    "IssueYear",
    "ExpiryYear",
    "Height",
    "HairColor",
    "EyeColor",
    "PassportId",
];
lazy_static::lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new(r#"^(\d+)(cm|in)$"#).unwrap();
    static ref HAIR_COLOR_REGEX: Regex = Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap();
    static ref PASSPORT_ID_REGEX: Regex = Regex::new(r#"^[0-9]{9}$"#).unwrap();
}

struct Passport<'a>(Vec<Entry<'a>>);

#[derive(strum::IntoStaticStr)]
enum Entry<'a> {
    BirthYear(u16),
    IssueYear(u16),
    ExpiryYear(u16),
    Height(&'a str),
    HairColor(&'a str),
    EyeColor(&'a str),
    PassportId(&'a str),
    CountryId,
}

impl<'a> Entry<'a> {
    fn from(input: (&'a str, &'a str)) -> Entry<'a> {
        match input.0 {
            "byr" => Entry::BirthYear(input.1.parse().unwrap()),
            "iyr" => Entry::IssueYear(input.1.parse().unwrap()),
            "eyr" => Entry::ExpiryYear(input.1.parse().unwrap()),
            "hgt" => Entry::Height(input.1),
            "hcl" => Entry::HairColor(input.1),
            "ecl" => Entry::EyeColor(input.1),
            "pid" => Entry::PassportId(input.1),
            "cid" => Entry::CountryId,
            f => panic!("Invalid field {}", f),
        }
    }

    #[inline]
    fn validate(&self) -> bool {
        match self {
            Entry::BirthYear(y) => (1920..=2002).contains(y),
            Entry::IssueYear(y) => (2010..=2020).contains(y),
            Entry::ExpiryYear(y) => (2020..=2030).contains(y),
            Entry::Height(h) => {
                let caps = HEIGHT_REGEX.captures(h);
                match caps {
                    Some(caps) => {
                        let num: u16 = caps[1].parse().unwrap();
                        let unit = &caps[2];
                        match (unit, num) {
                            ("cm", 150..=193) | ("in", 59..=76) => true,
                            _ => false,
                        }
                    }
                    None => false,
                }
            }
            Entry::HairColor(c) => HAIR_COLOR_REGEX.is_match(c),
            Entry::EyeColor(c) => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(c),
            Entry::PassportId(i) => PASSPORT_ID_REGEX.is_match(i),
            Entry::CountryId => true,
        }
    }
}

impl<'a> Passport<'a> {
    fn parse(batch: &'a str, re: &Regex) -> Passport<'a> {
        let entries = re
            .split(batch)
            .map(|pair| {
                let mut split = pair.split(':');
                Entry::from((split.next().unwrap(), split.next().unwrap()))
            })
            .collect();
        Passport(entries)
    }

    fn validate_required(&self) -> bool {
        let variants: Vec<&'static str> = self.0.iter().map(|e| e.into()).collect();
        REQUIRED_VARIANTS.iter().all(|k| variants.contains(k))
    }

    fn validate_valid(&self) -> bool {
        self.0.iter().all(|e| e.validate())
    }
}

fn parse<'a>(input: &'a str) -> Vec<Passport<'a>> {
    let regex = Regex::new("[\\s\\n]").unwrap();
    input
        .split("\n\n")
        .map(|batch| Passport::parse(batch, &regex))
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|p| p.validate_required())
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter(|p| p.validate_valid() && p.validate_required())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part2_invalid() {
        let input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        assert_eq!(part2(input), 0);
    }

    #[test]
    fn part2_valid() {
        let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        assert_eq!(part2(input), 4);
    }
}
