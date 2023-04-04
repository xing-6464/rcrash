fn main() {
    // 1 + 2 + ... + 100
    let mut sum = 0;
    let mut n = 1;
    loop {
        sum += n;
        n += 1;
        if n > 100 {
            break;
        }
    }
    println!("{} value is sum", sum);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 100 {
            break counter * 2;
        }
    };

    println!("{} value is counter", result);
}