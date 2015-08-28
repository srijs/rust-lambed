#[cfg(feature = "readline")]
extern crate readline;
extern crate lambed;

#[cfg(feature = "readline")]
use std::ffi::CString;

#[cfg(feature = "readline")]
fn main() {
    let prompt = CString::new("> ").unwrap();
    while let Ok(s) = readline::readline(&prompt) {
        let bytes = s.to_bytes();
        let string = String::from_utf8_lossy(bytes).into_owned().to_string();
        match lambed::parser::parse_string(&string) {
            Err(err) => println!("{:?}", err),
            Ok((term, _)) => {
                println!("{:?}", term);
                let mut scope = lambed::eval::Scope::new();
                let result = lambed::eval::eval(&mut scope, term);
                match result {
                    Ok(term) => println!("{:?}", term),
                    Err(err) => println!("{:?}", err)
                }
            }
        }
    }
}
