#[cfg(feature = "readline")]
extern crate readline;
extern crate lambed;

#[cfg(feature = "readline")]
fn interact<F: Fn(&String)>(f: F) {
    use std::ffi::CString;
    let prompt = CString::new("> ").unwrap();
    while let Ok(s) = readline::readline(&prompt) {
        let bytes = s.to_bytes();
        let string = String::from_utf8_lossy(bytes).into_owned().to_string();
        f(&string)
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
