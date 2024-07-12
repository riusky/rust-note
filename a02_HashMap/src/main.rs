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

}
