
// 如果你想强制闭包取得捕获变量的所有权，
// 可以在参数列表前添加 move 关键字，
// 这种用法通常用于闭包的生命周期大于捕获变量的生命周期时，
// 例如将闭包返回或移入其他线程。
use std::thread;
fn move_t(){
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    /// move 强制闭包取得捕获变量的所有权
println!("Here's a vector: {:?}", v);
});
handle.join().unwrap();
}


