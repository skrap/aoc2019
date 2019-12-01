fn main() {
    let lines = include_str!("input_1a.txt");
    println!(
        "total just modules: {}",
        lines
            .lines()
            .map(|s| s.parse::<i32>().unwrap() / 3 - 2)
            .sum::<i32>()
    );
    println!(
        "total with fuel: {}",
        lines
            .lines()
            .map(|s| {
                let mut total = s.parse::<i32>().unwrap() / 3 - 2;
                let mut extra_fuel = total / 3 - 2;
                loop {
                    if extra_fuel > 0 {
                        total += extra_fuel;
                        extra_fuel = extra_fuel / 3 - 2;
                    } else {
                        break;
                    }
                }
                total
            })
            .sum::<i32>()
    );
}
