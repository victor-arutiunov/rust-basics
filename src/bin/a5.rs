// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut times = 0;
    let mut i = 1;
    loop {
        println!("{}", i);
        times += 1;
        if i == 4 {
            break;
        }
        i += 1;
    }
    println!("times");
    println!("{}", times);
}
