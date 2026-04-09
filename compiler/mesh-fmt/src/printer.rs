//! Wadler-Lindig printer for the Mesh format IR.
//!
//! The printer converts a `FormatIR` tree into a formatted string by deciding
//! at each `Group` boundary whether to render flat (all on one line) or broken
//! (with line breaks and indentation). This approach produces optimal layouts
//! that respect the configured line width.

use crate::ir::FormatIR;

/// Configuration for the formatter output.
#[derive(Debug, Clone)]
pub struct FormatConfig {
    /// Number of spaces per indentation level. Default: 2.
    pub indent_size: usize,
    /// Maximum line width before groups break. Default: 100.
    pub max_width: usize,
}

impl Default for FormatConfig {
    fn default() -> Self {
        Self {
            indent_size: 2,
            max_width: 100,
        }
    }
}

/// Whether the current context is rendering flat or broken.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Everything on one line; `Space` renders as " ".
    Flat,
    /// Line breaks at `Space` positions; `Space` renders as newline + indent.
    Break,
}

/// A command on the printer's work stack.
#[derive(Debug)]
struct PrintCmd<'a> {
    indent: usize,
    mode: Mode,
    ir: &'a FormatIR,
}

fn group_fits_on_line(col: usize, flat_width: usize, max_width: usize) -> bool {
    if flat_width == usize::MAX || col > max_width {
        return false;
    }

    flat_width <= max_width.saturating_sub(col)
}

/// Render a `FormatIR` tree as a formatted string respecting the given config.
///
/// The algorithm uses a stack-based approach: at each `Group`, it measures
/// whether the flat rendering fits on the remaining line. If so, the group
/// is rendered flat; otherwise, it switches to broken mode.
pub fn print(ir: &FormatIR, config: &FormatConfig) -> String {
    let mut out = String::new();
    let mut col: usize = 0;
    let mut stack: Vec<PrintCmd> = vec![PrintCmd {
        indent: 0,
        mode: Mode::Break,
        ir,
    }];

    while let Some(cmd) = stack.pop() {
        match cmd.ir {
            FormatIR::Empty => {}

            FormatIR::Text(s) => {
                out.push_str(s);
                col += s.len();
            }

            FormatIR::Space => match cmd.mode {
                Mode::Flat => {
                    out.push(' ');
                    col += 1;
                }
                Mode::Break => {
                    out.push('\n');
                    let indent_str = " ".repeat(cmd.indent);
                    out.push_str(&indent_str);
                    col = cmd.indent;
                }
            },

            FormatIR::Hardline => {
                out.push('\n');
                let indent_str = " ".repeat(cmd.indent);
                out.push_str(&indent_str);
                col = cmd.indent;
            }

            FormatIR::Indent(child) => {
                stack.push(PrintCmd {
                    indent: cmd.indent + config.indent_size,
                    mode: cmd.mode,
                    ir: child,
                });
            }

            FormatIR::Group(child) => {
                // Measure flat width of the group contents.
                let flat_width = measure_flat(child);
                if group_fits_on_line(col, flat_width, config.max_width) {
                    // Fits on one line: render flat.
                    stack.push(PrintCmd {
                        indent: cmd.indent,
                        mode: Mode::Flat,
                        ir: child,
                    });
                } else {
                    // Doesn't fit: render broken.
                    stack.push(PrintCmd {
                        indent: cmd.indent,
                        mode: Mode::Break,
                        ir: child,
                    });
                }
            }

            FormatIR::IfBreak { flat, broken } => match cmd.mode {
                Mode::Flat => {
                    stack.push(PrintCmd {
                        indent: cmd.indent,
                        mode: cmd.mode,
                        ir: flat,
                    });
                }
                Mode::Break => {
                    stack.push(PrintCmd {
                        indent: cmd.indent,
                        mode: cmd.mode,
                        ir: broken,
                    });
                }
            },

            FormatIR::Concat(parts) => {
                // Push in reverse order so the first element is processed first.
                for part in parts.iter().rev() {
                    stack.push(PrintCmd {
                        indent: cmd.indent,
                        mode: cmd.mode,
                        ir: part,
                    });
                }
            }
        }
    }

    // Ensure output ends with a newline (canonical formatting).
    if !out.is_empty() && !out.ends_with('\n') {
        out.push('\n');
    }

    out
}

/// Measure the width of an IR node when rendered flat (all on one line).
///
/// Returns `usize::MAX` if the node contains a `Hardline` (which forces a break
/// and thus can never fit flat on a single line).
fn measure_flat(ir: &FormatIR) -> usize {
    match ir {
        FormatIR::Empty => 0,
        FormatIR::Text(s) => s.len(),
        FormatIR::Space => 1,
        FormatIR::Hardline => usize::MAX, // Forces a break
        FormatIR::Indent(child) => measure_flat(child),
        FormatIR::Group(child) => measure_flat(child),
        FormatIR::IfBreak { flat, .. } => measure_flat(flat),
        FormatIR::Concat(parts) => {
            let mut total: usize = 0;
            for part in parts {
                let w = measure_flat(part);
                if w == usize::MAX {
                    return usize::MAX;
                }
                total = total.saturating_add(w);
                if total == usize::MAX {
                    return usize::MAX;
                }
            }
            total
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::*;

    fn default_config() -> FormatConfig {
        FormatConfig::default()
    }

    #[test]
    fn group_fits_renders_flat() {
        // Group(Concat([Text("a"), Space, Text("b")])) should render as "a b\n"
        let ir = group(concat(vec![text("a"), space(), text("b")]));
        let result = print(&ir, &default_config());
        assert_eq!(result, "a b\n");
    }

    #[test]
    fn group_exceeds_width_breaks() {
        // With max_width=10, a group with 15 chars of text should break.
        let config = FormatConfig {
            indent_size: 2,
            max_width: 10,
        };
        let ir = group(concat(vec![
            text("hello"),
            space(),
            text("beautiful"),
            space(),
            text("world"),
        ]));
        let result = print(&ir, &config);
        assert_eq!(result, "hello\nbeautiful\nworld\n");
    }

    #[test]
    fn hardline_always_breaks() {
        let ir = concat(vec![text("a"), hardline(), text("b")]);
        let result = print(&ir, &default_config());
        assert_eq!(result, "a\nb\n");
    }

    #[test]
    fn indent_adds_spaces() {
        let ir = concat(vec![
            text("fn foo() do"),
            indent(concat(vec![hardline(), text("body")])),
            hardline(),
            text("end"),
        ]);
        let result = print(&ir, &default_config());
        assert_eq!(result, "fn foo() do\n  body\nend\n");
    }

    #[test]
    fn nested_indent() {
        let ir = concat(vec![
            text("a"),
            indent(concat(vec![
                hardline(),
                text("b"),
                indent(concat(vec![hardline(), text("c")])),
            ])),
            hardline(),
            text("d"),
        ]);
        let result = print(&ir, &default_config());
        assert_eq!(result, "a\n  b\n    c\nd\n");
    }

    #[test]
    fn if_break_flat_mode() {
        // Group that fits => flat mode => if_break selects flat variant.
        let ir = group(concat(vec![
            text("("),
            if_break(
                text("x, y"),
                concat(vec![text("x,"), hardline(), text("y")]),
            ),
            text(")"),
        ]));
        let result = print(&ir, &default_config());
        assert_eq!(result, "(x, y)\n");
    }

    #[test]
    fn if_break_broken_mode() {
        // Group that doesn't fit => broken mode => if_break selects broken variant.
        let config = FormatConfig {
            indent_size: 2,
            max_width: 5,
        };
        let ir = group(concat(vec![
            text("("),
            if_break(
                text("x, y, z"),
                concat(vec![
                    hardline(),
                    text("x,"),
                    hardline(),
                    text("y,"),
                    hardline(),
                    text("z"),
                ]),
            ),
            text(")"),
        ]));
        let result = print(&ir, &config);
        assert_eq!(result, "(\nx,\ny,\nz)\n");
    }

    #[test]
    fn empty_produces_nothing() {
        let ir = concat(vec![text("a"), FormatIR::Empty, text("b")]);
        let result = print(&ir, &default_config());
        assert_eq!(result, "ab\n");
    }

    #[test]
    fn nested_groups() {
        // Inner group fits flat, outer group might break.
        let config = FormatConfig {
            indent_size: 2,
            max_width: 20,
        };
        let ir = group(concat(vec![
            text("let x ="),
            space(),
            group(concat(vec![
                text("a"),
                space(),
                text("+"),
                space(),
                text("b"),
            ])),
        ]));
        let result = print(&ir, &config);
        assert_eq!(result, "let x = a + b\n");
    }

    #[test]
    fn space_in_broken_mode_uses_indent() {
        let config = FormatConfig {
            indent_size: 4,
            max_width: 5,
        };
        // "aaaa bbbb" is 9 chars, exceeds max_width=5 => group breaks.
        // Indent applies to the Space -> newline transition.
        let ir = group(indent(concat(vec![text("aaaa"), space(), text("bbbb")])));
        let result = print(&ir, &config);
        assert_eq!(result, "aaaa\n    bbbb\n");
    }

    #[test]
    fn measure_flat_hardline_returns_max() {
        let ir = concat(vec![text("a"), hardline(), text("b")]);
        assert_eq!(measure_flat(&ir), usize::MAX);
    }

    #[test]
    fn grouped_hardline_breaks_instead_of_overflowing() {
        let ir = group(concat(vec![text("a"), hardline(), text("b")]));
        let result = print(&ir, &default_config());
        assert_eq!(result, "a\nb\n");
    }
}
