use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    let generated = rng.gen_range(0..=100);

    let max_try = 6;
    let mut current_try = 0;

    println!("guess the number");
    loop {
        println!("enter a number between 0 and 100");

        // input
        let input = input();
        let input = match input {
            Some(value) => value,
            None => {
                println!("invalid input, expected a number.");
                continue;
            }
        };

        if evaluation(input, generated) {
            break;
        }
        current_try += 1;
        if current_try >= max_try {
            println!("too many tries. you lost");
            println!("the correct number was: {}", generated);
            break;
        } else {
            println!("remaining tries: {}", max_try - current_try);
        }
    }
}

fn input() -> Option<i32> {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("expected input from console");

    input.trim().parse().ok()
}

fn evaluation(input: i32, generated: i32) -> bool {
    if input == generated {
        println!("WIN you got the correct number {}", generated);
        return true;
    } else if input < generated {
        println!("Your input is too small.");
    } else {
        println!("your input is too high.");
    }
    false
}
