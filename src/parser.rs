use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while},
    character::complete::{char, digit0},
    character::is_alphanumeric,
    combinator::opt,
    error::{context, VerboseError},
    sequence::{delimited, separated_pair},
    IResult,
};

use variable::Variable;

mod variable;

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
pub struct Declaration {
    function_name: String,
    arguments: Vec<Variable>,
    outputs: Vec<Variable>,
}

fn val(input: &str) -> Res<&str, &str> {
    context("val", tag("val"))(input).map(|(next_input, res)| (next_input, res.into()))
}

fn parens(input: &str) -> Res<&str, &str> {
    context("parens", delimited(char('('), is_not(")"), char(')')))(input)
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
}
