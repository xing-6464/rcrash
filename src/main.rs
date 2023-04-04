use std::thread;

fn main() {


    let hello_message = "Hello World";

    thread::spawn(move || {
        println!("{}", hello_message);
    }).join();

}