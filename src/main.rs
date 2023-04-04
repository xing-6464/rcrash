fn main() {
    for i in 0..5 {
        println!("{}", i)
    }

    for i in 0..=5 {
        println!("{}", i)
    }

    let myarray = ["a", "b", "c"];
    for i in myarray.iter() {
        println!("{:?}", i);
    }

    let mut myarr = [1, 2, 3];
    for i in myarr.iter_mut() {
        *i *= 2;
    }
    for i in myarr.iter() {
        println!("{}", i);
    }
}