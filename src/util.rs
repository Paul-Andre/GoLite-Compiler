use std::u32;
use std::fmt::Write;

pub fn string_to_int(s: &str) -> u32 {
    assert_ne!(s, "");
    if s == "0" {
        return 0;
    }
    if &s[0..1] == "0" &&
        (&s[1..2] == "x" || &s[1..2] == "X")  {
            return u32::from_str_radix(&s[2..], 16).unwrap();
        }
    if &s[0..1] == "0" {
        return u32::from_str_radix(&s[1..], 8).unwrap();
    }
    return u32::from_str_radix(s, 10).unwrap();
}

pub fn indent(size: u32) -> String {
    let mut ret = "".to_string();
    for _ in 0..size {
        write!(ret, "\t").unwrap();
    }
    ret
}

// !!! This assumes it is a valid rune literal.
pub fn parse_rune_literal(value: &str) -> i32 {
    let letter : &str;
    if value.len() == 4 {
        letter = &value[1..3];
    } else {
        letter = &value[1..2];
    }

    let code_no = match letter {
        "\\a" => 7,
        "\\b" => 8,
        "\\f" => 12,
        "\\n" => 10,
        "\\r" => 13,
        "\\t" => 9,
        "\\v" => 11,
        "\\\\" => 92,
        "\\'" => 39,
        _ => letter.chars().next().unwrap() as i32 // Will this work?
    };
    return code_no;
}

// TODO: based on usage of this function, figure out if there's a better thing to return
// (Some options here would be to pass a formatter, or to return an iterator...)
pub fn parse_string_literal(value: &str) -> String {
    let letter = &value[0..1];

    match letter {
        "`" => { // Raw
            assert!(&value[value.len()-1..] == "`");
            return value[1..value.len()-1].to_string();
        },
        "\"" => { // Interpreted
            assert!(&value[value.len()-1..] == "\"");
            let mut ret = String::new();
            let mut it = value[1..value.len()-1].chars();

            while let Some(c) = it.next() {
                if (c == '\\' ){
                    if let Some(cc) = it.next() {
                        let escape =
                        match cc {
                            'a' => 7 as char,
                            'b' => 8 as char,
                            'f' => 12 as char,
                            'n' => 10 as char,
                            'r' => 13 as char,
                            't' => 9 as char,
                            'v' => 11 as char,
                            '\\' => '\\',
                            '"' => '"',
                            _ => {
                                panic!("Escape sequence not recognized.");
                            }
                        };
                        write!(ret, "{}", escape).unwrap();

                    } else {
                        panic!("Unfinished escape code in string literal");
                    }
                } else {
                    write!(ret, "{}", c).unwrap();
                }
            }
            return ret;
        }
        _ => {
            panic!("A string should be either interpreted or raw");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_int() {
        assert_eq!(string_to_int("0"), 0);
        assert_eq!(string_to_int("00"), 0);
        assert_eq!(string_to_int("000"), 0);
        assert_eq!(string_to_int("0x0"), 0);
        assert_eq!(string_to_int("0X0"), 0);

        assert_eq!(string_to_int("0Xf"), 15);
        assert_eq!(string_to_int("0XF"), 15);

        assert_eq!(string_to_int("0X10"), 16);
        assert_eq!(string_to_int("010"), 8);
        assert_eq!(string_to_int("10"), 10);

        assert_eq!(string_to_int("12341234"), 12341234);
        assert_eq!(string_to_int("0xfedced1"), 0xfedced1);
    }

}
