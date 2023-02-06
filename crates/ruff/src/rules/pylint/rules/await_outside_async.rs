use crate::ast::types::{FunctionDef, Range, ScopeKind};
use crate::checkers::ast::Checker;
use crate::define_violation;
use crate::registry::Diagnostic;
use crate::violation::Violation;

use ruff_macros::derive_message_formats;
use rustpython_ast::Expr;

define_violation!(
    pub struct AwaitOutsideAsync;
);
impl Violation for AwaitOutsideAsync {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("`await` should be used within an async function")
    }
}

/// PLE1142
pub fn await_outside_async(checker: &mut Checker, expr: &Expr) {
    if !checker
        .current_scopes()
        .find_map(|scope| {
            if let ScopeKind::Function(FunctionDef { async_, .. }) = &scope.kind {
                Some(*async_)
            } else {
                None
            }
        })
        .unwrap_or(true)
    {
        checker.diagnostics.push(Diagnostic::new(
            AwaitOutsideAsync,
            Range::from_located(expr),
        ));
    }
}