fn main() {
    // 所有权的规则
    // - Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    // - 值在任一时刻有且只有一个所有者。
    // - 当所有者（变量）离开作用域，这个值将被丢弃。

    // 作用域
    {
        // _s 在这里无效

        let _s = "hello"; // _s 从此处开始有效

        // 使用s
    } // 此作用域结束，s 不再有效

    // String类型
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    // 变量与数据交互的方式（一）：移动
    // 当我们将 s1 赋值给 s2，String 的数据被复制了，这意味着我们从栈上拷贝了它的指针、长度和容量。我们并没有复制指针指向的堆上数据。
    let s1 = String::from("hello");

    let s2 = s1;

    // println!("s1: {}, s2: {}", s1, s2); // 编译报错

    println!("s2: {}", s2);

    // 变量与数据交互的方式（二）：克隆
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1: {}, s2: {}", s1, s2);

    // 只在栈上的数据：拷贝
    let xx = 5;
    let yy = xx;

    println!("xx: {}, yy: {}", xx, yy);

    // 所有权与函数
    let ss = String::from("hello");

    takes_ownership(ss); // ss 的值移动到函数里
                         // 这里不再有效

    let zzz = 5; // zzz 进入作用域
    makes_copy(zzz); // zzz 应该移动到函数里，但是因为i32是Copy的，所以后面可以继续使用zzz

    // 返回值与作用域
    let s1 = gives_ownership(); // gives_ownership 将返回值的所有权转移给s1

    let s2 = String::from("hello"); // s2 进入作用域

    let s3 = takes_and_gives_ownership(s2); // s2 所有权移动到 takes_and_gives_ownership
                                            // 将返回值的所有权转移给s3
    println!("s1: {}, s3: {}", s1, s3);

    // 使用元组返回多个值
    let s4 = String::from("hello, world!");

    let (s5, length) = calculate_length(s4);

    println!("The length of '{}' is {}", s5, length);

    // 引用与借用
    {
        let s = String::from("hello");
        let length = calculate_length2(&s);

        println!("The length of '{}' is {}", s, length);
    }

    // 可变引用
    // 在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败
    {
        let mut s = String::from("hello");
        
        change(&mut s);

        println!("{}", s);
    }

    // Slice
    {
        let s = String::from("hello world");

        let word = first_word(&s);

        println!("first word: {}", word);
    }

    // 引用的规则:
    // 1.在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    // 2.引用必须总是有效的。
}

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // some_string 离开作用域，占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // some_integer 移出作用域

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
