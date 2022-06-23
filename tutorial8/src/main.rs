pub mod id_manager;
pub mod mem;
pub mod smart_pointers;
pub mod unsafe_code;

fn main() {
    println!("CIS198 Lecture 8: Systems Programming");

    // Fork example
    unsafe_code::test_fork();
}
