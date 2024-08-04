use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use topogi::App;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Usage: topogi <file>")
    }
    let path = &args[1];
    let source = std::fs::read_to_string(path).unwrap();
    let mut app = App::new(&source);

    loop {
        app.update();

        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    app.exit();
}
