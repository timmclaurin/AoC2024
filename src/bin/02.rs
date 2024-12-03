advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let line = input.lines();

    let mut safe = 0;
    let mut line_counter = 0;
    for l in line {
        line_counter += 1;
        let v_digits: Vec<&str> = l.split_whitespace().collect();
        println!("{}: {}", line_counter, l);

        let valid = process_line(&v_digits);

        // Good: 66 67 68 71 72 69
        // Bad:  38 40 41 42 44 47 48 52

        if valid.0 {
            print!("  <== Good");
            safe += 1;
        } else {
            print!("  <== Bad");
        }
        println!("");
    }

    return Some(safe as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    println!("Starting Part Two");
    let line = input.lines();

    let mut safe = 0;
    let mut line_counter = 0;
    for l in line {
        line_counter += 1;
        let v_digits: Vec<&str> = l.split_whitespace().collect();
        println!("{}: {}", line_counter, l);

        let mut ret = process_line(&v_digits);

        // Good: 66 67 68 71 72 69
        // Bad:  38 40 41 42 44 47 48 52

        if ret.0 {
            print!("  <== Good");
            safe += 1;
        } else {
            let inital_error = ret.1.clone();
            println!(
                "\n\tError at position: {}: {}",
                inital_error, v_digits[inital_error]
            );
            let mut new_v_digits = v_digits.clone();
            // Just remove the failing digit and try again
            println!(
                "\n\tRemoving position of error: {}: {}",
                ret.1, new_v_digits[inital_error]
            );
            new_v_digits.remove(inital_error);
            println!("\t{} Now: {}", line_counter, new_v_digits.join(" "));
            ret = process_line(&new_v_digits);

            if ret.0 {
                print!("  <== Good");
                safe += 1;
            } else {
                if inital_error == 0 {
                    continue;
                }
                new_v_digits = v_digits.clone();
                println!(
                    "\n\tRemoving position before error: {}: {}",
                    inital_error,
                    new_v_digits[inital_error - 1]
                );
                //remove the prior digit and see if we are any better
                new_v_digits.remove(inital_error - 1);

                println!("\t{} Now: {}", line_counter, new_v_digits.join(" "));
                ret = process_line(&new_v_digits);
                if ret.0 {
                    print!("  <== Good");
                    safe += 1;
                } else {
                    if inital_error == new_v_digits.len() - 1 {
                        continue;
                    }
                    new_v_digits = v_digits.clone();
                    println!(
                        "\n\tRemoving position after error: {}: {}",
                        inital_error + 1,
                        new_v_digits[inital_error + 1]
                    );
                    //remove the next digit and see if we are any better
                    new_v_digits.remove(inital_error + 1);
                    println!("{}: {}", line_counter, new_v_digits.join(" "));
                    ret = process_line(&new_v_digits);
                    if ret.0 {
                        print!("  <== Good");
                        safe += 1;
                    } else {
                        if inital_error < 2 {
                            continue;
                        }
                        new_v_digits = v_digits.clone();
                        println!(
                            "\n\tRemoving position after error: {}: {}",
                            inital_error - 2,
                            new_v_digits[inital_error - 2]
                        );
                        //remove the next digit and see if we are any better
                        new_v_digits.remove(inital_error - 2);
                        println!("{}: {}", line_counter, new_v_digits.join(" "));
                        ret = process_line(&new_v_digits);
                        if ret.0 {
                            print!("  <== Good");
                            safe += 1;
                        } else {
                            print!("  <== Bad");
                        }
                    }
                }
            }
        }
        println!("");
        // if line_counter == 20 {
        //     break;
        // }
    }

    return Some(safe as u32);
}

fn process_line(digits: &Vec<&str>) -> (bool, usize) {
    let mut prior_digit = -1;
    // 0 = up, 1 = down
    let mut direction = -1;
    for (i, d) in digits.iter().enumerate() {
        let digit = d.parse::<i32>().unwrap();
        // let mut we_still_good = true;

        if prior_digit != -1 {
            if digit != prior_digit {
                // Are we increasing?
                if digit > prior_digit {
                    if direction == -1 {
                        direction = 0;
                        print!("/");
                    } else if direction == 1 {
                        // We were decreasing
                        print!("*");
                        return (false, i);
                    } else {
                        print!("/");
                    }
                }

                // Are we decreasing?
                if digit < prior_digit {
                    if direction == -1 {
                        direction = 1;
                        print!("\\");
                    } else if direction == 0 {
                        // We were increasing
                        print!("*");
                        return (false, i);
                    } else {
                        print!("\\");
                    }
                }

                if (digit - prior_digit).abs() > 3 {
                    print!("*");
                    return (false, i);
                }
            } else {
                print!("=");
                return (false, i);
            }
        } else {
            print!("\t");
        }
        print!("{}", d);

        prior_digit = digit;
    }
    return (true, 0);
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

// for (i, d) in digits.iter().enumerate() {
//     if i == digits.len() - 1 {
//         // skip the last digit
//         break;
//     }
//     let next = digits.get(i + 1).unwrap_or(&d);

//     if d == next {
//         // if the current digit is the same as the next digit, skip
//         valid = false;
//         break;
//     }

//     if (next < d || next > d) && (next - d).abs() < 4 {
//         valid = true;
//     } else {
//         valid = false;
//     }
// }
