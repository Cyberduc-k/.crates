pub mod build;
pub mod snippet;
pub mod write;

// use crate::Severity;
// use annotate_snippets::{
//     display_list::{DisplayList, FormatOptions},
//     snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation},
// };
// use std::collections::HashMap;

pub fn emit(diagnostic: &crate::Diagnostic) {
    let mut writer = termcolor::StandardStream::stderr(termcolor::ColorChoice::Auto);
    let snippet = build::build(diagnostic);

    snippet.write(&mut writer).unwrap();
}

/*pub fn emit(diagnostic: &crate::Diagnostic) {
    let mut snippet = Snippet::default();
    let code = diagnostic.code.map(|code| format!("{:0>4}", code));
    let mut files = HashMap::new();

    for label in &diagnostic.labels {
        if let Some(span) = label.span {
            files
                .entry(span.file.name.to_string_lossy().into_owned())
                .or_insert((span.file.source.clone(), Vec::new()))
                .1
                .push(label);
        } else {
            snippet.footer.push(Annotation {
                label: label.message.as_ref().map(|s| s.as_str()),
                id: None,
                annotation_type: label.severity.into(),
            });
        }
    }

    for (file, (source, labels)) in &files {
        let start_offset = labels
            .iter()
            .min_by_key(|l| l.span.unwrap().start.offset)
            .unwrap()
            .span
            .unwrap()
            .line_start(false)
            .offset;
        let end_offset = labels
            .iter()
            .max_by_key(|l| l.span.unwrap().end.offset)
            .unwrap()
            .span
            .unwrap()
            .line_end(false)
            .offset;

        let mut slice = Slice {
            source: &source[start_offset..end_offset],
            line_start: labels.first().unwrap().span.unwrap().start.line + 1,
            origin: Some(file.as_str()),
            fold: false,
            annotations: Vec::new(),
        };

        for label in labels {
            let span = label.span.unwrap();
            let text = label.message.as_ref().map(|s| s.as_str()).unwrap_or("");
            let mut range = (
                span.start.offset - start_offset,
                span.end.offset - start_offset,
            );

            if span.start.line != span.end.line {
                range.0 = span.line_start(false).offset - start_offset;
                range.1 -= 1;
            }

            if range.0 == range.1 {
                range.1 += 1;
            }

            slice.annotations.push(SourceAnnotation {
                label: text,
                annotation_type: label.severity.into(),
                range,
            });
        }

        // slice.annotations.reverse();
        snippet.slices.push(slice);
    }

    snippet.title = Some(Annotation {
        label: Some(&diagnostic.message),
        id: code.as_ref().map(|s| s.as_str()),
        annotation_type: diagnostic.severity.into(),
    });

    snippet.opt = FormatOptions {
        margin: None,
        anonymized_line_numbers: false,
        color: true,
    };

    let display_list = DisplayList::from(snippet);

    println!("{}", display_list);
}

impl Into<AnnotationType> for Severity {
    fn into(self) -> AnnotationType {
        match self {
            Severity::Bug => AnnotationType::Error,
            Severity::Error => AnnotationType::Error,
            Severity::Warning => AnnotationType::Warning,
            Severity::Info => AnnotationType::Info,
            Severity::Help => AnnotationType::Help,
        }
    }
}*/
