// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal

fn main() {
    fn get_first_name() -> &'static str {
        "Victor"
    }

    fn get_last_name() -> &'static str {
        "Arutiunov"
    }

    let first_name = get_first_name();
    let last_name = get_last_name();

    println!("{} {}", first_name, last_name);
}
