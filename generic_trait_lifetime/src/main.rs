use std::fmt::Display;

use generic_trait_lifetime::{Message, NewsArticle, Summary, Tweet};

fn main() {
    // 未使用泛型
    {
        let numbers = vec![34, 50, 25, 100, 65];

        println!("The largest number is {}", largest_i32(&numbers));

        let chars = vec!['y', 'm', 'a', 'q'];

        println!("The largest number is {}", largest_char(&chars));
    }

    // 在函数定义中使用泛型
    {
        let numbers = vec![34, 50, 25, 100, 65];

        println!("The largest number is {}", largest(&numbers));

        let chars = vec!['y', 'm', 'a', 'q'];

        println!("The largest number is {}", largest(&chars));
    }

    // 结构体定义中的泛型
    {
        let origin = Point { x: 0, y: 0 };

        println!("The origin is {:#?}", origin);

        let endpoint = Point { x: 3.0, y: 4.0 };

        println!("The endpoint is {:#?}", endpoint);
    }

    // 方法定义中的泛型
    {
        let point = Point { x: 30, y: 40 };

        println!("The point x is {}, y is {}", point.x(), point.y());
    }

    // 为类型实现 trait
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
        };

        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
        };

        println!("New article available! {}", article.summarize());

        let message = Message {};

        println!("1 message available! {}", message.summarize());
    }

    // 函数中的泛型生命周期
    {
        let str1 = String::from("abcde");
        let str2 = "xyz";

        println!("The longest string is {}", longest(str1.as_str(), str2));
    }

    // 结合泛型类型参数、trait bounds 和生命周期
    {
        let str1 = String::from("abcde");
        let str2 = "xyz";
        let ann = String::from("Announcement");

        println!(
            "The longest string is {}",
            longest_with_an_announcement(str1.as_str(), str2, ann)
        );
    }
}

fn largest_i32(numbers: &[i32]) -> i32 {
    let mut largest = numbers[0];

    for &num in numbers {
        if largest < num {
            largest = num;
        }
    }

    largest
}

fn largest_char(numbers: &[char]) -> char {
    let mut largest = numbers[0];

    for &num in numbers {
        if largest < num {
            largest = num;
        }
    }

    largest
}

fn largest<T: Copy + PartialOrd>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if largest < item {
            largest = item;
        }
    }

    largest
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// 函数签名中的生命周期标注
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

// 结合泛型类型参数、trait bounds 和生命周期
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
