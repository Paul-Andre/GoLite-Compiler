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
    for _ in 1..size {
        write!(ret, "\t");
    }
    ret
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
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
