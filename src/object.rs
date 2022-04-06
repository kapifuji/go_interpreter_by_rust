#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(integer) => integer.to_string(),
            Object::Boolean(boolean) => boolean.to_string(),
            Object::Null => "null".to_string(),
            Object::ReturnValue(object) => object.inspect(),
        }
    }
}
