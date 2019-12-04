fn valid(num: i32) -> bool {
    let digits: Vec<char> = num.to_string().chars().collect();
    let mut has_double = false;
    for i in 0..(digits.len() - 1) {
        if digits[i] == digits[i + 1] {
            has_double = true;
        }
        if digits[i] > digits[i + 1] {
            return false;
        }
    }
    has_double
}

fn valid2(num: i32) -> bool {
    let digits: Vec<char> = num.to_string().chars().collect();
    for i in 0..(digits.len() - 1) {
        if digits[i] > digits[i + 1] {
            return false;
        }
    }
    for i in 0..(digits.len() - 1) {
        let prev = if i == 0 { None } else { digits.get(i - 1) };
        let this = digits.get(i);
        let next = digits.get(i + 1);
        let next2 = digits.get(i + 2);
        if prev != this && this == next && next != next2 {
            return true;
        }
    }
    false
}

pub fn day4(start: i32, end: i32) -> i32 {
    let mut count = 0;
    for i in start..(end + 1) {
        if valid2(i) {
            count += 1;
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        assert_eq!(true, valid(122345));
        assert_eq!(true, valid(111111));
        assert_eq!(false, valid(223450));
        assert_eq!(false, valid(123789));
    }

    #[test]
    fn test_valid2() {
        assert_eq!(true, valid2(112233));
        assert_eq!(false, valid2(123444));
        assert_eq!(true, valid2(111122));
    }
}
