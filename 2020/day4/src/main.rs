use std::collections::HashMap;
use std::fs;

fn main() {

    let input = read_input("input.txt");
    let mut passports = parse_input(&input);
    let num = count_valid_passports(&mut passports);
    println!("{} {}", passports.len(), num);

    let num = count_validated_passports(&mut passports);
    println!("{} {}", passports.len(), num);
}

fn read_input(path: &str) -> String {
    fs::read_to_string(path)
        .expect(&format!("Cannot open [{}]", path.to_string()))
        .to_string()
}

fn parse_input(input: &str) -> Vec<Passport> {
    input
        .split("\n\n")
        .map(|entry| parse_passport_entry(entry))
        .collect::<Vec<Passport>>()
}

fn parse_passport_entry(entry: &str) -> Passport {
    let mut passport = Passport::new();
    entry.split_whitespace()
        .for_each(|token| passport.parse_token(token));

    passport
}

fn count_valid_passports(passports: &mut Vec<Passport>) -> i32 {
    passports.into_iter().fold(0, |num, passport| {
        if passport.is_valid_entry() {
            return num+1;
        }
        num
    })
}

fn count_validated_passports(passports: &mut Vec<Passport>) -> i32 {
    passports.into_iter().fold(0, |num, passport| {
        if passport.is_valid() {
            return num+1;
        }
        num
    })
}

#[derive(Debug)]
struct Passport {
    data: HashMap<String,String>
}
impl Passport {
    fn new() -> Passport {
        Passport { data: HashMap::new()}
    }

    fn parse_token(self: &mut Self, token: &str) {
        let mut split_token = token.split(":");
        self.data.insert(split_token.next().unwrap().to_string(),
                         split_token.next().unwrap().to_string());
    }

    fn is_valid_entry(&self) -> bool {
        self.data.contains_key("byr") &&
        self.data.contains_key("iyr") &&
        self.data.contains_key("eyr") &&
        self.data.contains_key("hgt") &&
        self.data.contains_key("hcl") &&
        self.data.contains_key("ecl") &&
        self.data.contains_key("pid")
    }

    fn is_valid(&self) -> bool {
        self.is_valid_entry() &&
            self.is_iyr_valid() &&
            self.is_byr_valid() &&
            self.is_ecl_valid() &&
            self.is_eyr_valid() &&
            self.is_hcl_valid() &&
            self.is_hgt_valid() &&
            self.is_pid_valid()
    }

    fn is_byr_valid(&self) -> bool {
        let byr = self.data.get("byr").unwrap().parse::<i32>().unwrap();
        byr >= 1920 && byr <= 2002
    }

    fn is_iyr_valid(&self) -> bool {
        let iyr = self.data.get("iyr").unwrap().parse::<i32>().unwrap();
        iyr >= 2010 && iyr <= 2020
    }

    fn is_eyr_valid(&self) -> bool {
        let eyr = self.data.get("eyr").unwrap().parse::<i32>().unwrap();
        eyr >= 2020 && eyr <= 2030
    }

    fn is_hcl_valid(&self) -> bool {
        let hcl = self.data.get("hcl").unwrap();

        hcl.starts_with("#") &&
        hcl.len() == 7 &&
        hcl.strip_prefix("#")
            .unwrap()
            .chars()
            .all(|chr| ['0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'].contains(&chr))
    }

    fn is_ecl_valid(&self) -> bool {
        let valid_colors = ["amb","blu","brn","gry","grn","hzl","oth"];
        let ecl = self.data.get("ecl").unwrap().as_str();

        ecl.len() == 3 &&
        valid_colors.contains(&ecl)
    }

    fn is_pid_valid(&self) -> bool {
        let pid = self.data.get("pid").unwrap();

        pid.len() == 9 &&
        match pid.parse::<u32>() {
            Ok(pid) => pid < 1000000000,
            Err(_err) => false
        }
    }

    fn is_hgt_valid(&self) -> bool {
        let hgt = self.data.get("hgt").unwrap();

        if hgt.ends_with("cm") {
            return self.check_cm_bounds(self.prepare_hgt_value( hgt, "cm"));
        }

        if hgt.ends_with("in") {
            return self.check_in_bounds(self.prepare_hgt_value( hgt, "in"));
        }

        false
    }

    fn prepare_hgt_value(&self, hgt: &String, suffix: &str) -> i32 {
        hgt.strip_suffix(suffix)
            .unwrap()
            .parse::<i32>()
            .unwrap()
    }

    fn check_cm_bounds(&self, hgt:i32) -> bool {
        hgt >= 150 && hgt <= 193
    }

    fn check_in_bounds(&self, hgt:i32) -> bool {
        hgt >= 59 && hgt <= 76
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passport_token() {
        let token = "pid:760753108";
        let mut test_data = HashMap::new();
        test_data.insert("pid".to_string(), "760753108".to_string());
        let mut passport = Passport::new();
        passport.parse_token(token);
        assert_eq!(passport.data, test_data)
    }

    #[test]
    fn passport_entry() {
        let mut test_data = HashMap::new();
        test_data.insert("hcl".to_string(), "#ae17e1".to_string());
        test_data.insert("iyr".to_string(), "2013".to_string());
        test_data.insert("eyr".to_string(), "2024".to_string());
        test_data.insert("ecl".to_string(), "brn".to_string());
        test_data.insert("pid".to_string(), "760753108".to_string());
        test_data.insert("byr".to_string(), "1931".to_string());
        test_data.insert("hgt".to_string(), "179cm".to_string());

        let passport_entry = "hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm";

        let passport = parse_passport_entry(passport_entry);
        assert_eq!(passport.data, test_data);
    }

    #[test]
    fn check_for_invalid_passport() {
        let passport_entry = "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let passport = parse_passport_entry(passport_entry);
        println!("{:?}", passport);
        assert_eq!(passport.is_valid_entry(), false);
    }

    #[test]
    fn split_input()
    {
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

        let passports: Vec<Passport> = parse_input(input);
        assert_eq!(passports.len(), 4);
    }

    #[test]
    #[should_panic]
    fn input_not_readable() {
        read_input("non_existing_file.txt");
    }

    #[test]
    fn number_of_vlid_passports() {
        let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59";

        let mut passports: Vec<Passport> = parse_input(input);
        assert_eq!(count_valid_passports(&mut passports), 2);
    }

    #[test]
    fn validate_birth_year()
    {
        let mut passport = Passport::new();
        passport.data.insert("byr".to_string(),"1920".to_string());
        assert_eq!(passport.is_byr_valid(), true);

        let mut passport = Passport::new();
        passport.data.insert("byr".to_string(),"2002".to_string());
        assert_eq!(passport.is_byr_valid(), true);

        let mut passport = Passport::new();
        passport.data.insert("byr".to_string(),"1910".to_string());
        assert_eq!(passport.is_byr_valid(), false);

        let mut passport = Passport::new();
        passport.data.insert("byr".to_string(),"2003".to_string());
        assert_eq!(passport.is_byr_valid(), false);
    }

    #[test]
    fn validate_issue_year()
    {
        let mut passport = Passport::new();
        passport.data.insert("iyr".to_string(),"2010".to_string());
        assert_eq!(passport.is_iyr_valid(), true);

        let mut passport = Passport::new();
        passport.data.insert("iyr".to_string(),"2020".to_string());
        assert_eq!(passport.is_iyr_valid(), true);

        let mut passport = Passport::new();
        passport.data.insert("iyr".to_string(),"2000".to_string());
        assert_eq!(passport.is_iyr_valid(), false);

        let mut passport = Passport::new();
        passport.data.insert("iyr".to_string(),"2023".to_string());
        assert_eq!(passport.is_iyr_valid(), false);
    }

    #[test]
    fn validate_expiration_year()
    {
        let mut passport = Passport::new();
        passport.data.insert("eyr".to_string(),"2020".to_string());
        assert_eq!(passport.is_eyr_valid(), true);

        let mut passport = Passport::new();
        passport.data.insert("eyr".to_string(),"2030".to_string());
        assert_eq!(passport.is_eyr_valid(), true);

        let mut passport = Passport::new();
        passport.data.insert("eyr".to_string(),"2019".to_string());
        assert_eq!(passport.is_eyr_valid(), false);

        let mut passport = Passport::new();
        passport.data.insert("eyr".to_string(),"2031".to_string());
        assert_eq!(passport.is_eyr_valid(), false);
    }

    #[test]
    fn validate_heigth_cm()
    {
        let mut passport = Passport::new();
        passport.data.insert("hgt".to_string(),"150cm".to_string());
        assert_eq!(passport.is_hgt_valid(), true);

        passport.data.insert("hgt".to_string(),"193cm".to_string());
        assert_eq!(passport.is_hgt_valid(), true);

        passport.data.insert("hgt".to_string(),"40cm".to_string());
        assert_eq!(passport.is_hgt_valid(), false);

        passport.data.insert("hgt".to_string(),"250cm".to_string());
        assert_eq!(passport.is_hgt_valid(), false);

        passport.data.insert("hgt".to_string(),"183".to_string());
        assert_eq!(passport.is_hgt_valid(), false);
    }

    #[test]
    fn validate_heigth_in()
    {
        let mut passport = Passport::new();

        passport.data.insert("hgt".to_string(),"59in".to_string());
        assert_eq!(passport.is_hgt_valid(), true);

        passport.data.insert("hgt".to_string(),"76in".to_string());
        assert_eq!(passport.is_hgt_valid(), true);

        passport.data.insert("hgt".to_string(),"40in".to_string());
        assert_eq!(passport.is_hgt_valid(), false);

        passport.data.insert("hgt".to_string(),"100in".to_string());
        assert_eq!(passport.is_hgt_valid(), false);

        passport.data.insert("hgt".to_string(),"70".to_string());
        assert_eq!(passport.is_hgt_valid(), false);
    }

    #[test]
    fn validate_hair_color()
    {
        let mut passport = Passport::new();
        passport.data.insert("hcl".to_string(),"#123456".to_string());
        assert_eq!(passport.is_hcl_valid(), true);

        passport.data.insert("hcl".to_string(),"#abcdef".to_string());
        assert_eq!(passport.is_hcl_valid(), true);

        passport.data.insert("hcl".to_string(),"12AB".to_string());
        assert_eq!(passport.is_hcl_valid(), false);

        passport.data.insert("hcl".to_string(),"#123ABC".to_string());
        assert_eq!(passport.is_hcl_valid(), false);

        passport.data.insert("hcl".to_string(),"123abc".to_string());
        assert_eq!(passport.is_hcl_valid(), false);

    }

    #[test]
    fn validate_eye_color()
    {
        let mut passport = Passport::new();
        passport.data.insert("ecl".to_string(),"amb".to_string());
        assert_eq!(passport.is_ecl_valid(), true);

        passport.data.insert("ecl".to_string(),"blu".to_string());
        assert_eq!(passport.is_ecl_valid(), true);

        passport.data.insert("ecl".to_string(),"oth".to_string());
        assert_eq!(passport.is_ecl_valid(), true);

        passport.data.insert("ecl".to_string(),"arrr".to_string());
        assert_eq!(passport.is_ecl_valid(), false);
    }

    #[test]
    fn validate_pid()
    {
        let mut passport = Passport::new();
        passport.data.insert("pid".to_string(),"123456789".to_string());
        assert_eq!(passport.is_pid_valid(), true);

        passport.data.insert("pid".to_string(),"000456789".to_string());
        assert_eq!(passport.is_pid_valid(), true);

        passport.data.insert("pid".to_string(),"1234567890".to_string());
        assert_eq!(passport.is_pid_valid(), false);
    }
}