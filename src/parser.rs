use nom::error::VerboseError;
use nom::IResult;
mod variable;
pub mod function;

type Res<T, U> = IResult<T, U, VerboseError<T>>;