pub fn day1(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();
    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(file: &str) -> String {
    file.split("\n\n")
        .map(|elf_load| elf_load.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap()
        .to_string()
}

fn part2(file: &str) -> String {
    let mut elves = file
        .split("\n\n")
        .map(|elf_load| elf_load.lines().map(|item| item.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    elves.sort_by(|a, b| b.cmp(a));
    elves.iter().take(3).sum::<u32>().to_string()
}
