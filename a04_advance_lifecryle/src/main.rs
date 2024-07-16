/* 生命周期也是结构(类型)的一部分 */
fn main() {
    println!("Hello, world!");

    /* 无界生命周期 */
// 不安全代码(unsafe)经常会凭空产生引用或生命周期，这些生命周期被称为是 无界(unbound) 的。
// 无界生命周期往往是在解引用一个裸指针(裸指针 raw pointer)时产生的，
// 换句话说，它是凭空产生的，因为输入参数根本就没有这个生命周期：

    fn f<'a, T>(x: *const T) -> &'a T {
        unsafe {
            &*x
        }
    }
// 'a: 'b
// 假设有两个引用 &'a i32 和 &'b i32，它们的生命周期分别是 'a 和 'b，若 'a >= 'b，
// 则可以定义 'a:'b，表示 'a 至少要活得跟 'b 一样久。
    struct DoubleRef<'a,'b:'a, T> {
        r: &'a T,
        s: &'b T
    }


    /* 类型也可以用生命周期来限制 */
    // T: 'a
    // 表示类型 T 必须比 'a 活得要久：
    struct Ref<'a, T: 'a> {
        r: &'a T
    }
    /// 因为结构体字段 r 引用了 T，
    /// 因此 r 的生命周期 'a 必须要比 T 的生命周期更短(被引用者的生命周期必须要比引用长)。
    /// 也就是说 我被引用了 我的引用消失了 我才可以离场
    /* 这里应该可以自动推导出来 */
    /// 在 Rust 1.30 版本之前，
    /// 该写法是必须的，但是从 1.31 版本开始，编译器可以自动推导 T: 'a 类型的约束，因此我们只需这样写即可：
    /// struct Ref<'a, T> {
    //     r: &'a T
    // }





    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    use_list(&list);

    // 首先在直觉上，list.get_interface() 借用的可变引用，按理来说应该在这行代码结束后，就归还了，
    // 但是为什么还能持续到 use_list(&list) 后面呢？
/// 这是因为我们在 get_interface 方法中声明的 lifetime 有问题，
/// 该方法的参数的生命周期是 'a，而 List 的生命周期也是 'a，
/// 说明该方法至少活得跟 List 一样久，再回到 main 函数中，
/// list 可以活到 main 函数的结束，因此 list.get_interface() 借用的可变引用也会活到 main 函数的结束，
/// 在此期间，自然无法再进行借用了。

}
fn use_list(list: &List) {
    println!("{}", list.manager.text);
}
struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'a mut self) -> Interface<'b, 'a>
    where 'a:'b {
        Interface {
            manager: &mut self.manager
        }
    }
}