[//]: # (不可变性)
[//]: # (函数式编程强调不可变性，即数据一旦创建就不会被改变。Rust中默认的变量是不可变的，只有用mut关键字才能声明可变变量)
```rust
let x = 5; // 不可变
let mut y = 5; // 可变
y += 1;
```

[//]: # (高阶函数)
[//]: # (高阶函数是指可以接受其他函数作为参数，或者返回一个函数的函数。Rust中可以轻松定义和使用高阶函数：)
```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let double = |x| x * 2;
println!("{}", apply(double, 5)); // 输出 10
fn apply<F>(f: F, x: i32) -> i32
  where
          F: Fn(i32) -> i32,
{
  f(x)
}

let double = |x| x * 2;  // 这里就是一个闭包方法
println!("{}", apply(double, 5)); // 输出 10

```


[//]: # (闭包)
[//]: # (闭包是可以捕获其环境的匿名函数。Rust的闭包非常强大，可以从其定义的作用域中捕获变量：)
```rust
let x = 2;
let add_x = |y| y + x;
println!("{}", add_x(3)); // 输出 5

```

[//]: # (迭代器)
[//]: # (Rust中的迭代器（Iterator）提供了一种函数式编程风格的数据处理方式。你可以链式调用多个迭代器方法来对集合进行变换和过滤：)
```rust
let numbers = vec![1, 2, 3, 4, 5];
let even_numbers: Vec<i32> = numbers
    .iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * 2)
    .collect();
println!("{:?}", even_numbers); // 输出 [4, 8]

```

模式匹配
Rust的模式匹配非常强大，可以用来解构和匹配数据。match表达式和if let结构体常用于这种情况：
```rust
enum Option<T> {
    Some(T),
    None,
}

let maybe_number = Some(5);

match maybe_number {
    Some(x) => println!("数字是: {}", x),
    None => println!("没有数字"),
}

if let Some(x) = maybe_number {
    println!("数字是: {}", x);
}

```

[//]: # (函数组合与柯里化)
[//]: # (虽然Rust没有内置的柯里化（Currying）支持，但你可以通过闭包来实现函数组合和部分应用：)
```rust
fn add(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

let add_five = add(5);
println!("{}", add_five(3)); // 输出 8

```

[//]: # (类型系统)
[//]: # (Rust的强类型系统和模式匹配使得编写安全可靠的函数式代码成为可能。类型推断和泛型使得代码更简洁：)
```rust
fn identity<T>(x: T) -> T {
    x
}

println!("{}", identity(5)); // 输出 5
println!("{}", identity("hello")); // 输出 hello

```

[//]: # (总的来说，Rust结合了函数式编程的优点与系统编程的性能和安全性，使得开发者能够编写简洁、高效且安全的代码。)


# 函数式编程
> 罗马不是一天建成的，编程语言亦是如此，每一门编程语言在借鉴前辈的同时，也会提出自己独有的特性，Rust 即是如此。
> 当站在巨人肩膀上时，一个人所能看到的就更高更远，恰好，我们看到了函数式语言的优秀特性，例如：

- 使用函数作为参数进行传递
- 使用函数作为函数返回值
- 将函数赋值给变量
- 
见猎心喜，我们忍不住就借鉴了过来，于是你能看到本章的内容，天下语言一大。。。跑题了。

关于函数式编程到底是什么的争论由来已久，本章节并不会踏足这个泥潭，因此我们在这里主要关注的是函数式特性：

- 闭包 Closure
- 迭代器 Iterator
- 模式匹配
- 枚举

> 其中后两个在前面章节我们已经深入学习过，因此本章的重点就是闭包和迭代器，这些函数式特性可以让代码的可读性和易写性大幅提升。对于 Rust 语言来说，掌握这两者就相当于你同时拥有了倚天剑屠龙刀，威力无穷。