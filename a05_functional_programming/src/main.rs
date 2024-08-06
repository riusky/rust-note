mod main2;
mod Fnmain;
mod r#move;
mod r#FnMut

use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    /* 闭包 Closure */
    /// 闭包是一种 *匿名函数，它可以赋值给变量也可以作为参数传递给其它函数，不同于函数的是，它允许捕获调用者作用域中的值* ，例如：
    /// fn main() {
    //    let x = 1;
    //    let sum = |y| x + y;
    //
    //     assert_eq!(3, sum(2)); // 捕获调用者作用域中的值 x
    // }


    /* 闭包的定义 */
    /// 上面的代码展示了非常简单的闭包 sum，它拥有一个入参 y，同时捕获了作用域中的 x 的值，
    /// 因此调用 sum(2) 意味着将 2（参数 y）跟 1（x）进行相加，最终返回它们的和：3。
    /// 可以看到 sum 非常符合闭包的定义：可以赋值给变量，允许捕获调用者作用域中的值。
    ///
    /// Rust 闭包在形式上借鉴了 Smalltalk 和 Ruby 语言，
    /// 与函数最大的不同就是它的参数是通过 |parm1| 的形式进行声明，
    /// 如果是多个参数就 |param1, param2,...|， 下面给出闭包的形式定义：
    /// |param1, param2,...| {
    //     语句1;
    //     语句2;
    //     返回表达式
    // }
// 动作次数
    let intensity = 10;
    // 随机值用来决定某个选择
    let random_number = 7;

    // 开始健身
    workout(intensity, random_number);


    /* 闭包的类型推导 */
    /// 闭包并不会作为 API 对外提供，因此它可以享受编译器的类型推导能力，无需标注参数和返回值的类型。

    // 为了增加代码可读性，有时候我们会显式地给类型进行标注，出于同样的目的，也可以给闭包标注类型：
    let sum = |x: i32, y: i32| -> i32 {
        x + y
    };

    // 不标注 再使用的时候会被确定
    let sum  = |x, y| x + y;
    let v = sum(1, 2);
    /// 虽然类型推导很好用，但是它不是泛型，当编译器推导出一种类型后，它就会一直使用该类型：
    /// let example_closure = |x| x;
    //
    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // 这里会出错 闭包已经被确定类型了

    // 下面展示了同一个功能的函数和闭包实现形式：
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;
    // 虽然类型推导很好用，但是它不是泛型，当编译器推导出一种类型后，它就会一直使用该类型：
    // 第一次被调用之后就确定了类型 之后都会使用这个类型


    /// 假设我们要实现一个简易缓存，功能是获取一个值，然后将其缓存起来，那么可以这样设计：
    //
    // 一个闭包用于获取值
    // 一个变量，用于存储该值
    /// *下面的结构设计很cool*
    struct Cacher<T>
        where
            T: Fn(u32) -> u32, // Fn(u32) -> u32 是一个特征，用来表示 T 是一个闭包类型 一个没有函数申明的却带有函数功能的结构
    // 特征 Fn(u32) -> u32 从表面来看，就对闭包形式进行了显而易见的限制：该闭包拥有一个u32类型的参数，同时返回一个u32类型的值。
    {
        query: T,
        value: Option<u32>,
    }


    impl<T> Cacher<T>
        where
            T: Fn(u32) -> u32,
    {
        fn new(query: T) -> Cacher<T> {
            Cacher {
                query,
                value: None,
            }
        }

        // 先查询缓存值 `self.value`，若不存在，则调用 `query` 加载
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.query)(arg);//调用传入的闭包方法
                    self.value = Some(v);
                    v
                }
            }
        }
    }


}


fn workout(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    /* 这里需要特别的理解 */
    // 闭包中最后一行表达式返回的值，就是闭包执行后的返回值，因此 action() 调用返回了 intensity 的值 10
    // let action = ||... *只是把闭包赋值给变量 action，并不是把闭包执行后的结果赋值给 action*，
    // 因此这里 action 就相当于 **闭包函数** ，可以跟函数一样进行调用：action()

    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            action()
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action()
        );
    }
}