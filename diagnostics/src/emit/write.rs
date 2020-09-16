use super::snippet::*;
use crate::FileId;
use termcolor::{Color, ColorSpec, WriteColor};

impl Snippet<'_> {
    pub fn write(&self, mut writer: impl WriteColor) -> std::io::Result<()> {
        let mut blue = ColorSpec::new();
        blue.set_fg(Some(Color::Blue)).set_intense(true);
        let code = if let Some(code) = &self.code {
            format!("[{:0>4}]", code)
        } else {
            String::new()
        };

        writer.set_color(&self.severity.color())?;
        write!(writer, "{}{}", self.severity.to_string(), code)?;
        writer.reset()?;
        write!(writer, ": ")?;
        writer.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::White))
                .set_intense(true),
        )?;
        writeln!(writer, "{}", self.message)?;

        for part in &self.parts {
            let margin = part
                .lines
                .iter()
                .map(|l| l.idx + 1)
                .max()
                .unwrap_or(1)
                .to_string()
                .len();

            writer.set_color(&blue)?;
            write!(writer, "{}-->", " ".repeat(margin))?;
            writer.reset()?;
            writeln!(writer, " {}", part.file.name.display())?;

            let max_depth = part.max_depth();

            for (i, line) in part.lines.iter().enumerate() {
                let dots = i + 1 < part.lines.len() && part.lines[i + 1].idx != line.idx + 1;

                line.write(&mut writer, part.file, margin, max_depth, dots)?;
            }
        }

        Ok(())
    }
}

impl Line<'_> {
    pub fn write(
        &self,
        mut writer: impl WriteColor,
        file: FileId,
        margin: usize,
        max_depth: usize,
        dots: bool,
    ) -> std::io::Result<()> {
        let mut blue = ColorSpec::new();
        blue.set_fg(Some(Color::Blue)).set_intense(true);
        let num = (self.idx + 1).to_string();

        writer.set_color(&blue)?;
        write!(writer, "{}{} | ", " ".repeat(margin - num.len()), num)?;
        writer.reset()?;

        let mut before = vec![(' ', ColorSpec::new()); max_depth];

        for ann in &self.annotations {
            if let AnnotationKind::MultiStart(x) = ann.kind {
                if ann.start == 0 {
                    before[x - 1] = ('╭', ann.severity.color());
                }
            } else if let AnnotationKind::MultiLine(x) = ann.kind {
                before[x - 1] = ('│', ann.severity.color());
            } else if let AnnotationKind::MultiEnd(x) = ann.kind {
                before[x - 1] = ('│', ann.severity.color());
            }
        }

        for (sym, color) in &before {
            writer.set_color(color)?;
            write!(writer, "{}", sym)?;
        }

        if !before.is_empty() {
            writer.reset()?;
            write!(writer, " ")?;
        }

        writeln!(writer, "{}", file.source.lines().nth(self.idx).unwrap())?;

        for ann in &self.annotations {
            let label = if let Some(lbl) = ann.label { lbl } else { "" };

            match ann.kind {
                AnnotationKind::Single => {
                    writer.set_color(&blue)?;
                    write!(writer, "{} | ", " ".repeat(margin))?;

                    for (sym, color) in &before {
                        writer.set_color(color)?;
                        write!(writer, "{}", sym)?;
                    }

                    writer.set_color(&ann.severity.color())?;

                    if !before.is_empty() {
                        write!(writer, " ")?;
                    }

                    write!(
                        writer,
                        "{}{} {}",
                        " ".repeat(ann.start),
                        ann.severity.symbol().repeat(ann.end - ann.start),
                        label
                    )?;
                    writer.reset()?;
                    writeln!(writer)?;
                }
                AnnotationKind::MultiStart(x) => {
                    if ann.start != 0 {
                        writer.set_color(&blue)?;
                        write!(writer, "{} | ", " ".repeat(margin))?;
                        before[x - 1] = ('┌', ann.severity.color());

                        let mut repl = Vec::new();

                        for i in x..before.len() {
                            if before[i].0 == ' ' {
                                let old =
                                    std::mem::replace(&mut before[i], ('─', ann.severity.color()));

                                repl.push((i, old));
                            }
                        }

                        for (sym, color) in &before {
                            writer.set_color(color)?;
                            write!(writer, "{}", sym)?;
                        }

                        writer.set_color(&ann.severity.color())?;
                        write!(writer, "─")?;
                        before[x - 1].0 = '│';

                        for (i, old) in repl {
                            before[i] = old;
                        }

                        write!(
                            writer,
                            "{}{} {}",
                            "─".repeat(ann.start),
                            ann.severity.symbol().repeat(ann.end - ann.start),
                            label
                        )?;
                        writer.reset()?;
                        writeln!(writer)?;
                    }
                }
                AnnotationKind::MultiLine(_) => {}
                AnnotationKind::MultiEnd(x) => {
                    writer.set_color(&blue)?;
                    write!(writer, "{} | ", " ".repeat(margin))?;
                    before[x - 1].0 = '└';

                    let mut repl = Vec::new();

                    for i in x..before.len() {
                        if before[i].0 == ' ' {
                            let old =
                                std::mem::replace(&mut before[i], ('─', ann.severity.color()));

                            repl.push((i, old));
                        }
                    }

                    for (sym, color) in &before {
                        writer.set_color(color)?;
                        write!(writer, "{}", sym)?;
                    }

                    writer.set_color(&ann.severity.color())?;
                    write!(writer, "─")?;
                    before[x - 1] = (' ', ColorSpec::new());

                    for (i, old) in repl {
                        before[i] = old;
                    }

                    write!(
                        writer,
                        "{}{} {}",
                        "─".repeat(ann.start),
                        ann.severity.symbol().repeat(ann.end - ann.start),
                        label
                    )?;
                    writer.reset()?;
                    writeln!(writer)?;
                }
            }
        }

        if dots {
            writer.set_color(&blue)?;
            write!(writer, "...")?;
            writer.reset()?;
            write!(writer, "{}", " ".repeat(margin))?;

            for (sym, color) in &before {
                writer.set_color(color)?;
                write!(writer, "{}", sym)?;
            }

            writer.reset()?;
            writeln!(writer)?;
        }

        Ok(())
    }
}
