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

pub fn day4(start: i32, end: i32) -> i32 {
    let mut count = 0;
    for i in start..(end + 1) {
        if valid(i) {
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
        assert_ne!(true, valid(223450));
        assert_ne!(true, valid(123789));
    }
}
