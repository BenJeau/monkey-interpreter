#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Object {
    Integer(isize),
    Boolean(bool),
    Return(Box<Object>),
    Null,
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Return(value) => value.inspect(),
            Object::Null => "null".into(),
        }
    }
}
