use std::error::Error;
use csv;

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    //   Result<T, E> is Rust’s way of saying:
    // → Success: return T
    // → Error: return E
    // Here success is () (unit type = “nothing to return”).
    // Errors are wrapped in Box<dyn Error> → “any kind of error in a heap-allocated box.”

    let mut reader = csv::Reader::from_path(path)?; // `?` is Rust’s error propagation operator: If it succeeds → return the reader. If it fails → immediately return the error (Err) from this function.

    for result in reader.records(){
        let record = result?;

        println!("{:?}", record) //{:?} is the debug format (shows the record in a human-readable way).
    }
    Ok(()) //After reading all rows successfully, return Ok(()) → success, no errors.
}

fn main() {
    if let Err(e) = read_from_file("./example.csv") {
        eprintln!("{}", e) //prints error if there is one. Not println but eprintln!
    }
}
