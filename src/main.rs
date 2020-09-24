use bevy::prelude::*;
fn main() {
    App::build()
        .add_system(hi.system())
        .run();
}

fn hi() {
    println!("Hi!");
}
