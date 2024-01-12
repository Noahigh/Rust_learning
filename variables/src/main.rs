// 声明一个常量，常量的类型必须指定
const MAX_POINTS:u32 = 100_000;

fn main() {
    println!("Hello, world!");

    let x = 5;

    println!("The value of x is: {}", x);

    // x = 6;  // 这是错误的
    let x = 6;  // 这里用到了隐藏（Shadow）特性
    // 隐藏（Shadow）特性 本质上就是声明了一个新的变量，所以它的类型和值都是可以变的，甚至可以改变其可变性
    println!("The value of x is: {}", x);
    print_type_of(&x);

    let mut x = 32;
    println!("[改变变量可变性]The value of x is: {}", x);
    x = 33;
    println!("The value of x is: {}", x);

    // ==========================================
    let mut y = 2;  // 声明可变变量，自动推导类型为i32

    println!("The value of y is: {}", y);

    y = 3;

    println!("The value of y is: {}", y);

    println!("Ths value of MAX_POINTS is: {}", MAX_POINTS);

    // MAX_POINTS = 999_999;  // 这是错误的，因为常量总是不可变的。

    // ===========================================

    let z = 25.73;  // 自动推导类型为f64
    println!("The value of z is: {}", z);

    x = x + 1;
    println!("The value of x+1 is: {}", x);

    x = x - y;
    println!("The value of x-y is: {}", x);


}

// 输出变量类型
fn print_type_of<T>(_: &T) {
    println!("变量类型为：{}", std::any::type_name::<T>())
}
