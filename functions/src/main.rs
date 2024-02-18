fn main() {
    println!("Hello, world!");

    another_function(5, 7);
}


fn another_function(x: i32, y: i32) {
    println!("Another function.");  // 这是一个语句
    println!("The value of x is: {}.", x);  // 这是一个语句
    println!("The value of y is: {}.", y);  // 这是一个语句
}
