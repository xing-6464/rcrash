fn main() {
    let n = 5;

    if n > 0 {
        println!("{} is positive", n);
    } else if n < 0 {
        println!("{} is nefative", n);
    } else {
        println!("{} is ziro" ,n)
    }

    let m = if n < 0 {
        2.0
    } else {
        3.0
    };

    println!("{} value is m", m);
}