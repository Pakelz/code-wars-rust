fn main() {
    let test = "foo98";
    let mut check = true;
    let mut counter = 0;
    let mut result = String::new();

    for i in test.chars().rev() {
        if i.is_ascii_digit() {
            if check {
                let new_digit = i.to_digit(10).unwrap() + 1;
                result.push_str(&(new_digit % 10).to_string());
                if new_digit >= 10 {
                    counter = 1;
                }
                check = false;
            } else {
                let new_digit = i.to_digit(10).unwrap() + counter;
                result.push_str(&(new_digit % 10).to_string());
                if new_digit < 10 {
                    counter = 0;
                } else if new_digit >= 10 {
                    counter = 1;
                }
            }
        } else {
            check = true;
            result.push(i);
        }
    }

    let mut result: String = result.chars().rev().collect();
    if result.chars().any(|c| c.is_ascii_digit()) {
        println!("{result}");
    } else {
        result = result + "1";
        println!("{result}");
    }
}

#[cfg(test)]
mod test {

    // https://www.codewars.com/kata/54a91a4883a7de5d7800009c/train/rust
    // String Incrementer
    fn increment_string(s: &str) -> String {
        let mut check = true;
        let mut counter = 0;
        let mut result = String::new();

        for i in s.chars().rev() {
            if i.is_ascii_digit() {
                if check {
                    let new_digit = i.to_digit(10).unwrap() + 1;
                    result.push_str(&(new_digit % 10).to_string());
                    if new_digit >= 10 {
                        counter = 1;
                    }
                    check = false;
                } else {
                    let new_digit = i.to_digit(10).unwrap() + counter;
                    result.push_str(&(new_digit % 10).to_string());
                    if new_digit < 10 {
                        counter = 0;
                    } else if new_digit >= 10 {
                        counter = 1;
                    }
                }
            } else {
                check = true;
                result.push(i);
            }
        }

        let mut result: String = result.chars().rev().collect();
        if result.chars().any(|c| c.is_ascii_digit()) {
            result
        } else {
            result = result + "1";
            result
        }
    }

    #[test]
    fn test_increment_string() {
        let result = increment_string("foo0099");
        println!("{result}");
    }
}
