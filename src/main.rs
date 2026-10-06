use rand::RngExt;
use std::fs::File;
use std::io::Write;

fn powers(x: u64) -> Vec<u64> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < 10 {
        result.push(x.pow(i));
        i += 1;
    }
    result
}

fn check_collatz(x: u64) -> bool {
    if x == 0 {
        return false;
    };

    if x == 1 {
        return true;
    };
    if x.is_multiple_of(2) {
        check_collatz(x / 2)
    } else {
        check_collatz(3 * x + 1)
    }
}

fn collatz(arr: Vec<u64>) -> Vec<bool> {
    let mut result = Vec::new();
    for &x in arr.iter() {
        result.push(check_collatz(x));
    }
    result
}

fn find_collatz_above_threshold(
    max_starting_number: u64,
    threshold: u64,
    max_iterations: usize,
) -> (u64, bool) {
    for starting_number in 1..=max_starting_number {
        let mut current_number = starting_number;

        for _ in 1..=max_iterations {
            if current_number.is_multiple_of(2) {
                current_number /= 2;
            } else {
                current_number = 3 * current_number + 1;
            }

            if current_number > threshold {
                return (starting_number, true);
            }

            if current_number == 1 {
                break;
            }
        }
    }

    (0, false)
}

fn main() {
    let parse_failed = loop {
        println!("Enter a number: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let x: u64 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => break true,
        };

        if x == 0 {
            break false;
        }

        println!("You entered: {}", x);
        let x = x + rand::rng().random_range(0..=5);
        println!("After adding a random number between 0 and 5: {}", x);
        let arr = powers(x);
        println!("The array of powers of {}: {:?}", x, arr);

        let collatz_results: Vec<bool> = collatz(arr);
        println!("Collatz results: {:?}", collatz_results);

        let mut file = File::create("xyz.txt").expect("Unable to create file");
        let text = format!("{:?}", collatz_results);
        file.write_all(text.as_bytes())
            .expect("Unable to write data");
    };

    if parse_failed {
        println!("Ended because of an error...");
    } else {
        println!("Exiting at the user's request...");
    }

    let collatz_search_result = find_collatz_above_threshold(100, 1_000, 100);
    println!("Final result: {:?}", collatz_search_result);
}
