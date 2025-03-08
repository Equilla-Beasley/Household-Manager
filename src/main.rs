/*
Dependencies:
*egui- UI creator
*serde - Serializes and Deserializes data
 */
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

/*
Struct: To-Do List
Purpose: Holds a list of To-Do values
and operates on them
 */
#[derive(Debug)]
struct ToDoList{
    list: Vec<ToDo>,
}

impl ToDoList{
    //Name - Add
    //Purpose - Add a To-Do to the list
    fn add(&mut self, td: ToDo){
        self.list.push(td);
    }

}

//Main Function
fn main() {
    //Currently create two test to-do items and them adding them to a ToDoList to be printed
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

    let mut v = ToDoList{ list: Vec::new() };
    v.add(to_do1);
    v.add(to_do2);
    println!("{:#?}", &v);
}
