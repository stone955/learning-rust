fn main() {
    // 结构体定义
    {
        let user = User {
            active: true,
            username: String::from("someone"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        println!("username: {}, email: {}", user.username, user.email);
    }

    // 可变结构体。注意整个实例必须是可变的；Rust 并不允许只将某个字段标记为可变。
    {
        let mut user = User {
            active: true,
            username: String::from("someone"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        user.active = false;

        println!("username: {}, active: {}", user.username, user.active);
    }

    // 通过函数创建结构体实例
    {
        let email = String::from("someone@example.com");
        let username = String::from("someone");
        let user = build_user(email, username);
        println!("username: {}, email: {}", user.username, user.email);
    }

    // 通过已有的实例创建新的实例
    {
        let email = String::from("someone@example.com");
        let username = String::from("someone");
        let user = build_user(email, username);

        let another = User {
            email: String::from("another@example.com"),
            ..user
        };

        // println!("username: {}, email: {}", user.username, user.email); user moved another
        println!(
            "another username: {}, email: {}, sign_in_count: {}",
            another.username, another.email, another.sign_in_count
        );
    }

    // 元组结构体
    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("r: {}, g: {}, b: {}", black.0, black.1, black.2);
        println!("x: {}, y: {}, z: {}", origin.0, origin.1, origin.2);
    }

    // 编写一个计算长方形面积的程序
    {
        // 通过函数方式计算
        {
            let width = 30;
            let height = 50;

            println!(
                "The area of the rectangle is {} square pixels.",
                area(width, height)
            );
        }

        // 使用元组重构
        {
            let rect = (50, 50);

            println!(
                "The area of the rectangle is {} square pixels.",
                area_dimensions(rect)
            );
        }

        // 使用结构体重构
        {
            let rect = Rectangle {
                width: 60,
                height: 60,
            };

            println!(
                "The area of the rectangle is {} square pixels.",
                area_rectangle(&rect)
            );

            // 通过派生 trait 增加实用功能
            println!("The rect is {:#?}", rect);
        }

        // 使用结构体方法重构
        {
            let rect = Rectangle {
                width: 80,
                height: 80,
            };

            println!(
                "The area of the rectangle is {} square pixels.",
                rect.area()
            );

            let another = Rectangle {
                width: 100,
                height: 100,
            };

            println!("Can rect hold another? {}", rect.hold(&another));
        }

        // 关联函数
        {
            let rect = Rectangle::from(200, 200);

            println!("The rect is {:#?}", rect);
        }
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_dimensions(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_rectangle(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }
}

impl Rectangle {
    fn from(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
}
