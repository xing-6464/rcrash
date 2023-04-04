// 元组结构
struct Pair(i32, f32);

// 标准的 C 结构
#[derive(Debug)] // 派生属性
struct Person {
    name: String,
    age: u8,
}

// 单元结构（无字段, 通常在泛型里面使用）


fn main() {
    let pair = Pair(10, 4.2);
    println!("{}", pair.0);

    let jack = Person {
        name: String::from("jack"),
        age: 6,
    };

    println!("name = {}, age = {}", jack.name, jack.age);
    println!("{:?}", jack);

}