pub fn int_parts(whole: i32) -> Vec<i32> {
    let mut whole = whole;
    let mut ints: Vec<i32> = Vec::new();
    while whole >= 10 {
        ints.push(whole % 10);
        whole /= 10;
    }
    ints.push(whole);
    ints.reverse();
    ints
}

pub fn repeat(candidate: i32) -> bool {
    let mut len = 0;
    let mut last = 0;

    for individual in int_parts(candidate) {
        if len == 0 {
            last = individual;
            len = 1;
            continue;
        }

        if individual == last {
            len += 1;
        } else if len == 2 {
            return true;
        } else {
            last = individual;
            len = 1;
        }
    }
    len == 2
}

pub fn increasing(candidate: i32) -> bool {
    let mut min = 0;
    for individual in int_parts(candidate) {
        if individual >= min {
            min = individual;
        } else {
            return false;
        }
    }
    true
}

// It is a six-digit number.
// The value is within the range given in your puzzle input.
// Two adjacent digits are the same (like 22 in 122345).
// Going from left to right, the digits never decrease; they only ever increase or stay the same
// (like 111123 or 135679).
//153517-630395

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(int_parts(554321), vec![5, 5, 4, 3, 2, 1]);

        assert_eq!(repeat(112233), true);
        assert_eq!(repeat(123444), false);
        assert_eq!(repeat(111122), true);

        assert_eq!(increasing(11), true);
        assert_eq!(increasing(12), true);
        assert_eq!(increasing(123455), true);
        assert_eq!(increasing(554321), false);
    }

    #[test]
    fn test_ten() {
        assert_eq!(int_parts(10), vec![1, 0]);
    }
}
