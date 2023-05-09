use regex::Regex;

struct NumberType {
    number_no_space: Regex,
    number_no_010: Regex,
    bad_number: Regex,
}

impl NumberType {
    fn new() -> Self {
        let no_space = Regex::new(r"^\d{11}$").unwrap();
        let no_010 = Regex::new(r"^\d{8}").unwrap();
        let re = Regex::new(r"(?P<y>01[0-9])[\s.~]+(?P<m>\d{4})[\s.~]+(?P<d>\d{4})").unwrap();

        NumberType {
            number_no_space: no_space,
            number_no_010: no_010,
            bad_number: re,
        }
    }
    fn convert(&self, number: &String) -> String {
        self.bad_number.replace_all(number, "$y-$m-$d").to_string()
    }

    fn no_space_to_dash(&self, number: &String) -> String {
        let first = &number[0..3];
        let second = &number[3..7];
        let third = &number[7..11];

        let mut output = String::new();
        output.push_str(first);
        output.push_str("-");
        output.push_str(second);
        output.push_str("-");
        output.push_str(third);
        output
    }
    fn no_010_to_dash(&self, number: &String) -> String {
        let first = &number[0..4];
        let second = &number[4..8];

        let mut output = String::new();
        output.push_str("010");
        output.push_str("-");
        output.push_str(first);
        output.push_str("-");
        output.push_str(second);
        output
    }
}

pub fn convert_number(number: &String) -> String {
    let nt = NumberType::new();

    let changed = match number {
        x if nt.bad_number.is_match(number) => nt.convert(x),
        x if nt.number_no_space.is_match(number) => nt.no_space_to_dash(x),
        x if nt.number_no_010.is_match(number) => nt.no_010_to_dash(x),
        _ => number.to_owned(),
    };

    changed
}
