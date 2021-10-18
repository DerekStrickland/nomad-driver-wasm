use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    println!("hello from stdout!");
    eprintln!("hello from stderr!");

    for (key, value) in env::vars() {
        println!("{}={}", key, value);
    }

    let args: Vec<String> = env::args().collect();
    println!("Args are: {:?}", args);
    println!("");

    let path = Path::new("/local/ipsum.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("could not read {}", display).as_str());

    println!("{}", contents);
}
