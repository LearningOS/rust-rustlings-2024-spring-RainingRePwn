use std::rc::Rc;

// 定义太阳结构体
#[derive(Debug)]
struct Sun {}

// 定义行星枚举
#[derive(Debug)]
enum Planet {
Mercury(Rc<Sun>),
Venus(Rc<Sun>),
Earth(Rc<Sun>),
Mars(Rc<Sun>),
Jupiter(Rc<Sun>),
Saturn(Rc<Sun>),
Uranus(Rc<Sun>),
Neptune(Rc<Sun>),
}

impl Planet {
// 行星的详情方法
fn details(&self) {
println!("Hi from {:?}!", self)
}
}

fn main() {
// 创建 Rc 智能指针来共享太阳的所有权
let sun = Rc::new(Sun {});
println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

// 创建水星并共享太阳的所有权
let mercury = Planet::Mercury(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
mercury.details();

// 创建金星并共享太阳的所有权
let venus = Planet::Venus(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
venus.details();

// 创建地球并共享太阳的所有权
let earth = Planet::Earth(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
earth.details();

// 创建火星并共享太阳的所有权
let mars = Planet::Mars(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
mars.details();

// 创建木星并共享太阳的所有权
let jupiter = Planet::Jupiter(Rc::clone(&sun));
println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
jupiter.details();

// 创建土星并共享太阳的所有权
let saturn = Planet::Saturn(Rc::clone(&sun)); // 使用 Rc::clone 共享所有权
println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
saturn.details();

// 创建天王星并共享太阳的所有权
let uranus = Planet::Uranus(Rc::clone(&sun)); // 使用 Rc::clone 共享所有权
println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
uranus.details();

// 创建海王星并共享太阳的所有权
let neptune = Planet::Neptune(Rc::clone(&sun)); // 使用 Rc::clone 共享所有权
println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
neptune.details();

assert_eq!(Rc::strong_count(&sun), 9);

// 释放所有行星的引用
drop(mercury);
drop(venus);
drop(earth);
drop(mars);
drop(jupiter);
drop(saturn);
drop(uranus);
drop(neptune);

println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

assert_eq!(Rc::strong_count(&sun), 1);
}