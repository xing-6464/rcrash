enum Alphabet {
    A,
    B,
    C
}



fn main() {
    let letter = Alphabet::A;

    match letter {
        Alphabet::A => {
            println!("It a");
        }
        _ => {

        }
    }

    if let Alphabet::A = letter {
        println!("It a");
    }
}