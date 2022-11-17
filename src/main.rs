mod testing;

use druid::{AppLauncher, PlatformError, Widget, WindowDesc};
use druid::widget::{Align, Flex, Label, Padding};
use crate::testing::list::*;

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

    todo_list.clear();

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
