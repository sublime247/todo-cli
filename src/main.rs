use std::{env::args, str};
use serde_json;
use serde::Serialize;
use serde::Deserialize;
// extern crate serde;
#[derive(Debug,  Serialize, Deserialize)]
struct Todo{
    todo:String,
    status:bool,

}

impl Todo {
    fn add_todo(&mut self, _todo:String)->Self{
     Todo { todo: _todo, status: false}
    }
    
}


fn main() {
  let _todo= args().nth(1).expect("Please enter your todo");

  let mut  todo_list = Vec::new();

  let file_contents = std::fs::read_to_string("todo.txt").unwrap_or("[]".to_string());
  if !file_contents.is_empty(){
    todo_list = serde_json::from_str(&file_contents).unwrap();
  }


  
  let  todo = Todo{
    todo: _todo,
    status: false,
  };
  todo_list.push(todo);
  let json = serde_json::to_string(&todo_list).unwrap();
  std::fs::write("todo.txt", json.as_bytes()).unwrap();
}