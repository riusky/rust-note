/// 三种 Fn 特征
// 闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 Fn 特征也有三种：

// FnOnce，该类型的闭包会拿走被捕获变量的所有权。Once 顾名思义，说明该闭包只能运行一次：

fn fn_once<F>(func: F)
    where
        F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));// `func` moved due to this call // 转移在这
    println!("{}", func(4));// value used here after move // 转移后再次用
}


/*
将闭包也看作rust中的变量 有转移所有权等特性
闭包可以捕获当前作用域中的带值变量
所以一般作为参数传入方法中 处理方法内部的参数
*/

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}


fn fn_once2<F>(func: F)
    where
        F: FnOnce(usize) -> bool + Copy,// 改动在这里 添加了Copy的特性
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main2() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}