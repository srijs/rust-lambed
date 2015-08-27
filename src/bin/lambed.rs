extern crate readline;
extern crate lambed;

use std::ffi::CString;

fn main() {
    let prompt = CString::new("> ").unwrap();
    while let Ok(s) = readline::readline(&prompt) {
        let bytes = s.to_bytes();
        let string = String::from_utf8_lossy(bytes).into_owned().to_string();
        match lambed::parser::parse_string(&string) {
            Err(err) => println!("{:?}", err),
            Ok((term, _)) => {
                let mut context = lambed::eval::Context::new();
                let result = lambed::eval::eval(&mut context, term);
                match result {
                    Ok(term) => println!("{:?}", term),
                    Err(err) => println!("{:?}", err)
                }
            }
        }
    }
}
