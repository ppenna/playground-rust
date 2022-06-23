pub mod panic;
pub mod result;

use panic::RawPassword;

fn main() {
    println!("CIS198 Lecture 5: Error Handling in Rust");

    // panic::panics();
    // panic::not_implemented();
    // panic::not_implemented_2();
    // panic::not_reachable();

    println!("{}", panic::capitalize_firstchar("Hello"));
    println!("{}", panic::capitalize_firstchar("hello"));
    println!("{}", panic::capitalize_firstchar("❤️ ❤️"));
    // println!("{}", panic::capitalize_firstchar(""));

    // panics
    // let raw_pass = RawPassword::new_password("caleb", "123!", 20210225);
    // let raw_pass = RawPassword::new_password("caleb", "1234567", 20210225);

    let raw_pass = RawPassword::new_password("caleb", "1234567!", 20210225);
    println!("\"Hash\": {}", raw_pass.hash());
}
