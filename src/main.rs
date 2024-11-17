use std::{env::args, str};
use serde_json;
use serde::Serialize;
// extern crate serde;
#[derive(Debug,  Serialize)]
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

  let mut todo = Todo{
    todo: String::new(),
    status: false,
  };
  todo = todo.add_todo(_todo.to_string());

  println!("{:?}", todo);
  let json = serde_json::to_string(&todo).unwrap();
  std::fs::write("todo.txt", json.as_bytes()).unwrap();
}