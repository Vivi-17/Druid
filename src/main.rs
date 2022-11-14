use std::sync::{Arc, Mutex};
use druid::{AppLauncher, Data, WindowDesc, Widget, PlatformError};
use druid::widget::{Align, Flex, Label, Padding};

fn build_ui() -> impl Widget<()> {
    //Label::new("Hello world");

    let todo_school = TodoItem {
        category: Category::Work,
        title: "School".to_string(),
        note: Option::from("Homework".to_string()),
        completed: false,
        debug_timestamp: 0
    };

    let todo_play = TodoItem {
        category: Category::Play,
        title: "Play something".to_string(),
        note: Option::from("Anything.".to_string()),
        completed: false,
        debug_timestamp: 0
    };

    /*let list = TodoList {
      items: Arc::new(Vec::from([todo_school, todo_play]))
    };

    let mut text_list = String::new();
    for item in list.items.iter() {
        text_list.push_str(&("- ".to_owned() + &item.title + "\n"));
    }*/

    /*let binding = TodoList {items: Arc::new(Vec::from([todo_school, todo_play]))};
    let todo_list = TodoList::create(&binding);*/
    let todo_list = TodoList::create();

    println!("TEST 1");

    todo_list.add(todo_play);

    todo_list.add(todo_school);

    println!("TEST 2");

    todo_list.remove(TodoItem {
        category: Category::Play,
        title: "Play something".to_string(),
        note: Option::from("Anything.".to_string()),
        completed: false,
        debug_timestamp: 0
    });

    println!("TEST 3");

    let text_list = todo_list.list_to_string();

    println!("TEST 4");

    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::centered(Label::new(text_list)), 1.0),
                1.0))
}

fn main() -> Result<(), PlatformError> {
    AppLauncher::with_window(WindowDesc::new(build_ui())).launch(())?;
    Ok(())
}

//TODO: more methods!
/*
sort( 'condition' )
add( 'list of todos' )
len()
clear()
contains( 'todo' )
[get( 'index' )] -> rather a hashmap
[set( 'index' )] -> rather a hashmap
[insert()]
[copy()]
 */

#[derive(Clone, Data)]
/// The main model for a todo list application.
struct TodoList {
    items: Arc<Mutex<Vec<TodoItem>>>,
    //items: Arc<Vec<TodoItem>>,
}

impl TodoList {
    /*fn create(&self) -> &Arc<Vec<TodoItem>> {
        &self.items
    }*/

    fn create() -> TodoList {
        TodoList {
            items: Arc::new(Mutex::new(Vec::from([])))
           // items: Arc::new(vec![])
        }
    }

    fn add(&self, todo: TodoItem) {
        let cond =  self.items.clone();
        match cond.lock() {
            Ok(mut v) => {
                println!("[ADD] todo: {}", todo.title);
                v.push(todo);
            }
            Err(e) => {
                eprintln!("[ADD] it didnt worked due to: {:?}", e);
            }
        };
    }

    fn remove(&self, todo: TodoItem) {
        let cond =  self.items.clone();
        match cond.lock() {
            Ok(mut v) => {
                println!("[REMOVE] todo: {}", todo.title);
                v.retain(|v| *v != todo);
            }
            Err(e) => {
                eprintln!("[REMOVE] it didnt worked due to: {:?}", e);
            }
        };
    }

    fn list_to_string(&self) -> String {

        if self.items.clone().lock().unwrap().is_empty() {
            println!("LIST IS EMPTY");
        }

        let mut text_list = String::new();
        for item in self.items.clone().lock().unwrap().iter() {
            println!("ITEM: {}", item.title);
            text_list.push_str(&("- ".to_owned() + &item.title + "\n"));
        }
        text_list
    }
}

#[derive(Clone, Data, PartialEq)]
/// A single todo item.
struct TodoItem {
    category: Category,
    title: String,
    note: Option<String>,
    completed: bool,

    // `Data` is implemented for any `Arc`.
    /*
    due_date: Option<Arc<DateTime>>,
    */

    // You can specify a custom comparison fn
    // (anything with the signature (&T, &T) -> bool).
    /*
    #[data(same_fn = "PartialEq::eq")]
    added_date: DateTime,
    */

    // You can specify that a field should
    // be skipped when computing same-ness
    #[data(ignore)]
    debug_timestamp: usize,
}

impl TodoItem {
    fn default() -> TodoItem {
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
enum Category {
    Work,
    Play,
    Revolution,
    Default,
}
