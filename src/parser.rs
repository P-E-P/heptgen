use nom::{
    IResult,
    sequence::{delimited, separated_pair},
    character::complete::{char, digit0},
    bytes::complete::{is_not, tag, take_while},
    combinator::opt,
    branch::alt,
    error::{VerboseError, context},
    character::is_alphanumeric,
};

type Res<T, U> = IResult<T, U, VerboseError<T>>;

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
        match length.parse::<usize>().expect("Invalid number while parsing variable name type") {
            0 => MetaType::Primitive(t),
            size => MetaType::Vector(t, size),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Variable<'a> {
    kind: Type,
    name: &'a str,
}


#[derive(Debug, PartialEq, Eq)]
pub struct Declaration<'a> {
    function_name: String,
    arguments: Vec<Variable<'a>>,
    outputs: Vec<Variable<'a>>,
}

fn val(input: &str) -> Res<&str, &str> {
    context(
        "val",
        tag("val"),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn variable_name(input: &str) -> Res<&str, &str> {
    context(
        "variable name",
        take_while(valid_variable_char),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn valid_variable_char(chr: char) -> bool {
    chr.is_alphanumeric()
}

fn primitive_type(input: &str) -> Res<&str,(Type, &str)> {
    context(
        "primitive type",
        primitive,
    )(input)
    .map(|(next_input, res)| (next_input, (res, "0")))
}

fn primitive(input: &str) -> Res<&str, Type> {
    context(
        "primitive",
        take_while(valid_type_char),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn variable_type(input: &str) -> Res<&str, MetaType> {
    context(
        "variable type",
        alt((
            separated_pair(primitive, tag("^"), digit0),
            primitive_type,
        )),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn valid_type_char(chr: char) -> bool {
    chr.is_alphanumeric()
}

fn parens(input: &str) -> Res<&str, &str> {
    context(
        "parens",
        delimited(char('('), is_not(")"), char(')')),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn parens_test() {
        assert_eq!(parens("(abcd)"), Ok(("", "abcd")));
    }

    #[test]
    fn val_test() {
        assert_eq!(val("val"), Ok(("", "val")));
    }

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
        assert_eq!(primitive("custom "), Ok((" ", Type::Custom("custom".to_string()))));
    }

    #[test]
    fn variable_type_test() {
        assert_eq!(variable_type("int "), Ok((" ", MetaType::Primitive(Type::Integer))));
    }

}
