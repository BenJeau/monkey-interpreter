use super::object::Object;

pub enum Builtin {
    Len,
    Print,
    Println,
    Exit,
}

impl Builtin {
    pub const fn get(&self) -> Object {
        let function = match self {
            Self::Len => builtin_len,
            Self::Print => builtin_print,
            Self::Println => builtin_println,
            Self::Exit => builtin_exit,
        };

        Object::Builtin(function)
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "len" => Some(Self::Len),
            "print" => Some(Self::Print),
            "println" => Some(Self::Println),
            "exit" => Some(Self::Exit),
            _ => None,
        }
    }
}

fn builtin_len(arguments: &[Object]) -> Option<Object> {
    if arguments.len() != 1 {
        return Some(Object::Error(format!(
            "wrong number of arguments. Got {}, expected 1",
            arguments.len()
        )));
    }

    match &arguments[0] {
        Object::String(value) => Some(Object::Integer(value.len() as isize)),
        Object::Error(value) => Some(Object::Error(value.clone())),
        _ => Some(Object::Error(format!(
            "argument to \"len\" not supported, got {}",
            arguments[0].kind()
        ))),
    }
}

fn builtin_print(arguments: &[Object]) -> Option<Object> {
    for argument in arguments {
        print!("{}", argument.inspect());
    }
    println!();
    Some(Object::Null)
}

fn builtin_println(arguments: &[Object]) -> Option<Object> {
    for argument in arguments {
        println!("{}", argument.inspect());
    }
    Some(Object::Null)
}

fn builtin_exit(arguments: &[Object]) -> Option<Object> {
    if arguments.len() > 1 {
        return Some(Object::Error(format!(
            "wrong number of arguments. Got {}, expected 0 or 1",
            arguments.len()
        )));
    }

    match arguments.first().unwrap_or(&Object::Integer(0)) {
        Object::Integer(value) => std::process::exit(*value as i32),
        Object::Error(value) => Some(Object::Error(value.clone())),
        _ => Some(Object::Error(format!(
            "argument to \"exit\" not supported, got {}",
            arguments[0].kind()
        ))),
    }
}
