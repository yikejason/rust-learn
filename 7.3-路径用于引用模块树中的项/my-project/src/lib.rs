// super关键字演示
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // 相对路径 利用super关键字访问上级目录
        crate::serve_order(); // 绝对路径访问
    }
    fn cook_order() {}
}

// pub struct演示
mod front_of_house {
    pub struct Breakfast {
        pub toast: String,
        pub fruit: String,
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = front_of_house::Breakfast::summer("yes");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    meal.fruit = String::from("blueberries");
}

// pub enum 演示  (只要枚举 （Apple) 是公共的 它的变体也就是公共的)
mod cake_of_house {
    pub enum Apple {
        Soup,
        Salad,
    }
}
