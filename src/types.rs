type NumberAlias = number;
type BoolAlias = bool;
type StringAlias = str;

#[serde(tag = "t", content = "c")]
enum Colour {
    Red(i32),
    Green(i32),
    Blue((i32, str)),
}

struct Person {
  name: String,
  age: u32,
  enjoys_coffee: (i32, str),
}

struct ComplexType {
    colour_map: HashMap<String, Colour>,
    list_of_names: Vec<String>,
    optional_person: Option<Person>,
    weight_tuple: (i32, i32, i32)
}