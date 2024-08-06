use crossterm::event::{self, KeyCode};
use topogi_lang::ast::{apply, lambda, list, quote, symbol, Exp, Module};
use topogi_renderer::UIEngine;

#[derive(Debug)]
pub struct App {
    state: Exp,
    module: Module,
    ui: UIEngine,
    event_listeners: Vec<EventListener>,
    finished: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventListener {
    pub kind: EventKind,
    pub callback: Exp,
}

impl EventListener {
    pub fn new(kind: EventKind, listener: Exp) -> Self {
        Self {
            kind,
            callback: listener,
        }
    }

    pub fn from_exp(exp: &Exp) -> Self {
        let elems = exp.as_list().unwrap();
        let kind = EventKind::from_exp(&elems[0]);
        let listener = elems[1].clone();
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
        // panic!("{:#?}", module);
        let state = module.run("init", vec![]).unwrap();
        let ui = UIEngine::new().unwrap();

        let evnets = module.run("event-listener", vec![]).unwrap();
        let mut event_listeners = vec![];
        for event_def in evnets.as_list().unwrap_or(&[]) {
            event_listeners.push(EventListener::from_exp(event_def));
        }

        Self {
            state,
            module,
            ui,
            event_listeners,
            finished: false,
        }
    }

    pub fn poll_event(&mut self) {
        if self.finished {
            return;
        }

        if let Ok(event) = event::read() {
            for listener in self.event_listeners.clone() {
                match &listener.kind {
                    EventKind::KeyPress(key) => {
                        if let crossterm::event::Event::Key(k) = event {
                            if k.code == *key {
                                let exp = self
                                    .module
                                    .eval(apply(listener.callback.clone(), self.state.clone()))
                                    .unwrap();
                                if let Some("quit") = exp.as_symbol() {
                                    self.finish();
                                }
                                self.state = exp;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        let exp = self.module.run("view", vec![self.state.clone()]).unwrap();
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
