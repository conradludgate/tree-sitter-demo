use std::fmt::Debug;

use miette::{Context, IntoDiagnostic, Result};
use tree_sitter::{Language, Parser};

extern "C" {
    fn tree_sitter_go() -> Language;
}

fn main() -> Result<()> {
    let mut parser = Parser::new();
    parser
        .set_language(unsafe { tree_sitter_go() })
        .into_diagnostic()?;

    let name = "examples/example.go";
    let source = std::fs::read_to_string(name)
        .into_diagnostic()
        .context("could not read file contents")?;

    let tree = parser.parse(&source, None).unwrap();

    println!(
        "{:#?}",
        SourceNode {
            src: &source,
            node: tree.root_node(),
        }
    );

    Ok(())
}

pub struct SourceNode<'a> {
    src: &'a str,
    node: tree_sitter::Node<'a>,
}

impl<'a> Debug for SourceNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.node.is_error() {
            let mut tuple = f.debug_tuple(self.node.kind());

            let mut cursor = self.node.walk();
            for child in self.node.named_children(&mut cursor) {
                tuple.field(&SourceNode {
                    src: self.src,
                    node: child,
                });
            }

            tuple.finish()
        } else if self.node.named_child_count() > 0 {
            let mut tuple = f.debug_tuple(self.node.kind());

            let mut cursor = self.node.walk();
            for child in self.node.children(&mut cursor) {
                tuple.field(&SourceNode {
                    src: self.src,
                    node: child,
                });
            }

            tuple.finish()
        } else {
            write!(
                f,
                "{:?}",
                self.node
                    .utf8_text(self.src.as_bytes())
                    .map_err(|_| std::fmt::Error)?
            )
        }
    }
}
