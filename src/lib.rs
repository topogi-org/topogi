use crossterm::event::KeyCode;
use topogi_lang::ast::{apply, nil, quote, Exp, Module};
use topogi_renderer::UIEngine;

#[derive(Debug)]
pub struct App {
    state: Exp,
    module: Module,
    ui: UIEngine,
    events: Vec<Event>,
    finished: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub kind: EventKind,
    pub listener: Exp,
}

impl Event {
    pub fn new(kind: EventKind, listener: Exp) -> Self {
        Self { kind, listener }
    }

    pub fn from_exp(exp: &Exp) -> Self {
        let elems = exp.as_list().unwrap();
        assert_eq!(elems[0].as_symbol().unwrap(), "event");
        let kind = EventKind::from_exp(&elems[1]);
        let listener = elems[2].clone();
        Self::new(kind, listener)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventKind {
    KeyPress(KeyCode),
}

impl EventKind {
    pub fn from_exp(exp: &Exp) -> Self {
        let elems = exp.as_list().unwrap();
        match elems[0].as_symbol().unwrap() {
            "key-press" => EventKind::KeyPress(KeyCode::Char(
                elems[1].as_string().unwrap().chars().next().unwrap(),
            )),
            _ => panic!("Invalid event kind"),
        }
    }
}

// TOOD: error handling
impl App {
    pub fn new(source: &str) -> Self {
        let module = topogi_lang::loader::load_module("app", source).unwrap();
        let state = module.run("init", vec![]).unwrap();
        let ui = UIEngine::new().unwrap();

        let evnet_defs = module.run("events", vec![]).unwrap();
        let mut events = vec![];
        for event_def in evnet_defs.as_list().unwrap_or(&[]) {
            events.push(Event::from_exp(event_def));
        }

        Self {
            state,
            module,
            ui,
            events,
            finished: false,
        }
    }

    pub fn add_event(&mut self, kind: EventKind, listener: Exp) {
        self.events.push(Event { kind, listener });
    }

    pub fn poll_event(&mut self) {
        if self.finished {
            return;
        }

        if let Ok(event) = crossterm::event::read() {
            for e in self.events.clone() {
                match &e.kind {
                    EventKind::KeyPress(key) => {
                        if let crossterm::event::Event::Key(k) = event {
                            if k.code == *key {
                                let exp =
                                    self.module.eval(apply(e.listener.clone(), nil())).unwrap();
                                if let Some("quit") = exp.as_symbol() {
                                    self.finish();
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        let exp = self
            .module
            .run("view", vec![quote(self.state.clone())])
            .unwrap();
        self.ui.render(&exp).unwrap();
        self.poll_event();
    }

    pub fn finish(&mut self) {
        self.finished = true;
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn exit(&mut self) {
        self.ui.shutdown().unwrap();
    }
}
