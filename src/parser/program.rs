use crate::ast::Statement;

#[derive(Default)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for statement in &self.statements {
            write!(f, "{statement}")?;
        }
        Ok(())
    }
}
