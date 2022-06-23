// Also knowned at tagged unions, algebraic data types, sum types.
enum Days {
    Mon, // Data constructors.
    Tue,
    Wed,
}

enum Empty {}

// Usage.
fn f() {
    use self::Days::*;

    let d = Days::Mon;
    let d2 = Wed;

    match d {
        Mon => println!("hello monday"),
        Tue => println!("hello tue"),
        _ => println!("hello wed"),
    }
}

// Deriving traits.
#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum ServerCommand {
    Get(i32),
    Delete(String, String),
    Drop,
}

fn g(s: ServerCommand) {
    match &s {
        // Pattern guard.
        ServerCommand::Get(x) if *x > 10 => {}

        ServerCommand::Get(x) => {}

        ServerCommand::Delete(person, _) => {
            let person2: &String = person;
        }
        ServerCommand::Drop => {}
    }

    println!("{:?}", s);
}
