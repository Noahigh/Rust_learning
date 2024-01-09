// 声明一个常量
const MAX_POINTS:u32 = 100_000;


fn main() {
    println!("Hello, world!");

    let x = 5;

    println!("The value of x is: {}", x);

    // x = 6;  // 这是错误的
    let x = 6;  // 这里用到了隐藏（Shadow）特性
    // 隐藏（Shadow）特性 本质上就是声明了一个新的变量，所以它的类型和值都是可以变的，甚至可以改变其可变性
    println!("The value of x is: {}", x);

    let mut x = 32;
    println!("[改变变量可变性]The value of x is: {}", x);
    x = 33;
    println!("The value of x is: {}", x);

    // ==========================
    let mut y = 2;  // 声明可变变量

    println!("The value of y is: {}", y);

    y = 3;

    println!("The value of y is: {}", y);

    println!("Ths value of MAX_POINTS is: {}", MAX_POINTS);

    // MAX_POINTS = 999_999;  // 这是错误的，因为常量总是不可变的。
}
