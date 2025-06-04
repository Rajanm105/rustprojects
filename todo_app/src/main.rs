use std::io;

struct TodoItem {
    id: u64,
    title: String,
    completed: bool,
}

fn complete_item(item: &mut TodoItem){
    item.completed = true;
}

fn display_items(items: &Vec<TodoItem>) {
    for item in items {
        println!("{} - {} ({})", item.id, item.name, item.completed);
    }
}

fn main(){
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        println!("What would you like to do?");
        println!("1. Add a to-do item");
        println!("2. Complete a to-do item");
        println!("3. Display to-do items");
        println!("4. Quit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        let choice = choice.trim().parse::<u32>().expect("Invalid input");

        match choice {
            1 => {
                println!("Enter the name of the to-do item:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = todo_list.len() as u32 + 1;

                let item = TodoItem {
                    id,
                    name,
                    completed: false,
                };

                todo_list.push(item);
            },
            2 => {
                println!("Enter the ID of the to-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid input");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
                complete_item(item);
            },
            3 => {
                display_items(&todo_list);
            },
            4 => {
                println!("Goodbye!");
                return;
            },
            _ => {
                println!("Invalid choice");
            },
        }
    }

    let item1 = TodoItem {
        id: 1,
        name: String::from("Buy milk"),
        completed: false,
    };

    let item2 = TodoItem {
        id: 2,
        name: String::from("walk the dog"),
        completed: false,
    };

    todo_list.push(item1);
    todo_list.push(item2);

    println!("{:?}", todo_list);
}
