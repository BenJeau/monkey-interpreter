use super::object::Object;

pub enum Builtin {
    Len,
    Puts,
    Exit,
    First,
    Last,
    Rest,
    Push,
}

impl Builtin {
    pub const fn get(&self) -> Object {
        let function = match self {
            Self::Len => builtin_len,
            Self::Puts => builtin_puts,
            Self::Exit => builtin_exit,
            Self::First => builtin_first,
            Self::Last => builtin_last,
            Self::Rest => builtin_rest,
            Self::Push => builtin_push,
        };

        Object::Builtin(function)
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "len" => Some(Self::Len),
            "puts" => Some(Self::Puts),
            "exit" => Some(Self::Exit),
            "first" => Some(Self::First),
            "last" => Some(Self::Last),
            "rest" => Some(Self::Rest),
            "push" => Some(Self::Push),
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
        Object::Array(value) => Some(Object::Integer(value.len() as isize)),
        Object::Error(value) => Some(Object::Error(value.clone())),
        _ => Some(Object::Error(format!(
            "argument to \"len\" not supported, got {}",
            arguments[0].kind()
        ))),
    }
}

fn builtin_puts(arguments: &[Object]) -> Option<Object> {
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

fn builtin_first(arguments: &[Object]) -> Option<Object> {
    if arguments.len() != 1 {
        return Some(Object::Error(format!(
            "wrong number of arguments. Got {}, expected 1",
            arguments.len()
        )));
    }

    match &arguments[0] {
        Object::Array(value) => Some(value.first().cloned().unwrap_or_default()),
        Object::Error(value) => Some(Object::Error(value.clone())),
        _ => Some(Object::Error(format!(
            "argument to \"first\" not supported, got {}",
            arguments[0].kind()
        ))),
    }
}

fn builtin_last(arguments: &[Object]) -> Option<Object> {
    if arguments.len() != 1 {
        return Some(Object::Error(format!(
            "wrong number of arguments. Got {}, expected 1",
            arguments.len()
        )));
    }

    match &arguments[0] {
        Object::Array(value) => Some(value.last().cloned().unwrap_or_default()),
        Object::Error(value) => Some(Object::Error(value.clone())),
        _ => Some(Object::Error(format!(
            "argument to \"last\" not supported, got {}",
            arguments[0].kind()
        ))),
    }
}

fn builtin_rest(arguments: &[Object]) -> Option<Object> {
    if arguments.len() != 1 {
        return Some(Object::Error(format!(
            "wrong number of arguments. Got {}, expected 1",
            arguments.len()
        )));
    }

    match &arguments[0] {
        Object::Array(value) => {
            let Some((_, rest)) = value.split_at_checked(1) else {
                return Some(Object::Null);
            };

            Some(Object::Array(rest.to_vec()))
        }
        Object::Error(value) => Some(Object::Error(value.clone())),
        _ => Some(Object::Error(format!(
            "argument to \"rest\" not supported, got {}",
            arguments[0].kind()
        ))),
    }
}

fn builtin_push(arguments: &[Object]) -> Option<Object> {
    if arguments.len() != 2 {
        return Some(Object::Error(format!(
            "wrong number of arguments. Got {}, expected 2",
            arguments.len()
        )));
    }

    match (&arguments[0], &arguments[1]) {
        (Object::Array(value), item_to_push) => {
            let mut new_array = value.clone();
            new_array.push(item_to_push.clone());
            Some(Object::Array(new_array))
        }
        (Object::Error(value), Object::Error(items)) => {
            Some(Object::Error(format!("{value} {items}")))
        }
        (Object::Error(value), _) => Some(Object::Error(value.clone())),
        (_, Object::Error(items)) => Some(Object::Error(items.clone())),
        _ => Some(Object::Error(format!(
            "argument to \"push\" not supported, got {}",
            arguments[0].kind()
        ))),
    }
}
