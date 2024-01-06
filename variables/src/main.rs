// 声明一个常量
const MAX_POINTS:u32 = 100_000;


fn main() {
    println!("Hello, world!");

    let x = 5;

    println!("The value of x is: {}", x);

    // x = 6;  // 这是错误的
    let x = 6;  // 这里用到了隐藏（Shadow）特性
    println!("The value of x is: {}", x);

    // ==========================
    let mut y = 2;  // 声明可变变量

    println!("The value of y is: {}", y);

    y = 3;

    println!("The value of y is: {}", y);

    println!("Ths value of MAX_POINTS is: {}", MAX_POINTS);

    // MAX_POINTS = 999_999;  // 这是错误的，因为常量总是不可变的。
}
