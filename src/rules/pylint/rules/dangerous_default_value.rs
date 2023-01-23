use rustpython_ast::Arguments;
use crate::violation::Violation;
use ruff_macros::derive_message_formats;
use crate::define_violation;

use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::{Diagnostic, DiagnosticKind};

/// PLW0102
pub fn dangerous_default_value(checker: &mut Checker, arguments: &Arguments) {
    for expr in arguments
        .defaults
        .iter()
        .chain(arguments.kw_defaults.iter())
    {
        match &expr.node {
            rustpython_ast::ExprKind::Dict { .. } => {
                checker.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::DangerousDefaultValue(DangerousDefaultValue {
                        value: "Dict".to_string(),
                    }),
                    Range::from_located(expr),
                ))
            }
            rustpython_ast::ExprKind::List { .. } => {
                checker.diagnostics.push(Diagnostic::new(
                    DiagnosticKind::DangerousDefaultValue(DangerousDefaultValue {
                        value: "List".to_string()
                    }),
                    Range::from_located(expr),
                ))
            },
            _ => {}
        }
    }
}

define_violation!(
    pub struct DangerousDefaultValue {
        pub value: String,
    }
);
impl Violation for DangerousDefaultValue {
    #[derive_message_formats]
    fn message(&self) -> String {
        let DangerousDefaultValue { value } = self;
        format!(
            "Dangerous default value {value} used as argument, consider defaulting to None and assigning \
            inside the function body"
        )
    }
}
