fn main() {
    println!("Hello, world!");

    another_function(5, 7);

    let mut x = 2;
    x = another_function_1(x, 7);

    println!("The value of x is: {}.", x);
}


fn another_function(x: i32, y: i32) {
    println!("Another function.");  // 这是一个语句
    println!("The value of x is: {}.", x);  // 这是一个语句
    println!("The value of y is: {}.", y);  // 这是一个语句
}


fn another_function_1(mut x: i32, y: i32) -> i32 {
    x + y  // 这是一个表达式
}


fn multiplication_two_number(a: i32, b: i32) -> i32 {
    // return a * b  // 加 return 关键字 的写法
    a * b  // 省略 return 关键字的写法
}
