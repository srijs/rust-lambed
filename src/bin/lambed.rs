#[cfg(feature = "readline")]
extern crate linenoise;
extern crate lambed;

#[cfg(feature = "readline")]
fn interact<F: Fn(&String)>(f: F) {
    let _ = linenoise::history_load(".lambed_history");
    while let Some(line) = linenoise::input("> ") {
        let _ = linenoise::history_add(&line);
        let _ = linenoise::history_save(".lambed_history");
        f(&line)
    }
}

#[cfg(not(feature = "readline"))]
fn interact<F: Fn(&String)>(f: F) {
    use std::io;
    let mut stdin = io::stdin();
    let mut string = String::new();
    while let Ok(_) = stdin.read_line(&mut string) {
        f(&string);
        string.clear()
    }
}

fn main() {
    interact(|string| {
        match lambed::parser::parse_string(string) {
            Err(err) => println!("{:?}", err),
            Ok((term, _)) => {
                println!("{:?}", term);
                let result = lambed::eval::eval(term);
                println!("{:?}", result)
            }
        }
    })
}
