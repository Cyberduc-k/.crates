use crate::{FileId, Severity, Span};

pub struct Snippet<'a> {
    pub severity: Severity,
    pub message: &'a str,
    pub code: Option<u16>,
    pub parts: Vec<SnippetPart<'a>>,
}

pub struct SnippetPart<'a> {
    pub file: FileId,
    pub span: Span,
    pub lines: Vec<Line<'a>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line<'a> {
    pub idx: usize,
    pub annotations: Vec<Annotation<'a>>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Annotation<'a> {
    pub severity: Severity,
    pub span: Span,
    pub start: usize,
    pub end: usize,
    pub label: Option<&'a str>,
    pub kind: AnnotationKind,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnnotationKind {
    Single,
    MultiStart(usize),
    MultiLine(usize),
    MultiEnd(usize),
}

#[derive(Clone, PartialEq)]
pub struct MultilineAnnotation<'a> {
    pub severity: Severity,
    pub depth: usize,
    pub span: Span,
    pub line_start: usize,
    pub line_end: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub label: Option<&'a str>,
    pub overlaps_exactly: bool,
}

impl Snippet<'_> {
    pub fn finalize(&mut self) {
        for part in &mut self.parts {
            part.lines.retain(|l| {
                l.annotations
                    .iter()
                    .any(|a| !matches!(a.kind, AnnotationKind::MultiLine(_)))
            });

            for line in &mut part.lines {
                line.annotations.sort_by_key(|a| a.start);
                line.annotations.reverse();
            }
        }
    }
}

impl SnippetPart<'_> {
    pub fn max_depth(&self) -> usize {
        self.lines
            .iter()
            .flat_map(|l| {
                l.annotations.iter().filter_map(|a| match a.kind {
                    AnnotationKind::MultiStart(d) => Some(d),
                    AnnotationKind::MultiLine(d) => Some(d),
                    AnnotationKind::MultiEnd(d) => Some(d),
                    _ => None,
                })
            })
            .max()
            .unwrap_or(0)
    }
}

impl<'a> MultilineAnnotation<'a> {
    /// Compare two `MultilineAnnotation`s considering only the `Span` they cover.
    pub fn same_span(&self, other: &Self) -> bool {
        self.line_start == other.line_start
            && self.line_end == other.line_end
            && self.start_col == other.start_col
            && self.end_col == other.end_col
    }

    pub fn as_start(&self) -> Annotation<'a> {
        Annotation {
            severity: self.severity,
            span: self.span,
            start: self.start_col,
            end: self.start_col + 1,
            label: None,
            kind: AnnotationKind::MultiStart(self.depth),
        }
    }

    pub fn as_end(&self) -> Annotation<'a> {
        Annotation {
            severity: self.severity,
            span: self.span,
            start: self.end_col.saturating_sub(1),
            end: self.end_col,
            label: self.label.clone(),
            kind: AnnotationKind::MultiEnd(self.depth),
        }
    }

    pub fn as_line(&self) -> Annotation<'a> {
        Annotation {
            severity: self.severity,
            span: self.span,
            start: 0,
            end: 0,
            label: None,
            kind: AnnotationKind::MultiLine(self.depth),
        }
    }
}
