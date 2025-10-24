fn main() {
    counter_with_loop(4);
    println!();

    counter_with_loop_labels();
    println!();

    counter_with_while(4);
    println!();

    counter_with_for(4);
}

fn counter_with_loop(n: u32) {
    println!("Counting with loop till - {n}");
    let mut i = 0;

    loop {
        println!("i = {i}");
        i += 1;

        if i == n {
            break;
        }
    }
}

fn counter_with_loop_labels() {
    println!("Counting with loop labels");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // Since no label is provided it will default to breaking out from the inner-most loop
                break;
            }
            if count == 2 {
                // Breaks out from the top level loop
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn counter_with_while(n: u32) {
    println!("Counting with while till - {n}");

    let mut i = 0;

    while i < n {
        println!("i = {i}");

        i += 1;
    }
}

fn counter_with_for(n: u32) {
    println!("Count-down with for loop from - {n}");

    for i in (1..n + 1).rev() {
        println!("i = {i}");
    }

    println!("BOOOOOM!!!!!")
}
