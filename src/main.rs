fn main() {
    let test = "Hello099";
    let first: String = test
        .chars()
        .into_iter()
        .filter(|x| !x.is_ascii_digit())
        .collect();

    let second: String = test
        .chars()
        .into_iter()
        .filter(|x| x.is_ascii_digit())
        .collect();

    let mut second_i32: i32 = second.parse().unwrap();
    second_i32 += 1;
    let mut result = String::new();
    if second.contains("0") && second_i32 % 10 != 0 {
        let sec: String = second.chars().into_iter().filter(|x| *x == '0').collect();
        result = first + &sec + &second_i32.to_string();
    } else {
        result = first + &second_i32.to_string();
    }

    println!("{result}");
}

#[cfg(test)]
mod test {

    // https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust
    // String Incrementer
    fn increment_string(s: &str) -> String {
        let result: String;
        let first: String = s
            .chars()
            .into_iter()
            .filter(|x| !x.is_ascii_digit())
            .collect();

        let second: String = s
            .chars()
            .into_iter()
            .filter(|x| x.is_ascii_digit())
            .collect();

        let mut second_i32: i32 = second.parse().unwrap();
        second_i32 += 1;

        if second.contains('0') && second_i32 % 10 != 0 {
            let sec: String = second.chars().into_iter().filter(|x| *x == '0').collect();
            result = first + &sec + &second_i32.to_string();
            result
        } else {
            result = first + &second_i32.to_string();
            result
        }
    }

    #[test]
    fn test_increment_string() {
        let result = increment_string("foo0099");
        println!("{result}");
    }
}
