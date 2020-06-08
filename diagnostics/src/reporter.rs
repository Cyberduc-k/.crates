use crate::{Diagnostic, Severity, Span};
use std::sync::Mutex;

#[derive(Default)]
pub struct Reporter {
    diagnostics: Mutex<Vec<Diagnostic>>,
}

impl Reporter {
    pub fn add(&self, diagnostic: Diagnostic) {
        let is_bug = diagnostic.severity == Severity::Bug;

        self.diagnostics.lock().unwrap().push(diagnostic);

        if is_bug {
            self.report(true);
        }
    }

    pub fn remove(&self, span: Span, code: u16) {
        self.diagnostics
            .lock()
            .unwrap()
            .retain(|diag| !(diag.labels[0].span == Some(span) && diag.code == Some(code)));
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .lock()
            .unwrap()
            .iter()
            .any(|d| d.severity == Severity::Error || d.severity == Severity::Bug)
    }

    pub fn report(&self, exit: bool) {
        self.diagnostics.lock().unwrap().sort_by_key(|d| d.severity);

        for d in self.diagnostics.lock().unwrap().iter() {
            let _ = crate::emit::emit(d);
        }

        if self.has_errors() && exit {
            std::process::exit(0);
        }
    }
}
