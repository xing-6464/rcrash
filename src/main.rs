// 创建变量：let 关键字
// 变量默认不开变
// 可变变量：变量名称加关键字 mut
// 常量： const 关键字
// Shadowing: 隐藏

const A_CONST: i32 = 42;

fn get_number() -> i32 {
    42
}

fn main() {
    let x = 5;
    println!("The value of x is {}", x);

    let mut y = 6;
    println!("the value of y is {}", y);

    y = 10;
    println!("the value of y is {}", y);

    println!("the value of A_CONST is {}", A_CONST);

    let r = get_number();

    println!("the value of get_number() is {}", r);


    // 隐藏
    let x = x * 10;
    println!("the value of x is {}", x);

}
