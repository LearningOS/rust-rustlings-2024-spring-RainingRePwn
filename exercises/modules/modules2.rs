// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod delicious_snacks {
    // 使用 fruits 模块中的 PEAR 常量
    //use self::fruits::PEAR;
    // 使用 veggies 模块中的 CUCUMBER 常量
    //use self::veggies::CUCUMBER;

    // fruits 模块，包含水果常量定义
    pub mod fruits {
        // 定义 PEAR 常量为梨
        pub const PEAR: &'static str = "Pear";
        // 定义 APPLE 常量为苹果
        pub const APPLE: &'static str = "Apple";
    }

    // veggies 模块，包含蔬菜常量定义
    pub mod veggies {
        // 定义 CUCUMBER 常量为黄瓜
        pub const CUCUMBER: &'static str = "Cucumber";
        // 定义 CARROT 常量为胡萝卜
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    // 将 fruits 和 veggies 模块导入到 main 函数中
    use delicious_snacks::{fruits, veggies};

    // 打印出 favorite snacks，包括水果和蔬菜
    println!(
        "favorite snacks: {} and {}",
        // 使用 fruits 模块中的 PEAR 常量
        fruits::PEAR,
        // 使用 veggies 模块中的 CUCUMBER 常量
        veggies::CUCUMBER
    );
}

