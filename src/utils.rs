use std::result::Result as StdResult;
use std::error::Error;

pub type Result<T> = StdResult<T, Box<Error>>;
