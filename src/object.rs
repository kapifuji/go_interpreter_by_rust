use crate::ast;
use crate::environment;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
    Function {
        parameters: Vec<ast::Expression>,
        body: Box<ast::Statement>,
        environment: environment::Environment,
    },
}

impl Object {
    pub fn is_truthly(&self) -> bool {
        match self {
            Object::Boolean(boolean) => *boolean,
            Object::Null => false,
            _ => true,
        }
    }
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(integer) => integer.to_string(),
            Object::Boolean(boolean) => boolean.to_string(),
            Object::Null => "".to_string(),
            Object::ReturnValue(object) => object.inspect(),
            Object::Function {
                parameters, body, ..
            } => {
                let param_list = parameters
                    .iter()
                    .map(|param| param.to_code())
                    .collect::<Vec<String>>();
                let mut result = "fn(".to_string();
                result.push_str(param_list.join(", ").as_str());
                result.push_str(")");
                result.push_str(body.to_code().as_str());
                result.push_str("\n");

                return result;
            }
        }
    }
}
