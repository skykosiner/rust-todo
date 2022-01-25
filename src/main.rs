use std::io;

#[derive(Debug)]
struct TODO {
    title: String,
    id: u32,
    completed: bool,
}

fn input_todo() -> String {
    println!("Enter your TODO:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn again(todo: Vec<TODO>) -> String {
    println!("A to add another TODO\n T to list TODOS\n C to complete TODO\n Q to quit");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string().pop();

    println!("Your input is: {}", input);
    println!("{}", input);

    input = "C".to_string();

    match input.as_str() {
        "A" =>  input_todo(),
        "T" => list_todo(&todo),
        "C" => complete_todo(todo),
        "Q" => quit(),
        _ => "Must be A, T, C or Q".to_string()
    }
}

fn new_todo(title: String) -> TODO {
    let todo = TODO {
        title,
        id: 0,
        completed: false,
    };

    println!("Created TODO: {:?}", todo);

    return todo;
}

fn complete_todo(todos: Vec<TODO>) -> String {
    println!("These are your todos:");
    list_todo(&todos);

    println!("Enter the ID of the TODO you want to complete:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let id = input.trim().parse::<u32>().expect("Failed to parse ID");

    for mut todo in todos {
        let mut todo_to_return = TODO {
            title: "nice".to_string(),
            id: 69420,
            completed: false,
        };

        if todo.id == id {
            todo.completed = true;
            todo_to_return = todo;
        };

        println!("Updated TODO: {:?}", todo_to_return);
    };

    return "TODO: complete".to_string();
}

fn list_todo(todos: &Vec<TODO>) -> String {
    for todo in todos {
        println!("{:?}", todo);
    }
    return "T".to_string();
}


fn quit() -> String {
    println!("Quit");
    return "Q".to_string();
}

fn main() {
    let mut todos: Vec<TODO> = Vec::new();
    let input = input_todo();
    todos.push(new_todo(input.to_string()));

    let todo_push = TODO {
        title: "nice".to_string(),
        id: 69420,
        completed: false,
    };

    todos.push(todo_push);

    let test = again(todos);
    println!("{}", test);
}
