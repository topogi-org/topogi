use crossterm::event::KeyCode;
use topogi_lang::ast::{quote, Exp, Module};
use topogi_renderer::UIEngine;

pub struct App {
    state: Exp,
    module: Module,
    ui: UIEngine,
    events: Vec<Event>,
    finished: bool,
}

pub struct Event {
    pub kind: EventKind,
    pub listener: String,
}

pub enum EventKind {
    KeyPress(KeyCode),
}

// TOOD: error handling
impl App {
    pub fn new(source: &str) -> Self {
        let module = topogi_lang::loader::load_module("app", source).unwrap();
        let state = module.run("init", vec![]).unwrap();
        let ui = UIEngine::new().unwrap();

        Self {
            state,
            module,
            ui,
            events: Vec::new(),
            finished: false,
        }
    }

    pub fn add_event(&mut self, kind: EventKind, listener: String) {
        self.events.push(Event { kind, listener });
    }

    pub fn update(&mut self) {
        let exp = self
            .module
            .run("view", vec![quote(self.state.clone())])
            .unwrap();
        self.ui.render(&exp).unwrap();
    }

    pub fn exit(&mut self) {
        self.ui.shutdown().unwrap();
    }
}
