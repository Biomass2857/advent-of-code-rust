pub mod dec1 {
    pub fn calibrate(text: &str) -> u32 {
        let mut result: u32 = 0;
        let mut has_first: bool = false;
        let mut last_digit = 0;
        for c in text.chars() {
            if c.is_digit(10) {
                let digit = match c.to_string().parse::<u32>() {
                    Ok(n) => n,
                    Err(_) => continue
                };

                if !has_first {
                    result += digit * 10;
                    has_first = true;
                }

                last_digit = digit;
                continue;
            }

            if c == '\n' {
                result += last_digit;
                has_first = false;
            }
        }

        if let Some(end_char) = text.chars().last() {
            if end_char != '\n' {
                result += last_digit;
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
            assert_eq!(result, 53651);
        }
    }
}