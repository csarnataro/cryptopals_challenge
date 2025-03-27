use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let string_to_decode = &args[1];
    let chars: Vec<char> = string_to_decode.chars().collect();
    let decoded = decode_base64(chars);
    println!("Decoded: [{decoded}]");
}

fn decode_base64(chars: Vec<char>) -> String {
    let mut base64 = String::new();
    for n in (0..chars.len()).step_by(6) {
        let b00 = char_to_number(chars[n]);
        let b01 = char_to_number(chars[n + 1]);

        let b10 = char_to_number(chars[n + 2]);
        let b11 = char_to_number(chars[n + 3]);

        let b20 = char_to_number(chars[n + 4]);
        let b21 = char_to_number(chars[n + 5]);

        let b0 = b00 << 2 | b01 >> 2;
        let b1 = ((b01 & 3) << 4) | b10;
        let b2 = b11 << 2 | b20 >> 2;
        let b3 = ((b20 & 3) << 4) | b21;

        let c0 = LOOKUP_TABLE_BASE64[b0];
        let c1 = LOOKUP_TABLE_BASE64[b1];
        let c2 = LOOKUP_TABLE_BASE64[b2];
        let c3 = LOOKUP_TABLE_BASE64[b3];
        base64 = base64 + format!("{c0}{c1}{c2}{c3}").as_str();
    }

    return base64;
}

const LOOKUP_TABLE_HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

const LOOKUP_TABLE_BASE64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/',
];

/// Finds a char in an array of chars, returning its position
///
/// It returns an `Option<usize>` with the position of the char.
/// If the char is not found, it returns `None`.
fn at(a: [char; 16], elem: char) -> Option<usize> {
    return a.iter().position(|&x| x == elem);
}

/// It panics if `value` is not in the lookup table
fn char_to_number(value: char) -> usize {
    return at(LOOKUP_TABLE_HEX, value).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = char_to_number('3');
        assert_eq!(result, 3);
    }

    #[test]
    fn it_works_for_gt_10() {
        let result = char_to_number('c');
        assert_eq!(result, 12);
    }

    #[test]
    #[should_panic]
    fn should_panic_with_invalid_char() {
        char_to_number('q');
    }

    #[test]
    fn it_works_with_three_chars() {
        let result = decode_base64(String::from("49276d").chars().collect());
        assert_eq!(result, "SSdt");
    }

    #[test]
    fn it_works_with_full_string() {
        let result = decode_base64(String::from("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d").chars().collect());
        assert_eq!(
            result,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
