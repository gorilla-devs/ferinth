use crate::{Result, TraitResult};

pub mod converters;
mod implementation;

pub fn box_err<T>(result: Result<T>) -> TraitResult<T> {
    match result {
        Ok(i) => Ok(i),
        Err(i) => Err(Box::new(i)),
    }
}
