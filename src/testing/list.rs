use std::sync::{Arc, Mutex};
use druid::Data;

//TODO: more methods!
/*
sort( 'condition' )
contains( 'todo' )
[get( 'index' )] -> rather a hashmap
[set( 'index' )] -> rather a hashmap
[insert()]
[copy()]
 */
#[derive(Clone, Data)]
/// The main model for a todo list application.
pub struct TodoList {
    items: Arc<Mutex<Vec<TodoItem>>>,
}

impl TodoList {

    pub fn create() -> TodoList {
        TodoList {
            items: Arc::new(Mutex::new(Vec::from([])))
        }
    }

    pub fn add(&self, todo: TodoItem) {
        self.items.lock().unwrap().push(todo);
    }

    pub fn add_list(&self, todo_list: &mut Vec<TodoItem>) {
        self.items.lock().unwrap().append(todo_list);
    }

    pub fn remove(&self, todo: TodoItem) {
        self.items.lock().unwrap().retain(|x| *x != todo);
    }

    pub fn len(&self) -> usize {
        self.items.lock().unwrap().len()
    }

    pub fn clear(&self) {
        self.items.lock().unwrap().clear();
    }

    pub fn list_to_string(&self) -> String {

        if self.items.lock().unwrap().is_empty() {
            println!("LIST IS EMPTY");
        }

        let mut text_list = String::new();
        for item in self.items.lock().unwrap().iter() {
            println!("ITEM: {}", item.title);
            text_list.push_str(&("- ".to_owned() + &item.title + "\n"));
        }
        text_list
    }
}

#[derive(Clone, Data, PartialEq)]
/// A single todo item.
pub struct TodoItem {
    pub category: Category,
    pub title: String,
    pub note: Option<String>,
    pub completed: bool,

    // `Data` is implemented for any `Arc`.
    /*
    pub due_date: Option<Arc<DateTime>>,
    */

    // You can specify a custom comparison fn
    // (anything with the signature (&T, &T) -> bool).
    /*
    #[data(same_fn = "PartialEq::eq")]
    pub added_date: DateTime,
    */

    // You can specify that a field should
    // be skipped when computing same-ness
    #[data(ignore)]
    pub debug_timestamp: usize,
}

impl TodoItem {
    pub fn default() -> TodoItem {
        TodoItem {
            category: Category::Default,
            title: "".to_string(),
            note: None,
            completed: false,
            debug_timestamp: 0
        }
    }
}

#[derive(Clone, Data, PartialEq)]
/// The three types of tasks in the world.
pub enum Category {
    Work,
    Play,
    Revolution,
    Default,
}