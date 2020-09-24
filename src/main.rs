use bevy::prelude::*;
fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}

fn hi() {
    println!("Hi!");
}

struct Person;
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("First".to_string())))
        .spawn((Person, Name("Second".to_string())))
        .spawn((Person, Name("Third".to_string())));
}

fn greet_people(_person: &Person, name: &Name) {
    println!("Hello, {}!", name.0);
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(add_people.system())
            .add_system(hi.system())
            .add_system(greet_people.system());
    }
}
