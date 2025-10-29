// It's enough for us
#[derive(Debug, Default)]
pub enum ReturningClause {
    #[default]
    None,
    All,
    Columns(Vec<String>),
}
