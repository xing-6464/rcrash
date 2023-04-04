// 创建变量：let 关键字
// 变量默认不开变
// 可变变量：变量名称加关键字 mut
// 常量： const 关键字
// Shadowing: 隐藏

fn avg(a: u32, b: u32) -> u32 {
    (a & b) + ((a ^ b) >> 1)
}


fn main() {
    let a: u32 = "4294967295".parse::<u32>().unwrap();
    let b: u32 = 2;

    let (mul, is_overflow) = a.overflowing_mul(b);
    println!("mul = {}, is_overflow = {}", mul, is_overflow);


    assert_eq!(avg(4294967295, 4294967295), 4294967295);
    assert_eq!(avg(0, 0), 0);
    assert_eq!(avg(10, 20), 15);
    assert_eq!(avg(4294967295, 1), 2147483648);
    println!("passed");
}
