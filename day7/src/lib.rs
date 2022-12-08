#![feature(iter_intersperse)]

use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    *,
};

pub fn day7(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(input: &str) -> String {
    get_dir_sizes(input)
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let sizes = get_dir_sizes(input);

    let total_size = 70_000_000;
    let needed_space = 30_000_000;
    let used_space = sizes.get("").unwrap();
    let current_free_space = total_size - used_space;
    let need_to_free_at_least = needed_space - current_free_space;

    let mut dirs = sizes
        .iter()
        .filter(|(_, &size)| size >= need_to_free_at_least)
        .map(|(_, size)| size)
        .collect::<Vec<_>>();
    dirs.sort();
    dirs.first().unwrap().to_string()
}

fn get_dir_sizes(input: &str) -> BTreeMap<String, u32> {
    let (_, cmds) = commands(input).unwrap();
    let mut directories: BTreeMap<String, Vec<File>> = BTreeMap::new();
    let mut context: Vec<&str> = vec![];
    for command in cmds.iter() {
        match command {
            Operation::Cd(cd_opt) => match cd_opt {
                Cd::Up => {
                    context.pop();
                },
                Cd::Root => {
                    context.push("");
                },
                Cd::Down(path) => {
                    context.push(path);
                },
            },
            Operation::Ls(files) => {
                directories
                    .entry(context.iter().cloned().intersperse("/").collect::<String>())
                    .or_insert(vec![]);

                for file in files {
                    match file {
                        Files::File { size } => {
                            directories
                                .entry(context.iter().cloned().intersperse("/").collect::<String>())
                                .and_modify(|v| v.push(File { size: *size }));
                        },
                        Files::Dir(_) => (),
                    }
                }
            },
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split('/').collect::<Vec<&str>>();
        let size = files.iter().map(|File { size, .. }| size).sum::<u32>();

        for i in 0..dirs.len() {
            sizes
                .entry(dirs[0..=i].iter().cloned().intersperse("/").collect::<String>())
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    sizes
}

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Up,
    Root,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    Dir(&'a str),
    File { size: u32 },
}

#[derive(Debug)]
struct File {
    size: u32,
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, _)) =
        separated_pair(complete::u32, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;
    Ok((input, Files::File { size }))
}

fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dir_name) = alpha1(input)?;
    Ok((input, Files::Dir(dir_name)))
}

fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}

fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        path => Operation::Cd(Cd::Down(path)),
    };
    Ok((input, op))
}

fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;
    Ok((input, cmd))
}
