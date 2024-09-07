#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    Null,
}

impl Object {
    fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Null => "null".into(),
        }
    }
}
