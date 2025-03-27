use std::env;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let string_to_decode = &args[1];
    // let chars: Vec<char> = string_to_decode.chars().collect();
    // let decoded = decode_base64(chars);
    println!("Decoded: [{}]", one());
}

fn one() -> usize {
    return 1;
}


#[cfg(test)]
mod tests {
    use super::one;

    #[test]
    fn it_works() {
        let result = one();
        assert_eq!(result, 1);
    }
}
