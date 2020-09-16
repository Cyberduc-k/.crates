use super::snippet::*;
use crate::{Diagnostic, FileId};

pub fn build(diag: &Diagnostic) -> Snippet {
    let mut snippet = Snippet {
        severity: diag.severity,
        message: &diag.message,
        code: diag.code,
        parts: Vec::new(),
    };

    fn add_annotation_to_file<'a>(
        files: &mut Vec<SnippetPart<'a>>,
        file: FileId,
        idx: usize,
        ann: Annotation<'a>,
    ) {
        for part in files.iter_mut() {
            if part.file.name == file.name {
                for line in &mut part.lines {
                    if line.idx == idx {
                        line.annotations.push(ann);
                        return;
                    }
                }

                part.lines.push(Line {
                    idx,
                    annotations: vec![ann],
                });
                part.lines.sort();
                return;
            }
        }

        files.push(SnippetPart {
            file,
            span: ann.span,
            lines: vec![Line {
                annotations: vec![ann],
                idx,
            }],
        });
    }

    let mut multiline = Vec::new();

    for lbl in &diag.labels {
        if let None = lbl.span {
            continue;
        }

        let span = lbl.span.unwrap();
        let lo = span.start;
        let mut hi = span.end;

        if lo.col == hi.col && lo.line == hi.line {
            hi.col += 1;
        }

        if lo.line != hi.line {
            multiline.push((
                span.file,
                MultilineAnnotation {
                    severity: lbl.severity,
                    depth: 1,
                    span,
                    line_start: lo.line,
                    line_end: hi.line,
                    start_col: lo.col,
                    end_col: hi.col,
                    label: lbl.message.as_deref(),
                    overlaps_exactly: false,
                },
            ));
        } else {
            add_annotation_to_file(
                &mut snippet.parts,
                span.file,
                lo.line,
                Annotation {
                    severity: lbl.severity,
                    span,
                    start: lo.col,
                    end: hi.col,
                    label: lbl.message.as_deref(),
                    kind: AnnotationKind::Single,
                },
            );
        }
    }

    multiline.sort_by_key(|&(_, ref ml)| (ml.line_start, ml.line_end));

    for (_, ann) in multiline.clone() {
        for (_, a) in multiline.iter_mut() {
            if !ann.same_span(a)
                && num_overlap(ann.line_start, ann.line_end, a.line_start, a.line_end, true)
            {
                a.depth += 1;
            } else if ann.same_span(a) && &ann != a {
                a.overlaps_exactly = true;
            } else {
                break;
            }
        }
    }

    for (file, ann) in multiline {
        let mut end_ann = ann.as_end();

        if !ann.overlaps_exactly {
            add_annotation_to_file(&mut snippet.parts, file, ann.line_start, ann.as_start());

            let middle = std::cmp::min(ann.line_start + 4, ann.line_end);

            for line in ann.line_start + 1..middle {
                add_annotation_to_file(&mut snippet.parts, file, line, ann.as_line());
            }

            let line_end = ann.line_end - 1;

            if middle < line_end {
                add_annotation_to_file(&mut snippet.parts, file, line_end, ann.as_line());
            }
        } else {
            end_ann.kind = AnnotationKind::Single;
        }

        add_annotation_to_file(&mut snippet.parts, file, ann.line_end, end_ann);
    }

    snippet.finalize();
    snippet
}

fn num_overlap(
    a_start: usize,
    a_end: usize,
    b_start: usize,
    b_end: usize,
    inclusive: bool,
) -> bool {
    let extra = if inclusive { 1 } else { 0 };
    (b_start..b_end + extra).contains(&a_start) || (a_start..a_end + extra).contains(&b_start)
}
