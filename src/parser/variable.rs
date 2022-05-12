use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::{char, digit0, space0},
    character::is_alphanumeric,
    combinator::opt,
    error::{context, VerboseError},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use super::Res;

#[derive(Debug, PartialEq, Eq)]
pub enum MetaType {
    Vector(Type, usize),
    Primitive(Type),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Type {
    Float,
    Integer,
    Custom(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Variable {
    kind: MetaType,
    name: String,
}

impl Variable {
    pub fn new(name: String, kind: MetaType) -> Self {
        Variable { kind, name }
    }
}

impl From<&str> for Type {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "int" => Type::Integer,
            "float" => Type::Float,
            i => Type::Custom(i.to_string()),
        }
    }
}

impl From<(Type, &str)> for MetaType {
    fn from((t, length): (Type, &str)) -> Self {
        match length
            .parse::<usize>()
            .expect("Invalid number while parsing variable name type")
        {
            0 => MetaType::Primitive(t),
            size => MetaType::Vector(t, size),
        }
    }
}

pub fn variable(input: &str) -> Res<&str, Variable> {
    context(
        "variable",
        tuple((variable_name, space0, tag(":"), space0, variable_type)),
    )(input)
    .map(|(next_input, (name, _, _sep, _, vartype))| {
        (next_input, Variable::new(name.to_string(), vartype))
    })
}

fn variable_name(input: &str) -> Res<&str, &str> {
    context("variable name", take_while(valid_variable_char))(input)
        .map(|(next_input, res)| (next_input, res.into()))
}

fn valid_variable_char(chr: char) -> bool {
    chr.is_alphanumeric()
}

fn primitive(input: &str) -> Res<&str, Type> {
    context("primitive", take_while(valid_type_char))(input)
        .map(|(next_input, res)| (next_input, res.into()))
}

fn primitive_with_length(input: &str) -> Res<&str, (Type, &str)> {
    context("primitive with length", primitive)(input)
        .map(|(next_input, res)| (next_input, (res.into(), "0")))
}

fn variable_type(input: &str) -> Res<&str, MetaType> {
    context(
        "variable type",
        alt((
            separated_pair(primitive, tag("^"), digit0),
            primitive_with_length,
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn valid_type_char(chr: char) -> bool {
    chr.is_alphanumeric()
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn variable_name_test() {
        assert_eq!(variable_name("size : int"), Ok((" : int", "size")));
    }

    #[test]
    fn primitive_int_test() {
        assert_eq!(primitive("int"), Ok(("", Type::Integer)));
    }

    #[test]
    fn primitive_float_test() {
        assert_eq!(primitive("float"), Ok(("", Type::Float)));
    }

    #[test]
    fn primitive_custom_test() {
        assert_eq!(
            primitive("custom "),
            Ok((" ", Type::Custom("custom".to_string())))
        );
    }

    #[test]
    fn variable_primitive_test() {
        assert_eq!(
            variable_type("int "),
            Ok((" ", MetaType::Primitive(Type::Integer)))
        );
    }

    #[test]
    fn variable_vector_test() {
        assert_eq!(
            variable_type("float^256 "),
            Ok((" ", MetaType::Vector(Type::Float, 256)))
        );
    }

    #[test]
    fn variable_test() {
        assert_eq!(
            variable("data:float^256"),
            Ok((
                "",
                Variable::new("data".to_string(), MetaType::Vector(Type::Float, 256))
            ))
        );
    }

    #[test]
    fn variable_space_test() {
        assert_eq!(
            variable("data : float^256"),
            Ok((
                "",
                Variable::new("data".to_string(), MetaType::Vector(Type::Float, 256))
            ))
        );
    }
}
