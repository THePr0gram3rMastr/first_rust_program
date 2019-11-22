//"lint" that prevents warnings about enum "varients" never being constructed 
#[allow(dead_code)]

//u32 is an unsigned 32bit integer
enum Color {
  Red,
  Green,
  Blue,
  RGB(u32, u32, u32)
}
//function adds two unsigned integers and returns them to the caller
fn add(x: u32, y: u32) -> u32 {
  
  return x+y;
}

fn main() {
  
  println!("Welcome to my program");

  //println is very similar to c
  println!("{}", add(4, 9));

  let color = Color::RGB(3, 4, 4);

  //"match" deconstructs the enum, works similarly to a c-like switch
  match color {
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    Color::Blue => println!("Blue"),
    Color::RGB(r, g, b) => println!("Red {}, Green {}, Blue {}", r, g, b),
  }
}