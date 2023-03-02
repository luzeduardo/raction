use std::time::{Duration, Instant};

fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn break_value_assign() {
    let n = loop {
        break 123;
    };
    println!("{}", n);
}
fn main() {
    let n = 123456;

    let description = if is_even(n) {
        "even"
    } else {
        "odd"
    };
    println!("{} is {}", n, description);

    let j = 78676;
    let description2 = match is_even(j) {
        true => "even",
        false => "odd",
    };
    println!("{} is {}", j, description2);

    break_value_assign();

    let mut count = 0;
    let time_limit = Duration::new(1, 0);
    let start = Instant::now();

    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    print!("{}", count);

    for (x,y) in (0..).zip(0..) {
        if x + y > 100 {
            break;
        }
    }

    // nested loop
    'outer: for x in 0.. {
        for y in 0.. {
            for z in 0.. {
                if x + y + z > 1000 {
                    break 'outer
                }
            }
        }
    }
}
