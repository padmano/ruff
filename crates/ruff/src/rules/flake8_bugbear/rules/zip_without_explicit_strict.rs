use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::define_violation;
use crate::registry::Diagnostic;
use crate::violation::Violation;
use ruff_macros::derive_message_formats;
use rustpython_ast::{Expr, ExprKind, Keyword};

define_violation!(
    pub struct ZipWithoutExplicitStrict;
);
impl Violation for ZipWithoutExplicitStrict {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("`zip()` without an explicit `strict=` parameter")
    }
}

/// B905
pub fn zip_without_explicit_strict(
    checker: &mut Checker,
    expr: &Expr,
    func: &Expr,
    kwargs: &[Keyword],
) {
    if let ExprKind::Name { id, .. } = &func.node {
        if id == "zip"
            && checker.is_builtin("zip")
            && !kwargs.iter().any(|keyword| {
                keyword
                    .node
                    .arg
                    .as_ref()
                    .map_or(false, |name| name == "strict")
            })
        {
            checker.diagnostics.push(Diagnostic::new(
                ZipWithoutExplicitStrict,
                Range::from_located(expr),
            ));
        }
    }
}