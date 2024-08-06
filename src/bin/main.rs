use topogi::App;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: topogi <file>")
    }
    let path = &args[1];
    let source = std::fs::read_to_string(path).unwrap();
    let mut app = App::new(&source);

    while !app.finished() {
        app.update();
    }

    app.exit();
}
