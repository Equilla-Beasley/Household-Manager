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
    id: u32,
    title: String,
    description: String,
    completed: bool,
}

impl ToDo{
    /*
    Name: toggle_complete
    Purpose: toggles completed field between "true" and "false"
    */
    fn toggle_complete(&mut self){
        if self.completed{
            self.completed = false;
        }
        else {
            self.completed = true;
        }
    }
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
    /*
    Name: Add
    Parameters: td: To-do
    Purpose: Add a To-Do to the list
    */
    fn add(&mut self, td: ToDo){
        self.list.push(td);
    }

    /*
    Name: Remove
    Parameters: id: u32
    Purpose: Remove a To-Do from the list
     */
    fn remove(&mut self, id: u32){
        let index = self.list.iter().position(|x| x.id == id).unwrap(); //finds index of td
        self.list.remove(index);
    }

}

//Main Function
fn main() {
    //Currently create two test to-do items and them adding them to a ToDoList
    let to_do1 = ToDo{
        id: 001,
        title: String::from( "Test ToDo"),
        description: String::from("Made to test ToDo"),
        completed: false,
    };

    let to_do2 = ToDo{
        id: 002,
        title: String::from("Test ToDo: Part 2"),
        description: String::from("Another Test, duh"),
        completed: true,
    };

    let mut v = ToDoList{ list: Vec::new() };
    v.add(to_do1);
    v.add(to_do2);

    //TESTS
    println!("Test 1: {:#?}", &v);
    v.remove(002);
    println!("Test 2: {:#?}", &v);
}
