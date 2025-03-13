pub mod todo_list {
    pub struct TodoList {
        pub size: usize,
        v: Vec<TodoItem>,
    }

    impl TodoList {
        pub fn new() -> Self {
            Self {
                size: 0,
                v: Vec::new(),
            }
        }

        pub fn add_item(&mut self, title: String, desc: String) {
            self.v.push(TodoItem::new(title, desc));
            self.size += 1;
        }

        pub fn add_initilized_item(&mut self, item: TodoItem) {
            self.v.push(item);
            self.size += 1;
        }

        pub fn remove(&mut self, index: usize) -> Option<TodoItem> {
            if index < self.size {
                Some(self.v.remove(index))
            }
            else {
                None
            }
        }

        pub fn pop_item(&mut self) -> Option<TodoItem> {
            let value = self.v.pop();
            match value {
                Some(item) => {
                    self.size -= 1;
                    Some(item)
                }
                None => None
            }
        }

        pub fn display_list(&self) {
            for value in &self.v {
                println!("{}", value.display());
            }
        }
    }

    // I initially wanted to make TodoItem private, but I thought of initializing and also wanted a remove / pop feature
    // and using the Option<TodoItem> was a fun learning experience
    pub struct TodoItem {
        title: String,
        description: String,
    }

    impl TodoItem {
        pub fn new(title: String, desc: String) -> Self {
            Self {
                title: title,
                description: desc,
            }
        }

        pub fn get_title(&self) -> &String {
            &self.title
        }

        pub fn get_description(&self) -> &String {
            &self.description
        }

        pub fn display(&self) -> String {
            format!("{{ Title: {} | Description: {} }}", self.title, self.description)
        }
    }
}