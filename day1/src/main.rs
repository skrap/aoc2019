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
                let mut total = 0;
                let mut load = s.parse::<i32>().unwrap() / 3 - 2;
                while load > 0 {
                    total += load;
                    load = load / 3 - 2;
                }
                total
            })
            .sum::<i32>()
    );
}
