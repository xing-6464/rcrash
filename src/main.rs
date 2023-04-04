fn main() {
    let opcode: u8 = 42;

    match  opcode {
        42 => {
            println!("bingo!");
        }
        _ => {
            println!("{}", opcode);
        }
    }
}