// 创建变量：let 关键字
// 变量默认不开变
// 可变变量：变量名称加关键字 mut
// 常量： const 关键字
// Shadowing: 隐藏


fn main() {
    let a: i32 = 10;
    let b: char = 'A';

    let mytuple = (a, b);

    let (c,d) = mytuple;

    println!("c = {}, d = {}", c, d);
    println!("mytuple.0 = {}", mytuple.0);
    println!("mytuple.1 = {}", mytuple.1);

    let (res, is_overflow) = a.overflowing_add(10);

    println!("{} {}", res, is_overflow);

}
