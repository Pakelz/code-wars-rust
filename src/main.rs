fn main() {}

#[cfg(test)]
mod test {

    // https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust
    // String Incrementer
    fn increment_string(s: &str) -> String {
        let mut iter = s.chars().rev().peekable();
        let mut result = String::new();
        let mut first = true;
        let mut count = 0;
        let mut check = true;
        let mut new_digit = 0;
 
        if let Some(last) = s.chars().last() {
            if !last.is_ascii_digit() {
                return format!("{s}1");
            }
        } else {
            return format!("1");
        }

        while let Some(char) = iter.next() {
            if char.is_ascii_digit() {
                if let Some(next_char) = iter.peek() {
                    if (next_char.is_ascii_digit()) && first && check {
                        new_digit = char.to_digit(10).unwrap() + 1;
                        result.push_str(&(new_digit % 10).to_string());
                        if new_digit >= 10 {
                            count = 1;
                        }
                        first = false;
                    } else if (!next_char.is_ascii_digit()) && first && check {
                        new_digit = char.to_digit(10).unwrap() + 1;
                        let rev: String = new_digit.to_string().chars().rev().collect();
                        result.push_str(&rev);
                        if new_digit >= 10 {
                            count = 1;
                        }
                        first = false;
                        check = false;
                    } else if next_char.is_ascii_digit() && check {
                        new_digit = char.to_digit(10).unwrap() + count;
                        let rev: String = (new_digit % 10).to_string().chars().rev().collect();
                        result.push_str(&rev);
                        if new_digit >= 10 {
                            count = 1;
                        } else {
                            count = 0;
                        }
                    } else if !next_char.is_ascii_digit() && check {
                        new_digit = char.to_digit(10).unwrap() + count;
                        let rev: String = new_digit.to_string().chars().rev().collect();
                        result.push_str(&rev);
                        check = false;
                    } else {
                        result.push(char);
                    }
                } else {
                    // Masalah
                    if (new_digit >= 10 || s.len() == 1) && check {
                        let value = char.to_digit(10).unwrap() + 1;
                        let rev: String = value.to_string().chars().rev().collect();
                        result.push_str(&rev);
                        println!("{new_digit}");
                    } else {
                        result.push(char);
                    }
                }
            } else {
                result.push(char);
            }
        }

        result = result.chars().rev().collect();
        result
    }

    #[test]
    fn test_increment_string() {
        let result = increment_string("99999999");
        println!("{result}");
    }
}
