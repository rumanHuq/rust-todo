use std::io;

#[derive(Debug)]
struct Todo {
  id: i16,
  title: String,
  completed: bool,
  deleted: bool,
}

fn add_todo(todos: &mut Vec<Todo>, title: &str){
  let id = todos.len() as i16 + 1;
  let item = Todo {
    id: id,
    title: title.to_string(),
    completed: false,
    deleted: false
  };
  println!("Item pushed: {:?} \n", &item);
  todos.push(item);
  
}
fn remove_todo(todos: &mut Vec<Todo>, id: i16){
  if let Some(todo) = todos.iter_mut().find(|x| x.id == id) {
    todo.deleted = true
  }
}
fn mark_done(todos: &mut Vec<Todo>, id: i16){
  if let Some(todo) = todos.iter_mut().find(|x| x.id == id) {
    todo.completed = true
  }
}
fn print_todos(todos: &Vec<Todo>){
  println!("\n\nTodo List:\n-------------------");
  
  for todo in todos {
    if !todo.deleted {
      let done = if todo.completed {"✔"} else {" "};
      println!("[{}] {} {}", done, todo.id, todo.title);
    };
  }
}
fn invalid_command(command: &str) {  
    println!("Invalid command: {}", command);
}

fn main() {
  let mut todos: Vec<Todo> = Vec::new();
  print_todos(&todos);
  
  loop {
    let mut command = String::new();
    
    io::stdin()
      .read_line(&mut command)
      .expect("Something went wrong");
      
    let command_parts: Vec<&str> = command.split_whitespace().collect();
    
    match command_parts.len() {
      0 => invalid_command(&command),
      1 => match command_parts[0] {
        "list" => print_todos(&todos),
        _ => invalid_command(&command)  
      }
      _ => match command_parts[0] {
        "add" => add_todo(&mut todos, &command_parts[1..].join(" ")),
        "remove" => if let Ok(num) = command_parts[1].parse::<i16>() {
          remove_todo(&mut todos, num)
        },
        "done" => if let Ok(id) = command_parts[1].parse::<i16>() {
          mark_done(&mut todos, id)
        }
        _ => for cmd in command_parts {
          invalid_command(&cmd) 
        }
      }
    }
  }
}
