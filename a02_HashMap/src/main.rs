use std::collections::HashMap;
fn main() {

    println!("Hello, world!");
    /* 创建 HashMap */
    // 创建一个HashMap，用于存储宝石种类和对应的数量
    let mut my_gems = HashMap::new();
    // 将宝石类型和对应的数量写入表中
    my_gems.insert("红宝石", 1);
    my_gems.insert("蓝宝石", 2);
    my_gems.insert("河边捡的误以为是宝石的破石头", 18);

    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let mut teams_map = HashMap::new();
    for team in &teams_list {
        teams_map.insert(&team.0, team.1);
    }
    println!("{:?}",teams_map);
    /* 再我运行10多次之后 我发现HashMap是没有顺序的 */

    // 上面的做法不够 rusty
    // Rust 为我们提供了一个非常精妙的解决办法：先将 Vec 转为迭代器，接着通过 collect 方法，将迭代器中的元素收集后，转成 HashMap
    let teams_list = vec![
        ("中国队".to_string(), 100),
        ("美国队".to_string(), 10),
        ("日本队".to_string(), 50),
    ];
    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    println!("{:?}",teams_map);

    /* 所有权转移 */
    //  若类型实现 Copy 特征，该类型会被复制进 HashMap，因此无所谓所有权
    // 若没实现 Copy 特征，所有权将被转移给 HashMap 中
    let name = String::from("Sunface");
    let age = 18;
    let mut handsome_boys = HashMap::new();
    // handsome_boys.insert(name, age); //handsome_boys.insert(name.clone(), age);
    handsome_boys.insert(name.clone(), age);
    println!("因为过于无耻，{}已经被从帅气男孩名单中除名", name);
    println!("还有，他的真实年龄远远不止{}岁", age);


    /* 查询 HashMap */
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);   // 注意这里返回的是一个Option<&i32> &i32 可以看出返回的是一个借用值
    // get 方法返回一个 Option<&i32> 类型：当查询不到时，会返回一个 None，查询到时返回 Some(&i32)
    // &i32 是对 HashMap 中值的借用，如果不使用借用，可能会发生所有权的转移
    let score: i32 = scores.get(&team_name).copied().unwrap_or(0);  //直接获取值

    //  Option::copied
    // copied方法用于将包含引用的Option转换为包含值的Option。
    // 它仅在Option包含一个可以复制的引用时有效。
    // 例如，如果你有一个Option<&T>，你可以使用copied将其转换为一个Option<T>，前提是T实现了Copy trait。
    let x: Option<&i32> = Some(&10);
    let copied_x: Option<i32> = x.copied();
    println!("{:?}", copied_x); // 输出: Some(10)
    // 在这个示例中，copied方法将Option<&i32>转换为了Option<i32>。

    // Option::unwrap_or
    // unwrap_or方法用于在Option是Some时返回其中的值，而在Option是None时返回提供的默认值。
    let x: Option<i32> = Some(10);
    let y: Option<i32> = None;
    let value1 = x.unwrap_or(5);
    let value2 = y.unwrap_or(5);
    println!("{}", value1); // 输出: 10
    println!("{}", value2); // 输出: 5


    /* 更新 HashMap 中的值 */
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    // 覆盖已有的值
    let old = scores.insert("Blue", 20);
    assert_eq!(old, Some(10));

    // 查询新插入的值
    let new = scores.get("Blue");
    assert_eq!(new, Some(&20));

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(5);
    assert_eq!(*v, 5); // 不存在，插入5

    // 查询Yellow对应的值，若不存在则插入新值
    let v = scores.entry("Yellow").or_insert(50);
    assert_eq!(*v, 5); // 已经存在，因此50没有插入

    // 在已有值的基础上更新 |- 全局变量  |- 临时缓存
    let text = "hello world wonderful world";
    // 在文本中统计词语出现的次数
    let mut map = HashMap::new();
    // 根据空格来切分字符串(英文单词都是通过空格切分)
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
