use std::fs;

fn main() {
        let mut total = 0;
        let mut lineN = 0;
        let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];



        for line in fs::read_to_string("input.txt").unwrap().lines() {
            let mut found = false;
            let mut found2 = false;
            let mut a = 0;
            let mut b = 0;
            let mut chars = vec![];
            for i in line.chars() {
                if i.is_numeric() {
                    chars = vec![];
                    if !found {
                        a = i.to_string().parse::<i32>().unwrap();
                        found = true;
                    } else {
                        b = i.to_string().parse::<i32>().unwrap();
                        found2 = true
                    }
                } else {
                    chars.push(i);

                    let mut index = 1;
                    for word in words {
                        if chars.clone().into_iter().collect::<String>().contains(word) {
                          //  println!("{}", line.clone());
                          //  println!("{}", chars.clone().into_iter().collect::<String>());
                            if !found {
                                a = index;
                                found = true;
                            } else {
                                b = index;
                                found2 = true
                            }
                            chars.drain(0..chars.len()-1);
                        }
                        index += 1;
                    }
                }
            }

            println!("Line {lineN}: {a} - {b}");
            if found2 {
                total += (a * 10) + b;
            } else {
                total += (a * 10) + a;
            }
            lineN += 1;
        }

    println!("result: {total}")
}