extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::{
    fs::read_to_string,
    io::Error as IoError,
};

use docopt::Docopt;

static USAGE: &str = "
Usage:
advent_of_code_2018 <day> <challenge>

Options:
    day        The day of the advent
    challenge  The challenge for that day
";

#[derive(Deserialize)]
struct Opt {
    pub arg_day: usize,
    pub arg_challenge: usize,
}

fn main() {
    let args: Opt = Docopt::new(USAGE)
                .and_then(|d| d.deserialize())
                .unwrap_or_else(|e| e.exit());
    if args.arg_day > 25 {
        eprintln!("An advent is only 25 days long");
        return;
    }
    if args.arg_challenge > 2 || args.arg_challenge < 1 {
        eprintln!("Each day only has 2 challenges");
    }
    let input = get_input(args.arg_day).unwrap();
    match args.arg_day {
        1 => day_one(&input, args.arg_challenge),
        2 => day_two(&input, args.arg_challenge),
        _ => eprintln!("Haven't gotten to that day yet..."),
    }

}

fn get_input(day: usize) -> Result<String, IoError>  {
    let path = format!("inputs/{}.txt", day);
    read_to_string(path)
}


fn day_one(input: &str, challenge: usize) {
    fn challenge_one(input: &str) {
        let mut answer: isize = 0;
        for line in input.lines() {
            let parsed: isize = line.parse().unwrap();
            answer += parsed;
        }
        println!("The answer is {}", answer);
    }
    fn challenge_two(input: &str) {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut answer: isize = 0;
        set.insert(answer);
        loop {
            for line in input.lines() {
                let parsed: isize = line.parse().unwrap();
                answer += parsed;
                if !set.insert(answer) {
                    println!("The answer is {}", answer);
                    return;
                }
            }
        }
    }
    if challenge == 1 {
        challenge_one(input);
    } else {
        challenge_two(input);
    }
}

fn day_two(input: &str, challenge: usize) {
    fn challenge_one(input: &str) {
        use std::collections::HashMap;
        let mut twos = 0;
        let mut threes = 0;
        for line in input.lines() {
            println!("{}", line);
            let mut  map = HashMap::new();
            for ch in line.chars() {
                let ct = map.entry(ch).or_insert(0);
                *ct += 1;
            }
            map.retain(|_, ct| *ct == 2 || *ct == 3);
            let mut found_two = false;
            let mut found_three = false;
            for (ch, val) in map.iter() {
                println!("{}: {}", ch, val);
                if *val == 2 && !found_two {
                    twos += 1;
                    found_two = true
                }
                if *val == 3 && !found_three {
                    threes += 1;
                    found_three = true;
                }
                if found_two && found_three {
                    break;
                }
            }
        }
        println!("The answer is: {} * {} = {}", twos, threes, twos * threes);
    }

    fn challenge_two(input: &str) {
        // use std::collections::HashMap;
        // let mut first_char_matches = HashMap::new();
            'outer: for (i, line) in input.lines().enumerate() {
                println!("checking line {}", i);
                'inner: for (j, second) in input.lines().enumerate() {
                    if i == j {
                        continue 'inner;
                    }
                    let mut diff_count = 0;
                    'chars: for (lhs, rhs) in line.chars().zip(second.chars()) {
                        if lhs != rhs {
                            diff_count += 1;
                        }
                        if diff_count > 1 {
                            break 'chars;
                        }
                    }
                    if diff_count == 1 {
                        let answer: String = line.chars().zip(second.chars()).filter_map(|(lhs, rhs)| if lhs == rhs {
                            Some(lhs)
                        } else {
                            None
                        }).collect();
                        println!("The answer is {}", answer);
                        return;
                    }
                }
            }
    }
    if challenge == 1 {
        challenge_one(input);
    } else {
        challenge_two(input);
    }
}
