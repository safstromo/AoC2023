use std::num;

fn main() {
    let input = include_str!("../input.txt");
    part1(input);
}

fn part1(input: &str) -> i32 {
    let chars: Vec<_> = input.lines().map(|s| s.chars()).collect();

    let mut numbers = vec![];

    for array in chars.iter() {
        let num: Vec<_> = array
            .clone()
            .into_iter()
            .filter(|c| c.is_numeric())
            .collect();

        numbers.push(num);
    }
    let mut strnums = vec![];

    for array in numbers.into_iter() {
        let first = array.first().expect("No first char");
        let last = array.last().expect("No last char");

        strnums.push(first.to_string() + &last.to_string());
    }

    let total: i32 = strnums.into_iter().map(|s| s.parse::<i32>().unwrap()).sum();

    dbg!(&total);
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = part1(input);
        assert_eq!(result, 142);
    }
}
