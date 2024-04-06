// 声明一个常量，常量的类型必须指定
const MAX_POINTS:u32 = 100_000;

fn main() {
    println!("Hello, world!");

    let x = 5;

    println!("The value of x is: {}", x);

    // x = 6;  // 这是错误的
    let x = 6;  // 这里用到了隐藏（Shadow）特性
    // 隐藏（Shadow）特性 本质上就是let声明了一个新的变量，所以它的类型和值都是可以变的，甚至可以改变其可变性
    println!("The value of x is: {}", x);
    print_type_of(&x);

    let mut x = 32;
    println!("[改变变量可变性]The value of x is: {}", x);
    x = 33;
    println!("The value of x is: {}", x);
    // 但是如果声明的时候加上了mut，使变量可变，那么只能改变它的值，不能改变值的类型
    // x = " ";  // 这是错误的，
    println!("!!!The value of x is: {}", x);

    // ==========================================
    let mut y = 2;  // 声明可变变量，自动推导类型为i32

    println!("The value of y is: {}", y);

    y = 3;  // 更改变量的值（2->3）

    println!("The value of y is: {}", y);

    println!("Ths value of MAX_POINTS is: {}", MAX_POINTS);

    // MAX_POINTS = 999_999;  // 这是错误的，因为常量总是不可变的。

    // ===========================================

    let z = 25.73;  // 自动推导类型为f64
    println!("The value of z is: {}", z);
    println!("The type of z is:");
    print_type_of(&z);

    x = x + 1;
    println!("The value of x+1 is: {}", x);

    x = x - y;
    println!("The value of x-y is: {}", x);

    let guess:u32 = "42".parse().expect("Not a number");  // 将字符串形式的数字转换为数字形式，此处必须标记变量类型，否则不知道转换成什么类型
    println!("The value of guess: {}", guess);

    // 新写法
    let n = 99i32;
    println!("数字+类型写法：{}", n);

    // bool
    let t = true;
    let f: bool = false;  // 附带了显式类型标注的语句


    // ===========================================

    let c = 'z';  // 一个char类型，占4个字节，是一个Unicode变量值
    // let z:char = '';
    // let heart_eyed_cat = '';

    // ===========================================

    // 复合类型（Compound Type）- 元组（Typle） 固定长度，无法更改
    let tup = (500, 6.4, 1);

    let t = (1, 25.0, 'f');  // 元组可以是不同类型的数据
    println!("The value of t: {:?}", t);

    // 复合类型（Compound Type）- 数组（Array）

    let t = (1, 25.0, 'f');  // 元组可以是不同类型的数据
    println!("The value of t: {:?}", t);
    let (x,y,z) = t;  // 元组解构（Destructuring）
    println!("The value of z is: {}", z);
    println!("The value of y is: {}", t.1);  // 索引访问形式
    println!("The value of x is: {}", t.0);  // 索引访问形式
    print_type_of(&t.1);


    // let l = [1, 1.1, 'l'];  // 数组必须是同一类型的数据
    // let l = [1, 2, 'l'];  // 数组必须是同一类型的数据
    let l = [1, 2, 3];  // 数组必须是同一类型的数据
    println!("The value of l: {:?}", l);
    let [a, b, c] = l;  // 数组解构（Destructuring）
    println!("The value of l[0]: {}", l[0]);  // 索引访问形式
    println!("The value of l[1]: {}", l[1]);  // 索引访问形式
    println!("The value of l[2]: {}", c);

}

// 输出变量类型
fn print_type_of<T>(_: &T) {
    println!("变量类型为：{}", std::any::type_name::<T>())
}

