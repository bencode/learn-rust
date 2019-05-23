// 有点让我想到C++
use ferris_says::say;

use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    // 创建Buffer?, 那应该可以创建各种自定义前缀，正则表达式也可以使用类似语法创建。
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    // 默认数据不可变？ 用这个操作符来表示副作用操作？
    let mut writer = BufWriter::new(stdout.lock());

    // unwrap()这个名字比较有趣，含义还不清楚。
    say(out, width, &mut writer).unwrap();
}
