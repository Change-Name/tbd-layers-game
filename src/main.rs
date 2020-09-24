use bevy::prelude::*;
fn main() {
    App::build()
        .add_default_plugins()
        .add_plugin(HelloPlugin)
        .run();
}

struct Person;
struct Name(String);
struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands
        .spawn((Person, Name("First".to_string())))
        .spawn((Person, Name("Second".to_string())))
        .spawn((Person, Name("Third".to_string())));
}

fn greet_people(
    time: Res<Time>, 
    mut timer: ResMut<GreetTimer>, 
    _person: &Person, 
    name: &Name
    ) {
        timer.0.tick(time.delta_seconds);
        if timer.0.finished {
            println!("Hello, {}!", name.0);
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}
