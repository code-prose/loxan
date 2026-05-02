pub struct Logger;

impl Logger {
    // Perhaps something like this in the future
    // Error: Unexpected "," in argument list.
    //
    //     15 | function(first, second,);
    //                                ^-- Here.

    pub fn report(line: i32, loc: String, message: String) {
        println!("[line {line}] Error {loc}: {message}");
    }

    pub fn error(line: i32, message: String) {
        Logger::report(line, String::from(""), message);
    }
}
