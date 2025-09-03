use ttk::prelude::*;

fn main() {
    print::info(1, "Welcome to the TTK demo!");

    let username = query::line(2, "Enter your username:").unwrap();
    if username.contains(char::is_whitespace) {
        exit::err(3, "Username cannot contain whitespace");
    }

    print::ln();
    let id = query::item::<i32>(2, "Enter your ID (integer):").expect("Invalid ID");
    if id < 0 {
        print::warn(3, "Negative IDs are dangerous!");
    }

    print::ln();
    print::dbg!(2, "Username: {username}, ID: {id}");

    print::info(1, "Thanks for viewing the demo!");
    exit::ok();
}
