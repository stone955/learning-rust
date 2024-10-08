use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize(&self) -> String {
        // 默认实现
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct Message {}

impl Summary for Message {}

// 通过 + 指定多个 trait bound
pub fn notify(item: impl Summary + Display) {
    println!("Breaking news! {}", item.summarize());
}

// 通过 where 简化 trait bound
pub fn notify2<T, U>(t: T, u: U) -> (T, U)
where
    T: Clone + Display,
    U: Clone + Debug,
{
    (t, u)
}

// 返回实现了 trait 的类型
// pub fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//         } // 编译失败
//     }
// }
