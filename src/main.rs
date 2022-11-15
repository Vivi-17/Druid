use std::sync::{Arc, Mutex};
use druid::{AppLauncher, Data, WindowDesc, Widget, PlatformError};
use druid::widget::{Align, Flex, Label, Padding};

fn build_ui() -> impl Widget<()> {

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

    let todo_def = TodoItem {
        category: Category::Default,
        title: "Something default".to_string(),
        note: Option::from("Nothing special.".to_string()),
        completed: false,
        debug_timestamp: 0
    };

    let mut list = Vec::new();
    let todo_list = TodoList::create();

    println!("TEST 1");

    list.push(todo_school);
    list.push(todo_def);

    todo_list.add_list(&mut list);

    todo_list.add(todo_play);

    println!("TEST 2");

    /*todo_list.remove(TodoItem {
        category: Category::Play,
        title: "Play something".to_string(),
        note: Option::from("Anything.".to_string()),
        completed: false,
        debug_timestamp: 0
    });*/

    println!("TEST 3 - len: {}", todo_list.len());

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
}

impl TodoList {

    fn create() -> TodoList {
        TodoList {
            items: Arc::new(Mutex::new(Vec::from([])))
        }
    }

    fn add(&self, todo: TodoItem) {
        self.items.clone().lock().unwrap().push(todo);
    }

    fn add_list(&self, todo_list: &mut Vec<TodoItem>) {
        self.items.clone().lock().unwrap().append(todo_list);
    }

    fn remove(&self, todo: TodoItem) {
        self.items.clone().lock().unwrap().retain(|x| *x != todo);
    }

    fn len(&self) -> usize {
        self.items.clone().lock().unwrap().len()
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
