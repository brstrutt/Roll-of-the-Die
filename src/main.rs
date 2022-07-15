use bevy::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::debug!("App is starting");
    App::new()
        .add_startup_system(add_people)
        .add_system(greet_people)
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    log::debug!("Hello World");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Vanessa Belakor".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        log::debug!("hello {}!", name.0);
    }
}