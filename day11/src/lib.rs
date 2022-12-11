use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, multispace1},
    multi::separated_list1,
    sequence::{delimited, preceded},
    *,
};

pub fn day11(input_file: String) {
    let file = std::fs::read_to_string(input_file).unwrap();

    println!("p1: {}", part1(&file));
    println!("p2: {}", part2(&file));
}

fn part1(file: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(file).unwrap();
    let magic_trick = monkeys.iter().map(|monkey| monkey.test.divisible).product::<u64>();

    for _round in 0..20 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = &mut monkeys[monkey_index];
                let item = monkey.process(true, magic_trick);
                let monkey_to_send_to = monkey.test(item);
                monkeys.get_mut(monkey_to_send_to as usize).unwrap().items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.touches);
    monkeys.iter().rev().take(2).map(|monkey| monkey.touches).product::<u64>().to_string()
}

fn part2(file: &str) -> String {
    let (_, mut monkeys) = separated_list1(tag("\n\n"), monkey)(file).unwrap();
    let magic_trick = monkeys.iter().map(|monkey| monkey.test.divisible).product::<u64>();

    for _round in 0..10_000 {
        for monkey_index in 0..monkeys.len() {
            for _ in 0..monkeys[monkey_index].items.len() {
                let monkey = &mut monkeys[monkey_index];
                let item = monkey.process(false, magic_trick);
                let monkey_to_send_to = monkey.test(item);
                monkeys.get_mut(monkey_to_send_to as usize).unwrap().items.push_back(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| monkey.touches);
    monkeys.iter().rev().take(2).map(|monkey| monkey.touches).product::<u64>().to_string()
}

///////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
enum Value {
    Old,
    Num(u64),
}

#[derive(Debug)]
enum Operation {
    Mul((Value, Value)),
    Add((Value, Value)),
}

#[derive(Debug)]
struct Test {
    divisible: u64,
    r#true: u64,
    r#false: u64,
}

#[derive(Debug)]
struct Monkey {
    test: Test,
    touches: u64,
    items: VecDeque<u64>,
    operation: Operation,
}

impl Monkey {
    fn process(&mut self, relief_lowers_worry_level: bool, magic_trick: u64) -> u64 {
        self.touches += 1;
        let item = self.items.pop_front().unwrap();

        let worry_level = match &self.operation {
            Operation::Mul((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                (num_a * num_b) % magic_trick
            },
            Operation::Add((a, b)) => {
                let num_a = match a {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };
                let num_b = match b {
                    Value::Old => item,
                    Value::Num(num) => *num,
                };

                (num_a + num_b) % magic_trick
            },
        };

        if relief_lowers_worry_level {
            worry_level / 3
        } else {
            worry_level
        }
    }

    fn test(&self, item: u64) -> u64 {
        if item % self.test.divisible == 0 {
            self.test.r#true
        } else {
            self.test.r#false
        }
    }
}

fn value(input: &str) -> IResult<&str, Value> {
    alt((
        tag("old").map(|_| Value::Old),
        nom::character::complete::u64.map(Value::Num),
    ))(input)
}

fn operation(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("Operation: new = ")(input)?;
    let (input, value_1) = value(input)?;
    let (input, operator) = delimited(multispace1, alt((tag("*"), tag("+"))), multispace1)(input)?;
    let (input, value_2) = value(input)?;

    Ok((input, match operator {
        "*" => Operation::Mul((value_1, value_2)),
        "+" => Operation::Add((value_1, value_2)),
        _ => unreachable!(),
    }))
}

fn test(input: &str) -> IResult<&str, Test> {
    let (input, divisible) = preceded(tag("Test: divisible by "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, r#true) = preceded(tag("If true: throw to monkey "), complete::u64)(input)?;
    let (input, _) = multispace1(input)?;
    let (input, r#false) = preceded(tag("If false: throw to monkey "), complete::u64)(input)?;

    Ok((input, Test {
        r#true,
        r#false,
        divisible,
    }))
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    let (input, _id) = delimited(tag("Monkey "), complete::u64, tag(":"))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, items) = preceded(
        tag("Starting items: "),
        separated_list1(tag(", "), nom::character::complete::u64),
    )(input)?;
    let (input, _) = multispace1(input)?;
    let (input, op) = operation(input)?;
    let (input, _) = multispace1(input)?;
    let (input, test) = test(input)?;

    Ok((input, Monkey {
        test,
        touches: 0,
        operation: op,
        items: VecDeque::from(items),
    }))
}
