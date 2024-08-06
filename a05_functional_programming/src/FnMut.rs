// FnMut，它以可变借用的方式捕获了环境中的值，因此可以修改该值：



fn main() {
    let mut s = String::new();

    let update_string =  |str| s.push_str(str);
    // calling `update_string` requires mutable binding due to mutable borrow of `s`
    update_string("hello");

    println!("{:?}",s);
}

// 修改为

fn main2() {
    let mut s = String::new();

    let mut update_string =  |str| s.push_str(str); // let mut update_string
    update_string("hello");

    println!("{:?}",s);
}

// Fn 特征，它以不可变借用的方式捕获环境中的值 让我们把上面的代码中 exec 的 F 泛型参数类型修改为 Fn(&'a str)：

fn main3() {
    let mut s = String::new();

    let update_string =  |str| s.push_str(str);

    exec(update_string);

    println!("{:?}",s);
}

fn exec<'a, F: Fn(&'a str)>(mut f: F)  {
    f("hello")
}