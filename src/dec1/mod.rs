pub mod dec1 {
    trait OptionMapable<T> {
        fn to_option(&self) -> Option<&T>;
    }

    impl<T, E> OptionMapable<T> for Result<T, E> {
        fn to_option(&self) -> Option<&T> {
            match self {
                Ok(value) => Some(value),
                Err(_) => None
            }
        }
    }

    fn contains_digit_word(str: &str) -> Option<u32> {
        const DIGIT_WORDS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
        for (index, word) in DIGIT_WORDS.iter().enumerate() {
            if str.contains(word) {
                return Some(index as u32 + 1);
            }
        }

        None
    }

    pub fn calibrate(text: &str) -> u32 {
        let mut result: u32 = 0;
        for line in text.split("\n") {
            let mut current_sub_string_front = String::from("");
            for c in line.chars() {
                current_sub_string_front.push_str(&c.to_string());

                match c.to_string().parse::<u32>().to_option().or(contains_digit_word(&current_sub_string_front).as_ref()) {
                    Some(digit) => {
                        result += *digit * 10;
                        break;
                    },
                    None => {}
                }
            }

            let mut current_sub_string_back = String::from("");

            for c in line.chars().rev() {
                current_sub_string_back.insert_str(0, &c.to_string());

                match c.to_string().parse::<u32>().to_option().or(contains_digit_word(&current_sub_string_back).as_ref()) {
                    Some(digit) => {
                        result += digit;
                        break;
                    },
                    None => {}
                }
            }
        }

        result
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn works() {
            let result = calibrate("1hello2world3somewords\nhewbbewh7hjhewhewhjdj8hjjwehweehwj9hejewh1dde");
            assert_eq!(result, 84);
        }


        use std::fs;

        #[test]
        fn calc_value() {
            let text = fs::read_to_string("input_dec1.txt").unwrap();
            let result = calibrate(&text);
            assert_eq!(result, 53894);
        }
    }
}