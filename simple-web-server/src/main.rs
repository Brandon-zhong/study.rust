use std::fs::File;

fn main() {
    let mut f: File = match File::create("foo.txt") {
        Ok(file) => file,
        Err(err) => {
            println!("error: could not create file. {}", err);
            return;
        }
    };


}
