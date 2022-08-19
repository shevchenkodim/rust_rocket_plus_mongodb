
#[post("/some_params?<param1>&<param2")]
fn some_params_view(param1: String, param2: i32) {
    println!("{}", param1);
    println!("{}", param2);
}

#[derive(Debug, PartialEq, FromFormField)]
enum Color {
    Red,
    Blue,
    Green
}

#[derive(Debug, PartialEq, FromForm)]
struct Pet<'r> {
    name: &'r str,
    age: usize,
}

#[derive(Debug, PartialEq, FromForm)]
struct Person<'r> {
    pet: Pet<'r>,
}

#[get("/some_params_with_struct?<name>&<color>&<person>&<other>")]
fn some_params_with_struct_view(name: &str, color: Vec<Color>, person: Person<'_>, other: Option<usize>) {
    assert_eq!(name, "George");
    assert_eq!(color, [Color::Red, Color::Green, Color::Green, Color::Blue]);
    assert_eq!(other, None);
    assert_eq!(person, Person {
        pet: Pet { name: "Fi Fo Alex", age: 1 }
    });
}
