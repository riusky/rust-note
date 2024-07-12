/// Vec的两种创建方式

fn main() {
    /* 创建动态数组 */
    // Vec::new
    // 使用 Vec::new 创建动态数组是最 rusty 的方式，它调用了 Vec 中的 new 关联函数：
    let v: Vec<i32> = Vec::new();
    // 这里，v 被显式地声明了类型 Vec<i32>，
    // 这是因为 Rust 编译器无法从 Vec::new() 中得到任何关于类型的暗示信息，
    // 因此也无法推导出 v 的具体类型，但是当你向里面增加一个元素后，一切又不同了：

    let mut v = Vec::new();
    v.push(1);
    // 此时，v 就无需手动声明类型，因为编译器通过 v.push(1)，推测出 v 中的元素类型是 i32，因此推导出 v 的类型是 Vec<i32>。

    // vec![]
    // 还可以使用宏 vec! 来创建数组，与 Vec::new 有所不同，前者能在创建同时给予初始化值：

    let v2 = vec![1, 2, 3];
    // 同样，此处的 v2 也无需标注类型，编译器只需检查它内部的元素即可自动推导出 v 的类型是 Vec<i32> （Rust 中，整数默认类型是 i32，在数值类型中有详细介绍）。


    /* 更新 Vector */
    // 向数组尾部添加元素，可以使用 push 方法：
    let mut v = Vec::new();
    v.push(1);
    // 与其它类型一样，必须将 v 声明为 mut 后，才能进行修改。


    /* Vector 与其元素共存亡 */
    // 跟结构体一样，Vector 类型在超出作用域范围后，会被自动删除：
    {
        let v = vec![1, 2, 3];

        // ...
    } // <- v超出作用域并在此处被删除
    // 当 Vector 被删除后，它内部存储的所有内容也会随之被删除。
    // 目前来看，这种解决方案简单直白，但是当 Vector 中的元素被引用后，事情可能会没那么简单。


    /* 从 Vector 中读取元素 */
    // 读取指定位置的元素有两种方式可选：
    //  通过下标索引访问。
    // 使用 get 方法。
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("第三个元素是 {}", third);

    match v.get(2) {
        Some(third) => println!("第三个元素是 {third}"),
        None => println!("去你的第三个元素，根本没有！"),
    }
    /* 它返回了 Option<&T> 语言安全的体现: 注意是使用get返回的Option */
    /* 下标索引与 .get 的区别 */  // 数组越界的问题


    /* 同时借用多个数组元素 */
    let mut v = vec![1, 2, 3, 4, 5];
    // 这里不可变借用
    let first = &v[0];
    println!("The first element is: {first}");
    // 可变借用
    v.push(6); // pub fn push(&mut self, value: T)
    // println!("The first element is: {first}");
    // 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，/* 然后把旧数组拷贝过来 */
    // 这种情况下，之前的引用显然会指向一块无效的内存，这非常 rusty —— 对用户进行严格的教育。


    /* 迭代遍历 Vector 中的元素 */ // 每次下标访问都会触发数组边界检查 比用下标的方式去遍历数组更安全也更高效
    let v = vec![1, 2, 3];
    for i in &v {
        println!("{i}");
    }
    // 也可以在迭代过程中，修改 Vector 中的元素：
    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 10;
        println!("{i}")
    }

}
