use todo::todo_list;
use std::io;

fn main() {
    let mut list = todo_list::TodoList::new();
    // NOTE: Item is temp and loses ownership when added to list. (Design choice for now easy fix if undesired)
    let item = todo_list::TodoItem::new("Clean house".to_string(), "Guests are coming, clean the house".to_string());

    // Important note, I have not read the chapter on lifetimes, so I have not used &str for my structs, leaving me with strings
    // and the ugly .to_string() attached to every string literal

    // Default way of adding items
    list.add_item("Get milk".to_string(), "Go to Walmart and get some milk for the recipe".to_string());
    list.add_item("Go running".to_string(), "Go running at the gym for 2 miles".to_string());
    list.add_item("Walk the dog".to_string(), "At 1 PM during lunch, go home to walk the dog".to_string());

    // Generated examples using AI
    list.add_item("Buy groceries".to_string(), "Get milk, eggs, and bread from the store.".to_string());
    list.add_item("Workout".to_string(), "Go for a 30-minute run and do strength training.".to_string());
    list.add_item("Read a book".to_string(), "Read at least 20 pages of 'The Pragmatic Programmer'.".to_string());
    list.add_item("Clean the kitchen".to_string(), "Wash dishes, wipe counters, and take out the trash.".to_string());
    list.add_item("Finish project".to_string(), "Work on the Rust project and test new features.".to_string());
    list.add_item("Call a friend".to_string(), "Catch up with Alex and discuss weekend plans.".to_string());
    list.add_item("Plan weekly meals".to_string(), "Decide on meals for the week and make a shopping list.".to_string());
    list.add_item("Review code".to_string(), "Look over PRs and refactor where necessary.".to_string());
    list.add_item("Organize desk".to_string(), "Sort papers, clean workspace, and declutter.".to_string());
    list.add_item("Learn a new Rust feature".to_string(), "Read about lifetimes and practice with examples.".to_string());

    
    // Showing of different ways of adding items
    // Again, this causes us to lose ownership of the item
    list.add_initilized_item(item);

    println!("This program takes a bunch of TodoItems, pops them off one by one, and inevitably closes out");
    println!("First, add your own item");
    

    let mut user_title = String::new();
    let mut user_description = String::new();

    println!("Enter the name of the task:");

    // Reading for user_title
    io::stdin()
        .read_line(&mut user_title)
        .expect("Failed to read from stdin");

    println!("Enter the description for the task:");

    // Reading for user_description
    io::stdin()
        .read_line(&mut user_description)
        .expect("Failed to read from stdin");

    // Another important note, user_* is lost due to passing of ownership, again an easy solution if undesired
    // Note I acknowledge that this function call is ugly (The newlines from user input were messing things up)
    list.add_item(user_title.trim().to_string(), user_description.trim().to_string());

    println!("Remember, the one you added last should be the first off (LIFO)\n");

    println!("There are exactly {} items. The last index is {}", list.size, list.size - 1);

    loop {
        let value = list.pop_item();

        match value {
            Some(item) => println!("{}", item.display()),
            None => {
                println!("There was no item to pop!");
                println!("Exiting program.");
                break;
            }
        }
    }
}
