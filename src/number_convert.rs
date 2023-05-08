use regex::Regex;

struct NumberType {
    // number_dash: Regex,
    number_dot: Regex,
    number_space: Regex,
    number_no_space: Regex,
    number_no_010: Regex,
}

impl NumberType {
    fn new() -> Self {
        // let dash = Regex::new(r"^\d{3}-\d{4}-\d{4}$").unwrap();
        let dot = Regex::new(r"^\d{3}\.\d{4}\.\d{4}$").unwrap();
        let space = Regex::new(r"^\d{3} \d{4} \d{4}$").unwrap();
        let no_space = Regex::new(r"^\d{11}$").unwrap();
        let no_010 = Regex::new(r"^\d{8}").unwrap();

        NumberType {
            // number_dash: dash,
            number_dot: dot,
            number_space: space,
            number_no_space: no_space,
            number_no_010: no_010,
        }
    }
}

fn space_to_dash(number: &String) -> String {
    number.replace(" ", "-")
}

fn dot_to_dash(number: &String) -> String {
    number.replace(".", "-")
}

fn no_space_to_dash(number: &String) -> String {
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
fn no_010_to_dash(number: &String) -> String {
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

pub fn convert_number(number: &String) -> String {
    let nt = NumberType::new();

    let changed = match number {
        x if nt.number_dot.is_match(number) => dot_to_dash(x),
        x if nt.number_space.is_match(number) => space_to_dash(x),
        x if nt.number_no_space.is_match(number) => no_space_to_dash(x),
        x if nt.number_no_010.is_match(number) => no_010_to_dash(x),
        _ => number.to_owned(),
    };

    changed
}
