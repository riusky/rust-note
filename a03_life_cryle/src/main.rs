fn main() {
    println!("Hello, world!");

    ///
    /// Rust的生命周期（lifetime）是Rust的借用检查器用于确保引用在使用时有效的一个重要概念。
    /// 它帮助Rust在编译时确保内存安全，防止悬空引用和数据竞争。
    /// 理解生命周期对编写安全且高效的Rust代码非常关键。
    /// 以下是生命周期的详细介绍：
    ///
    /// *生命周期的基本概念*
    /// 生命周期是Rust编译器用来跟踪引用如何在程序中存活的一种方式。
    /// 每个引用都有一个生命周期，它表示引用在程序中保持有效的范围。
    /// Rust在编译时通过生命周期来确保所有引用在它们被使用时都是有效的。
    ///
    /// *生命周期标注*
    /// 生命周期标注用单引号（'）后跟一个名字表示，如'a。
    /// 生命周期标注通常出现在函数签名中，以表示输入和输出引用的关系。
    /// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    //  }
    ///
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    println!("The longest string is {}", string1);




    /// 下面的错误可以考虑其他方式解决  move等
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);


    // 函数的返回值如果是一个引用类型，那么它的生命周期只会来源于：
    // 函数参数的生命周期
    // 函数体中某个新建引用的生命周期 若是后者情况，就是典型的悬垂引用场景：
    /// fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }
    /// 那遇到这种情况该怎么办？最好的办法就是返回内部字符串的所有权，然后把字符串的所有权转移给调用者：
    fn longest_t<'a>(_x: &str, _y: &str) -> String {
        String::from("really long string")
    }

    let s = longest_t("not", "important");
}

///struct ImportantExcerpt {
//     part: &str,
// }
// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

/* 每一个引用参数都会获得独自的生命周期 */
/* 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期 */
/* 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期 */

/* 结构体中的生命周期实现方法注意 */
struct ImportantExcerpt<'a> {
    part: &'a str,
}
// impl 中必须使用结构体的完整名称，包括 <'a>，因为生命周期标注也是结构体类型的一部分！
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}



impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'b str
        where
            'a: 'b, // 'a: 'b，是生命周期约束语法，跟泛型约束非常相似，用于说明 'a 必须比 'b 活得久
    {
        println!("Attention please: {}", announcement);
        self.part
    }
}



/* 静态生命周期 */

// 在 Rust 中有一个非常特殊的生命周期，那就是 'static，拥有该生命周期的引用可以和整个程序活得一样久。
// 不用考虑生命周期失效 但是一般不使用
fn staticlife() {

let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
}
// 生命周期 'static 意味着能和程序活得一样久，例如字符串字面量和特征对象
// 实在遇到解决不了的生命周期标注问题，可以尝试 T: 'static，有时候它会给你奇迹


/* 一个复杂例子: 泛型、特征约束 */

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
