//todoのテストを書く

use crate::todo;
use crate::todo::Todo;
use crate::todo::TodoList;
use crate::todo::TodoState;

#[test]
fn test_todo() {
    let todo = Todo::new("test todo".to_string());
    assert_eq!(todo.title, "test todo");
    assert_eq!(todo.state, TodoState::Todo);
    assert_eq!(todo.id, 0);
}

#[test]
fn test_todolist() {
    let mut todolist = TodoList::new();
    let todo = Todo::new("test todo".to_string());
    todolist.add(todo);
    assert_eq!(todolist.list.len(), 1);
    assert_eq!(todolist.list[0].title, "test todo");
    assert_eq!(todolist.list[0].state, TodoState::Todo);
    assert_eq!(todolist.list[0].id, 0);
}

// Path: bktodo/src/main.rs
fn main() {
    let mut todolist = todo::TodoList::new();
    let todo = todo::Todo::new("test todo".to_string());
    todolist.add(todo);
    println!("{}", todolist.list[0].title);
    println!("{}", todolist.list[0].state);
    println!("{}", todolist.list[0].id);
}

// Path: bktodo/src/todo.rs
//todoの構造体を定義
#[derive(Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub state: TodoState,
}

//todoの状態を定義
#[derive(Debug)]
pub enum TodoState {
    Todo,
    Doing,
    Done,
}

//todoのリストを定義
#[derive(Debug)]
pub struct TodoList {
    pub list: Vec<Todo>,
}

//todoの構造体に関数を定義
impl Todo {
    pub fn new(title: String) -> Self {
        Todo {
            id: 0,
            title: title,
            state: TodoState::Todo,
        }
    }
}

//todoのリストに関数を定義
impl TodoList {
    pub fn new() -> Self {
        TodoList { list: Vec::new() }
      }

      pub fn add(&mut self, todo: Todo) {
         self.list.push(todo);
      }
}
