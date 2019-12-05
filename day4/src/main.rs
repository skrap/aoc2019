fn main() {
    dbg!(check(111111));
    dbg!(check(223450));
    dbg!(check(123789));

    let mut count = 0;
    for number in 372304..=847060 {
        if check(number) {
            count += 1;
        }
    }
    dbg!(count);

    dbg!(check_part2(112233));
    dbg!(check_part2(123444));
    dbg!(check_part2(111122));

    let mut count = 0;
    for number in 372304..=847060 {
        if check_part2(number) {
            count += 1;
        }
    }
    dbg!(count);
}

fn check(number: isize) -> bool {
    let digits = [
        number / 100_000,
        (number / 10000) % 10,
        (number / 1000) % 10,
        (number / 100) % 10,
        (number / 10) % 10,
        number % 10,
    ];

    if !digits.windows(2).any(|window| window[0] == window[1]) {
        return false;
    }

    if !digits.windows(2).all(|window| window[0] <= window[1]) {
        return false;
    }

    return true;
}


fn check_part2(number: isize) -> bool {
    let digits = [
        number / 100_000,
        (number / 10000) % 10,
        (number / 1000) % 10,
        (number / 100) % 10,
        (number / 10) % 10,
        number % 10,
    ];

    if !digits.windows(2).all(|window| window[0] <= window[1]) {
        return false;
    }

    let mut in_a_row = 1;
    let mut last_digit = digits[0];
    for digit in &digits[1..] {
        if last_digit == *digit {
            in_a_row += 1;
        } else if in_a_row == 2 {
            return true;
        } else {
            in_a_row = 1;
        }
        last_digit = *digit;
    }

    in_a_row == 2
}
