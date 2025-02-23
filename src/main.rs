use egui::Ui;
use serde::{Serialize, Deserialize};

/*
Struct: To-Do
Purpose: Holds information for some task
that needs to be done by the client.
*/
#[derive(Debug)]
struct ToDo{
    title: String,
    description: String,
    completed: bool,
}

//Main Function
fn main() {
    //Currently create two test to-do items and them adding them to a vector to be printed
    let to_do1 = ToDo{
        title: String::from( "Test ToDo"),
        description: String::from("Made to test ToDo"),
        completed: false,
    };

    let to_do2 = ToDo{
        title: String::from("Test ToDo: Part 2"),
        description: String::from("Another Test, duh"),
        completed: true,
    };

    let mut v: Vec<ToDo> = Vec::new();
    v.push(to_do1);
    v.push(to_do2);
    println!("{:#?}", &v);
}
