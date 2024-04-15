fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
    // for region in regions {  // 2021 Edition 及以后，可直接用这种写法，for隐式地将regions转换成迭代器
        println!("{}", &region);
    }
}


fn main() {
    greet_world();
}
