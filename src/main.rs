fn main() {
    println!("Hello, world!");
}

const lookup_table: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];

fn at(a: [char; 16], elem: char) -> Option<usize> {
    return a.iter().position(|&x| x == elem);
}
fn char_to_number(value: char) -> i8 {
    return match at(lookup_table, value) {
        Some(position) => position.try_into().unwrap(),
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = char_to_number('q');
        assert_eq!(result, -1);
    }
}