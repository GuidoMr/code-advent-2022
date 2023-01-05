fn main() {
    let mut input = include_str!("../input.txt")
        .split("\r\n\r\n")
        .map(|x| {
            x.lines()
                .map(|y| {
                    y.parse::<u32>()
                        .expect("failed to parse input values to u32")
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    input.sort_unstable();

    println!("{}", input.iter().rev().take(3).sum::<u32>());
}
