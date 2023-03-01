type NumberAlias = number;
type BoolAlias = bool;
type StringAlias = str;

#[serde(tag = "t", content = "c")]
enum Colour {
    Red(i32),
    Green(i32),
    Blue(i32),
}

struct Person {
  name: String,
  age: u32,
  enjoys_coffee: bool,
}