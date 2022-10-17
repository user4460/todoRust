//todoリストを作成する関数
fn todo_list_create() {
    let mut todo_list = TodoList::new();
    let mut todo_list_file = File::open("todo_list.json").unwrap();
    let mut todo_list_string = String::new();
    todo_list_file.read_to_string(&mut todo_list_string).unwrap();
    let todo_list_json: Value = serde_json::from_str(&todo_list_string).unwrap();
    let todo_list_json = todo_list_json.as_object().unwrap();
    let todo_list_json = todo_list_json.get("todo_list").unwrap();
    let todo_list_json = todo_list_json.as_array().unwrap();
    for todo_json in todo_list_json {
        let todo_json = todo_json.as_object().unwrap();
        let todo_json = todo_json.get("todo").unwrap();
        let todo_json = todo_json.as_object().unwrap();
        let todo_json = todo_json.get("todo").unwrap();
        let todo_json = todo_json.as_str().unwrap();
        let todo_json = todo_json.to_string();
        todo_list.todo_list.push(Todo { todo: todo_json });
    }
    println!("Enter the todo");
    let mut todo = String::new();
    io::stdin().read_line(&mut todo).unwrap();
    todo = todo.trim().to_string();
    todo_list.todo_list.push(Todo { todo });
    let todo_list_json = serde_json::to_string(&todo_list).unwrap();
    let mut todo_list_file = File::create("todo_list.json").unwrap();
    todo_list_file.write_all(todo_list_json.as_bytes()).unwrap();
}