// a1

// fn first_name() {
//     let first_name = "Athithyan";
//     println!("{first_name}")
// }

// fn last_name() {
//     let last_name = "Rengaswamy";
//     println!("{last_name}");
// }

// fn main() {
//     first_name();
//     last_name();
// }

// fn sub(i1: i32, i2: i32) -> i32 {
//     i1-i2
// }
// fn main() {
//     let res = sub(3, 2);
//     print!("{res}")
// }

// a2

// fn add(a: i32, b:i32) -> i32 {
//     a+b
// }
// fn print_it(a: i32) {
//     println!("{a}")
// }
// fn main() {
//     print_it(add(1, 2))
// }

//a3.1
// fn conditional(cond: u64) {
//     if cond > 0 {
//         println!("hello")
//     } else {
//         println!("goodbye")
//     }
// }
// fn main() {
//     conditional(10000000000000000000)
// }

// fn conditional(number: u32) {
//     if number == 5 {
//         println!("=5")
//     } else if number > 5 {
//         println!(">5")
//     } else {
//         println!("<5")
//     }
// }

// fn main() {
//     conditional(6)
// }

// fn conditional(name: &str) {
//     match name {
//         "Athi" => println!("That's my name"),
//         "jeya" => println!("That's not my name"),
//         _ => println!("I dont know this case")
//     }
// }

// fn main() {
//     conditional("jeya")
// }

// fn main() {
//     let mut count = 1;
//     loop {
//         println!("count: {count}");
//         count += 1;
//         if count == 5 {
//             break;
//         }
//     }
// }

// fn main() {
//     let mut count = 5;

//     while count > 0 {
//         println!("count: {count}");
//         count -= 1;
//     }
// }

// enum Color {
//     Red,
//     Blue,
//     Green,
// }

// fn any(color: Color) {
//     match color {
//         Color::Red => println!("The color is red"),
//         Color::Blue => println!("The color is blue"),
//         Color::Green => println!("The color is green"),
//     }
// }

// fn main() {
//     any(Color::Green)
// }

// structs portion

// enum Flavors {
//     OrangeJuice,
//     AppleJuice
// }

// struct Drink {
//     flavor: Flavors,
//     ounce: u32
// }

// fn print_drink(drink: &Drink) {
//     match drink.flavor {
//         Flavors::AppleJuice => println!("its apple juice"),
//         Flavors::OrangeJuice => println!("its orange juice")
//     }

//     println!("ounce: {:?}", drink.ounce);
// }

// fn main() {
//     let drink = Drink {
//         flavor: Flavors::OrangeJuice,
//         ounce: 2
//     };
//     print_drink(&drink);
//     print_drink(&drink)

// }

// fn get_co_ordinates() -> (u32, u32) {
//     (3, 4)
// }

// fn main() {
//     let (_x, y) = get_co_ordinates();
//     if y == 5 {
//         println!("=5")
//     } else if y > 5 {
//         println!(">5")
//     } else {
//         println!("<5")
//     }
// }

// enum BoxColor {
//     Red, Green, Blue
// }

// impl BoxColor {
//     fn print(&self) {
//         match self {
//             BoxColor::Red => println!("color: Red"),
//             BoxColor::Green => println!("color: Green"),
//             BoxColor::Blue => println!("color: Blue"),
//         }

//     }
// }

// struct Dimensions {
//     height: u32,
//     width: u32,
//     depth: u32
// }

// impl Dimensions {
//     fn print(&self) {
//         println!("height: {:?}, width: {:?}, depth: {:?}", self.height, self.width, self.height);
//     }
// }

// struct CustomBox {
//     dimensions: Dimensions,
//     weight: u32,
//     color: BoxColor
// }

// impl CustomBox {
//     fn new(dimensions: Dimensions, weight: u32, color: BoxColor) -> Self {
//         Self { dimensions, weight, color }
//     }
//     fn print(&self) {
//         self.color.print();
//         self.dimensions.print();
//         println!("weight: {:?}", self.weight)
//     }
// }

// fn main() {
//     let my_box_dimensions = Dimensions { height: 10, width: 11, depth: 12 };
//     let my_box = CustomBox::new(my_box_dimensions, 100, BoxColor::Blue);
//     // loop {
//         my_box.print()
//     // }
// }

//a14 strings

// struct Person {
//     name: String,
//     age: u32,
//     fav_color: String
// }

//     fn print_person(name: &str, fav_color: &str) {
//         println!("name: {:?}, fav color: {:?}", name, fav_color);
//     }

// fn main() {
//     let persons = vec![
//         Person{ name: "Athi".to_owned(), age: 24, fav_color: "blue".to_owned() },
//         Person{ name: "Yazhi".to_owned(), age: 5, fav_color: "red".to_owned() },
//         Person{ name: "Jeya".to_owned(), age: 6, fav_color: "green".to_owned() },
//     ];

//     for person in persons {
//         if person.age < 10 {
//             print_person(&person.name, &person.fav_color)
//         }
//     }
// }

// Option<u32> some(5), None

// a18 - Result

// struct Customer {
//     age: u8,
//     name: String
// }

// fn can_user_purchase(customer: &Customer) -> Result<(), String> {
//     if customer.age < 18 {
//         return Err("User is underage, and cannot buy this product".to_owned())
//     }
//     Ok(())
// }

// fn main() {
//     let customer = Customer { age: 17, name: "Athi".to_owned() };
//     let can_purchase = can_user_purchase(&customer);
//     match can_purchase {
//         Ok(_) => println!("User {:?} can purchase this product", customer.name),
//         Err(err) => println!("{err}")
//     }
// }

// Result with question mark operator - a18b

// #[derive(Debug)]
// enum EmployeeType {
//     LineSupervisor,
//     KitchenStaff,
//     AssemblyTechnicians,
//     MaintanenceCrew,
//     MarketingDepartmentEmployee,
//     Managers,
// }

// struct Employee {
//     employee_type: EmployeeType,
//     is_employed: bool,
// }

// fn can_enter_the_building(employee: &Employee) -> Result<(), String> {
//     if !employee.is_employed {
//         return Err("Not employed".to_owned())
//     }
//     match employee.employee_type {
//         EmployeeType::MaintanenceCrew => Ok(()),
//         EmployeeType::MarketingDepartmentEmployee => Ok(()),
//         EmployeeType::Managers => Ok(()),
//         _ => Err("Employee cannot enter the building".to_owned())
//     }
// }

// fn print_access(employee: &Employee) -> Result<(), String> {
//     can_enter_the_building(employee)?;
//     println!("Employee can enter the building!");
//     Ok(())
// }

// fn main() {
//     let employee = Employee {
//         employee_type: EmployeeType::KitchenStaff,
//         is_employed: true,
//     };
//     match print_access(&employee) {
//         Err(err) => println!("{err}"),
//         Ok(_) => ()
//     }
// }

// a19 - Hashmap

// use std::collections::HashMap;

// fn main() {
//     let mut map = HashMap::new();
//     map.insert("chairs", 5);
//     map.insert("beds", 3);
//     map.insert("tables", 2);
//     map.insert("couches", 0);
//     let mut total_stock = 0;

//     for (item, qty) in map.iter() {
//         total_stock +=  qty;
//         let stock_count = if qty == &0 {
//             "out of stock".to_owned()
//         } else {
//             format!("{:?}", qty)
//         };
//         println!("item={item}, stock={stock_count}")
//     }

//     println!("Total stock = {total_stock}")

// }

// #[derive(Debug)]
// struct User {
//     user_id: u8,
//     name: String
// }

// fn find_user(name: &str) -> Option<u8> {
//     match name {
//         "athi" => Some(1),
//         "jeya" => Some(2),
//         _ => None
//     }
// }

// fn main() {
//     let user_name = "jeyaa";
//     let user = find_user(user_name).map(|user_id| User { user_id, name: user_name.to_owned() });
//     match user {
//         Some(user) => println!("{:?}", user),
//         _ => println!("User not found")
//     }

// }

fn main() {}

// template
// fn main() {}
