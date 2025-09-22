use ttk::prelude::*;

fn main() {
    print::info(1, "Welcome to the TTK demo!");
    print::ln();

    let username = query::line(2, "Enter your username:").unwrap();
    if username.contains(char::is_whitespace) || username.is_empty() {
        exit::err(3, "Invalid username");
    }

    print::ln();
    let id = query::item::<i32>(2, "Enter your ID (integer):").expect("Invalid ID");
    if id < 0 {
        print::warn(3, "Negative IDs are dangerous!");
    }

    print::ln();
    let countries = ["USA", "CAN", "GBR", "DEU", "JPN", "RUS"];
    let country = query::one(2, "Choose your country:", &countries, None).unwrap();

    print::ln();
    let roles = ["Admin", "Writer", "Announcer", "Editor", "Reader"];
    let selected_roles = query::many(2, "Choose your roles:", &roles, &[4]).unwrap();

    print::ln();
    print::dbg!(
        2,
        "Username: {username}, ID: {id}, Country: {}, Roles: {}",
        countries[country],
        from_fn::display(|f| {
            let mut first = true;
            for role in selected_roles
                .iter()
                .zip(roles)
                .filter_map(|(selected, role)| selected.then_some(role))
            {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;

                write!(f, "{role}")?;
            }

            Ok(())
        }),
    );

    print::info(1, "Thanks for viewing the demo!");
    exit::ok();
}
