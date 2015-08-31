extern crate copperline;
extern crate lambed;

fn interact<F: Fn(&String)>(f: F) {
    let mut c = copperline::Copperline::new();
    while let Ok(line) = c.read_line("> ") {
        f(&line);
        c.add_history(line);
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
