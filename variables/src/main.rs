fn main() {
    let x = 5;

    println!("The value of x is {}", x);

    // x = 6;
    /*
        ❯ cargo run
       Compiling variables v0.1.0 (/Users/stone/Documents/Code/stone/rust/learning-rust/variables)
    error[E0384]: cannot assign twice to immutable variable `x`
     --> src/main.rs:5:5
      |
    2 |     let x = 5;
      |         -
      |         |
      |         first assignment to `x`
      |         help: consider making this binding mutable: `mut x`
    ...
    5 |     x = 6;
      |     ^^^^^ cannot assign twice to immutable variable

    For more information about this error, try `rustc --explain E0384`.
    error: could not compile `variables` (bin "variables") due to 1 previous error

    */

    let mut y = 10;

    println!("The value of y is {}", y);

    y = 15;

    println!("The value of y is {}", y);

    // 常量
    // 常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变。
    // 常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须注明。
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );

    // 遮蔽
    let z = 5;

    let z = z + 1;

    {
        // 只在作用域范围内生效
        let z = z * 2;

        println!("The value of z in the inner scope is: {}", z);
    }

    println!("The value of z is: {}", z);

    // 改变数据类型
    let spaces = "    ";
    let spaces = spaces.len();

    let mut spaces = "    ";
    // spaces = spaces.len(); // 编译失败
  }