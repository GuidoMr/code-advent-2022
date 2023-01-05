fn main() {
    println!(
        "{}",
        include_str!("../input.txt")
            .split("\r\n\r\n")
            .map(|x| {
                x.lines()
                    .map(|y| {
                        y.parse::<u32>()
                            .expect("failed to parse input values to u32")
                    })
                    .sum::<u32>()
            })
            .max()
            .unwrap()
    );
}
