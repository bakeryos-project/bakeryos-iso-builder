use crate::cmd::root::Root;

fn main() {
    let command: Root = argh::from_env();
    let result = command.handle();

    match result {
        Err(e) => println!("{e}"),
        _ => {}
    }
}

mod cmd;
mod helpers;
mod mkiso;
mod utils;
