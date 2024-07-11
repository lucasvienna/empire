mod db;
mod models;
mod schema;

use db::lib;
use db::users::create_user;
use std::io::stdin;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("Starting Empire server...");
    let exit_code = loop {
        // read message queue
        read();
        break tick();
    };

    let connection = &mut lib::establish_connection();

    let mut name = String::new();
    println!("What is your username?");
    stdin().read_line(&mut name).unwrap();
    let title = name.trim_end(); // Remove the trailing newline
    let user = create_user(connection, title);
    println!("\nSaved user with ID {}", user.id);

    ExitCode::from(exit_code)
}

/**
 * Main game loop
 */
fn tick() -> u8 {
    println!("Tick");
    0
}

fn read() {}
