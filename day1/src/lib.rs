pub fn day1(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();
    part1(&file);
    part2(&file);
}

fn part1(file: &str) {
    let max = file
        .split("\n\n")
        .map(|elf_load| elf_load.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();

    println!("The elves with the most calories are: {:?}", max);
}

fn part2(file: &str) {
    let mut elves = file
        .split("\n\n")
        .map(|elf_load| elf_load.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    elves.sort_by(|a, b| b.cmp(a));

    println!(
        "The elves with the most calories are: {:?}",
        elves.iter().take(3).sum::<u32>()
    );
}
