pub struct Logger;

impl Logger {
    // Perhaps something like this in the future
    // Error: Unexpected "," in argument list.
    //
    //     15 | function(first, second,);
    //                                ^-- Here.

    pub fn report(line: usize, loc: String, message: String) {
        println!("[line {line}] Error {loc}: {message}");
    }

    pub fn error(line: usize, message: String) {
        Logger::report(line, String::from(""), message);
    }
}
