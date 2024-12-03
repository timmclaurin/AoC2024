advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let line = input.lines();
    let mut col_a: Vec<i32> = Vec::new();
    let mut col_b: Vec<i32> = Vec::new();
    for l in line {
        let mut split = l.split("   ");
        let a = split.next().unwrap().parse::<i32>().unwrap();
        let b = split.next().unwrap().parse::<i32>().unwrap();
        col_a.push(a);
        col_b.push(b);
    }
    col_a.sort();
    col_b.sort();

    let mut delta = 0;
    for (a, b) in col_a.iter().zip(col_b.iter()) {
        delta += (a - b).abs();
    }
    let result = delta as u32;

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u32> {
    let line = input.lines();
    let mut col_a: Vec<i32> = Vec::new();
    let mut col_b: Vec<i32> = Vec::new();
    for l in line {
        let mut split = l.split("   ");
        let a = split.next().unwrap().parse::<i32>().unwrap();
        let b = split.next().unwrap().parse::<i32>().unwrap();
        col_a.push(a);
        col_b.push(b);
    }

    let mut all_count = 0;
    for a in col_a.iter() {
       let count =  col_b.iter().filter(|n| *n == a).count();
     //  print!(concat!("count: ", "{}:{}", "\n"), a, count);
       all_count += count as i32 * a;
    }

    return Some(all_count as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
