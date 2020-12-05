pub fn validate_birth(input: &String) -> bool {
    match input.parse::<i32>() {
        Ok(num) => {
            if num >= 1920 && num <= 2002 {
                return true;
            }
            false
        }
        Err(_) => false
    }
}

pub fn validate_issue(input: &String) -> bool {
    match input.parse::<i32>() {
        Ok(num) => {
            if num >= 2010 && num <= 2020 {
                return true;
            }
            false
        }
        Err(_) => false
    }
}

pub fn validate_expr(input: &String) -> bool {
    match input.parse::<i32>() {
        Ok(num) => {
            if num >= 2020 && num <= 2030 {
                return true;
            }
            false
        }
        Err(_) => false
    }
}

pub fn validate_height(input: &String) -> bool {
    if input.len() < 3 { return false; }

    let units = &input[input.len() - 2..input.len()];
    match units {
        "cm" => {
            match input[..input.len() - 2].parse::<i32>() {
                Ok(num) => {
                    if num >= 150 && num <= 193 {
                        return true;
                    }
                    false
                }
                Err(_) => false
            }
        }
        "in" => {
            match input[..input.len() - 2].parse::<i32>() {
                Ok(num) => {
                    if num >= 59 && num <= 76 {
                        return true;
                    }
                    false
                }
                Err(_) => false
            }
        }
        _ => false
    }
}

fn is_hex_char(input: char) -> bool {
    if input == '0' ||
        input == '1' ||
        input == '2' ||
        input == '3' ||
        input == '4' ||
        input == '5' ||
        input == '6' ||
        input == '7' ||
        input == '8' ||
        input == '9' ||
        input == 'a' ||
        input == 'b' ||
        input == 'c' ||
        input == 'd' ||
        input == 'e' ||
        input == 'f'
    {
        return true;
    }
    false
}

pub fn validate_hair(input: &String) -> bool {
    if input.as_bytes()[0] as char != '#' { return false; }
    if input.len() != 7 { return false; }

    let color_string = &input[1..input.len()];
    if color_string.chars().all(|x| is_hex_char(x)) { return true; }
    false
}

pub fn validate_eye(input: &String) -> bool {
    if input == "amb" ||
        input == "blu" ||
        input == "brn" ||
        input == "gry" ||
        input == "grn" ||
        input == "hzl" ||
        input == "oth"
    {
        return true;
    }
    false
}

pub fn validate_pid(input: &String) -> bool {
    if input.len() != 9 { return false; }
    if input.chars().all(|x| x.is_numeric()) { return true; }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_birth() {
        assert_eq!(validate_birth(&"2002".to_string()), true);
        assert_eq!(validate_birth(&"1920".to_string()), true);
        assert_eq!(validate_birth(&"2003".to_string()), false);
        assert_eq!(validate_birth(&"1919".to_string()), false);
        assert_eq!(validate_birth(&"test".to_string()), false);
    }

    #[test]
    fn test_validate_issue() {
        assert_eq!(validate_issue(&"2010".to_string()), true);
        assert_eq!(validate_issue(&"2020".to_string()), true);
        assert_eq!(validate_issue(&"2009".to_string()), false);
        assert_eq!(validate_issue(&"2021".to_string()), false);
        assert_eq!(validate_issue(&"test".to_string()), false);
    }

    #[test]
    fn test_validate_expr() {
        assert_eq!(validate_expr(&"2020".to_string()), true);
        assert_eq!(validate_expr(&"2030".to_string()), true);
        assert_eq!(validate_expr(&"2009".to_string()), false);
        assert_eq!(validate_expr(&"2031".to_string()), false);
        assert_eq!(validate_expr(&"test".to_string()), false);
    }

    #[test]
    fn test_validate_height() {
        assert_eq!(validate_height(&"150cm".to_string()), true);
        assert_eq!(validate_height(&"193cm".to_string()), true);
        assert_eq!(validate_height(&"149cm".to_string()), false);
        assert_eq!(validate_height(&"194cm".to_string()), false);

        assert_eq!(validate_height(&"59in".to_string()), true);
        assert_eq!(validate_height(&"76in".to_string()), true);
        assert_eq!(validate_height(&"58in".to_string()), false);
        assert_eq!(validate_height(&"77in".to_string()), false);

        assert_eq!(validate_height(&"7a7in".to_string()), false);
        assert_eq!(validate_height(&"77".to_string()), false);
        assert_eq!(validate_height(&"145".to_string()), false);
    }

    #[test]
    fn test_validate_hair() {
        assert_eq!(validate_hair(&"#123456".to_string()), true);
        assert_eq!(validate_hair(&"#ffffff".to_string()), true);
        assert_eq!(validate_hair(&"#123919".to_string()), true);

        assert_eq!(validate_hair(&"123919".to_string()), false);
        assert_eq!(validate_hair(&"#######".to_string()), false);
        assert_eq!(validate_hair(&"#ao1234".to_string()), false);
        assert_eq!(validate_hair(&"#12345A".to_string()), false);
    }

    #[test]
    fn test_validate_eye() {
        assert_eq!(validate_eye(&"amb".to_string()), true);
        assert_eq!(validate_eye(&"blu".to_string()), true);
        assert_eq!(validate_eye(&"brn".to_string()), true);
        assert_eq!(validate_eye(&"gry".to_string()), true);
        assert_eq!(validate_eye(&"grn".to_string()), true);
        assert_eq!(validate_eye(&"hzl".to_string()), true);
        assert_eq!(validate_eye(&"oth".to_string()), true);


        assert_eq!(validate_eye(&"aaa".to_string()), false);
        assert_eq!(validate_eye(&"AMB".to_string()), false);
    }

    #[test]
    fn test_validate_passport() {
        assert_eq!(validate_pid(&"000000000".to_string()), true);
        assert_eq!(validate_pid(&"000000001".to_string()), true);
        assert_eq!(validate_pid(&"999999999".to_string()), true);

        assert_eq!(validate_pid(&"aaaaaaaaa".to_string()), false);
        assert_eq!(validate_pid(&"9999999999".to_string()), false);
        assert_eq!(validate_pid(&"-111111111".to_string()), false);
    }
}
