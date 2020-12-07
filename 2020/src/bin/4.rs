use std::collections::HashSet;

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

fn main() -> Result<()> {
    let complete_passport: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .cloned()
        .collect();

    let passports = include_str!("../../inputs/4.txt")
        .split("\n\n")
        .map(|passport| {
            passport
                .split(|c| c == '\n' || c == ' ')
                .filter(|s| !s.is_empty())
                .map(|s| s.split(':').next().unwrap())
                .filter(|s| *s != "cid")
                .collect::<HashSet<&str>>()
        })
        .filter(|pass| complete_passport.difference(pass).count() == 0)
        .count();

    println!("{:#?}", passports);

    let passports = include_str!("../../inputs/4.txt")
        .split("\n\n")
        .map(Passport::from)
        .filter(|p| p.is_valid())
        .count();

    println!("{:#?}", passports);

    Ok(())
}

struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
}

impl Default for Passport {
    fn default() -> Self {
        Passport {
            byr: String::from(""),
            iyr: String::from(""),
            eyr: String::from(""),
            hgt: String::from(""),
            hcl: String::from(""),
            ecl: String::from(""),
            pid: String::from(""),
        }
    }
}

impl From<&str> for Passport {
    fn from(s: &str) -> Self {
        let infos = s
            .split(|c| c == '\n' || c == ' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.split(':').collect())
            .map(|v: Vec<&str>| (v[0], v[1]))
            .collect::<Vec<(&str, &str)>>();

        let mut p = Passport {
            ..Default::default()
        };

        for (k, v) in infos {
            match k {
                "byr" => {
                    p.byr = String::from(v);
                }
                "iyr" => {
                    p.iyr = String::from(v);
                }
                "eyr" => {
                    p.eyr = String::from(v);
                }
                "hgt" => {
                    p.hgt = String::from(v);
                }
                "hcl" => {
                    p.hcl = String::from(v);
                }
                "ecl" => {
                    p.ecl = String::from(v);
                }
                "pid" => {
                    p.pid = String::from(v);
                }
                _ => (),
            };
        }

        p
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        let byr = self.byr.parse::<u32>().unwrap_or(1900);
        if byr < 1920 || byr > 2002 {
            return false;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        let iyr = self.iyr.parse::<u32>().unwrap_or(1900);
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        let eyr = self.eyr.parse::<u32>().unwrap_or(1900);
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        let hs: Vec<&str> = self.hgt.splitn(2, |c: char| !c.is_digit(10)).collect();
        if hs.len() <= 1 {
            return false;
        } else {
            let h = hs[0].parse::<u32>().unwrap_or(120);
            if !match hs[1] {
                "m" => h >= 150 && h <= 193,
                "n" => h >= 59 && h <= 76,
                _ => false,
            } {
                return false;
            }
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        let mut hcl = self.hcl.clone();
        if !hcl.starts_with('#') || !hcl.len() == 7 {
            return false;
        }

        hcl.drain(0..1);
        if hcl.chars().any(|c| !matches!(c, '0'..='9' | 'a'..='f')) {
            return false;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if !matches!(
            self.ecl.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        ) {
            return false;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if self.pid.chars().any(|c| !matches!(c, '0'..='9')) || self.pid.len() != 9 {
            return false;
        }

        true
    }
}

// cargo run --release --bin 4
