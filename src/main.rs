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

    fn increment_string_best_practice(s: &str) -> String {
        if let Some(last) = s.chars().last() {
            match last.to_digit(10) {
                Some(9) => format!("{}0", &increment_string_best_practice(&s[..s.len() - 1])),
                Some(num) => format!("{}{}", &s[..s.len() - 1], num + 1),
                None => format!("{s}1"),
            }
        } else {
            format!("1")
        }
    }

    #[test]
    fn test_increment_string() {
        let result = increment_string("99999999");
        let result2 = increment_string_best_practice("asdasd223123flls01");
        println!("{result}");
        println!("{result2}");
    }

    fn make_looper(string: &str) -> impl FnMut() -> char + '_ {
        let mut counter = 0;
        move || {
            let mut iter = string.chars();
            let result;
            if counter <= string.len() - 1 {
                result = iter.nth(counter).unwrap();
                counter += 1;
            } else {
                counter = 0;
                result = iter.nth(counter).unwrap();
                counter += 1;
            }
            result
        }
    }

    fn best_practice_make_looper(string: &str) -> impl FnMut() -> char + '_ {
        let mut it = string.chars().cycle();
        move || it.next().unwrap()
    }

    #[test]
    fn test_make_looper() {
        let mut abc = make_looper("abc");
        assert_eq!(abc(), 'a');
        assert_eq!(abc(), 'b');
        assert_eq!(abc(), 'c');
        assert_eq!(abc(), 'a');
        assert_eq!(abc(), 'b');
        assert_eq!(abc(), 'c');
    }

    #[test]
    fn test_bp_make_looper() {
        let mut abc = best_practice_make_looper("abc");
        assert_eq!(abc(), 'a');
        assert_eq!(abc(), 'b');
        assert_eq!(abc(), 'c');
        assert_eq!(abc(), 'a');
        assert_eq!(abc(), 'b');
        assert_eq!(abc(), 'c');
    }

    // https://www.codewars.com/kata/5511b2f550906349a70004e1/train/rust
    // Last Digit
    fn last_digit(str1: &str, str2: &str) -> i32 {
        let left = str1.chars().last().unwrap().to_digit(10).unwrap() as i32;
        let check = str2.chars().rev().skip_while(|&c| c == '0').next();
        let mut right: u32;
        if let Some(x) = check {
            if str2.chars().last().unwrap() == '0' {
                right = x.to_digit(10).unwrap() * 10;
            } else {
                let new_right = str2
                    .chars()
                    .rev()
                    .take(2)
                    .collect::<String>()
                    .chars()
                    .rev()
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap();
                right = new_right;
            }
        } else {
            return 1;
        }

        match left {
            2 | 3 | 7 | 8 => {
                right %= 4;
                if right == 0 {
                    right = 4;
                }
                left.pow(right) % 10
            }
            _ => {
                right %= 10;
                println!("{right}");
                if right == 0 {
                    right = 2;
                }
                left.pow(right) % 10
            }
        }
    }

    fn bp_last_digit(str1: &str, str2: &str) -> i32 {
        if str2 == "0" {
            return 1;
        }
        let x = str1.chars().last().unwrap().to_digit(10).unwrap();
        let m = str2
            .chars()
            .fold(0, |a, x| (a * 10 + x.to_digit(10).unwrap()) % 4);
        let exp = if m == 0 { 4 } else { m };
        (x.pow(exp) % 10) as i32
    }

    #[test]
    fn test_last_digit() {
        let result = last_digit("4", "7");
        assert_eq!(result, 4);
        assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376", "2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
        assert_eq!(
            last_digit(
                "3715290469715693021198967285016729344580685479654510946723",
                "68819615221552997273737174557165657483427362207517952651"
            ),
            7
        );
        assert_eq!(bp_last_digit("3715290469715693021198967285016729344580685479654510946723", "68819615221552997273737174557165657483427362207517952651"), 7);
    }
}
