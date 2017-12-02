
use nom;

named!(hello_recognizer, tag!("hello"));

pub fn parse() {
    let hello =&b"hello"[..];
    let result = hello_recognizer(hello);
    println!("{:?}", result);
}