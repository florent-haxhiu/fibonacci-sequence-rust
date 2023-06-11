use std::io;

fn main() {
    let mut count = 0;
    let mut num_one = 0;
    let mut num_two = 1;

    let mut end = String::new().parse().expect("Not a number!");

    io::stdin()
        .read_line(&mut end)
        .expect("Failed to read line");
    
    let end: u32 = end.trim().parse().expect("Enter an amount of times to loop");

    loop {
        if count == end {
            break;
        }
        let num_three = num_one + num_two;
        num_one = num_two;
        num_two = num_three;
        println!("{num_one}");
        count = count + 1;
    }
}
