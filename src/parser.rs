use nom::{
    bytes::complete::{tag, take_while},
    character::complete::{char, space0, space1},
    combinator::opt,
    error::{context, VerboseError},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use variable::{variable, Variable};

mod variable;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Declaration {
    pub name: String,
    pub inputs: Vec<Variable>,
    pub outputs: Vec<Variable>,
}

impl Declaration {
    pub fn new(name: String, inputs: Vec<Variable>, outputs: Vec<Variable>) -> Self {
        Declaration {
            name,
            inputs,
            outputs,
        }
    }
}

fn variable_separator(input: &str) -> Res<&str, &str> {
    context("variable separator", tuple((space0, tag(";"), space0)))(input)
        .map(|(next_input, (_, sep, _))| (next_input, sep))
}

fn variable_list(input: &str) -> Res<&str, Vec<Variable>> {
    context(
        "variable list",
        separated_list0(variable_separator, variable),
    )(input)
    .map(|(next_input, res)| (next_input, res))
}

pub fn function_declaration(input: &str) -> Res<&str, Declaration> {
    context(
        "function declaration",
        tuple((
            opt(tag("val ")),
            space0,
            tag("fun"),
            space1,
            function_name,
            space0,
            argument_list,
            space0,
            tag("returns"),
            space0,
            argument_list,
        )),
    )(input)
    .map(
        |(next_input, (_val, _, _fun, _, name, _, inputs, _, _returns, _, outputs))| {
            (
                next_input,
                Declaration::new(name.to_string(), inputs, outputs),
            )
        },
    )
}

fn function_name(input: &str) -> Res<&str, &str> {
    context("function name", take_while(valid_function_char))(input)
        .map(|(next_input, res)| (next_input, res))
}

fn valid_function_char(chr: char) -> bool {
    chr.is_alphanumeric()
}

fn argument_list(input: &str) -> Res<&str, Vec<Variable>> {
    context(
        "argument list",
        tuple((char('('), space0, variable_list, space0, char(')'))),
    )(input)
    .map(|(next_input, (_par_open, _, variables, _, _par_close))| (next_input, variables))
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    use super::variable::{MetaType, Type};

    #[test]
    fn variable_list_test() {
        assert_eq!(
            variable_list("test:float^256 ; test2:float; test3: int"),
            Ok((
                "",
                vec![
                    Variable::new("test".to_string(), MetaType::Vector(Type::Float, 256)),
                    Variable::new("test2".to_string(), MetaType::Primitive(Type::Float)),
                    Variable::new("test3".to_string(), MetaType::Primitive(Type::Integer))
                ]
            ))
        );
    }

    #[test]
    fn argument_list_test() {
        assert_eq!(
            argument_list("( test:float^256 ; test2:float; test3: int)"),
            Ok((
                "",
                vec![
                    Variable::new("test".to_string(), MetaType::Vector(Type::Float, 256)),
                    Variable::new("test2".to_string(), MetaType::Primitive(Type::Float)),
                    Variable::new("test3".to_string(), MetaType::Primitive(Type::Integer))
                ]
            ))
        );
    }

    #[test]
    fn function_declaration_test() {
        assert_eq!(
            function_declaration("val fun function(data: int) returns(o:int)"),
            Ok((
                "",
                Declaration::new(
                    "function".to_string(),
                    vec![Variable::new(
                        "data".to_string(),
                        MetaType::Primitive(Type::Integer)
                    )],
                    vec![Variable::new(
                        "o".to_string(),
                        MetaType::Primitive(Type::Integer)
                    )]
                )
            ))
        );
    }
}
