fn main() {
    let x = 4;
    /*
    在之前代码中，我们一直在用闭包的匿名函数特性（赋值给变量），然而闭包还拥有一项函数所不具备的特性：捕获作用域中的值。
    */

    let equal_to_x = |z| z == x; // 这里的x还可以使用

    let y = 4;

    assert!(equal_to_x(y));
}


fn main2() {
    let x = 4;

    /*
    对于函数来说，就算你把函数定义在 main 函数体中，它也不能访问 x：
    */
    fn equal_to_x(z: i32) -> bool {
        z == x // 注意这里的 x 报错了，函数是单独的空间域
    }

    let y = 4;

    assert!(equal_to_x(y));
}