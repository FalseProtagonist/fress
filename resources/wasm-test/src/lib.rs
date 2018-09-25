// #![feature(custom_attribute)]
#![allow(unused_imports)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_fressian;

use std::fmt;
use serde::ser::{Serialize, Serializer, SerializeMap};

use serde_fressian::de::{self};
use serde_fressian::ser::{self};
use serde_fressian::error::{Error as FressError, ErrorCode};
use serde_fressian::value::{self, Value};
use serde_fressian::wasm::{self};

#[no_mangle]
pub extern "C" fn hello() -> *mut u8
{
    let data = vec![["hello", "from", "wasm!"], ["isn't", "this", "exciting?!"]];
    wasm::to_js(data)
}

#[no_mangle]
pub extern "C" fn big_string() -> *mut u8
{
    let data = vec!["😉 😎 🤔 😐 🙄😉 😎 🤔 😐 🙄😉 😎 🤔 😐 🙄😉 😎 🤔 😐 🙄😉 😎 🤔 😐 🙄😉 😎 🤔 😐 🙄"];
    wasm::to_js(data)
}

#[no_mangle]
pub extern "C" fn echo(ptr: *mut u8, len: usize) ->*mut u8
{
    let val: Result<Value, FressError> = wasm::from_ptr(ptr, len);
    wasm::fress_dealloc(ptr, len); //this will break with STR
    wasm::to_js(val)
}

#[no_mangle]
pub extern "C" fn get_errors() -> *mut u8
{
    let msg = FressError::msg("some message".to_string());
    let unmatched_code = FressError::unmatched_code(42, 43);
    let unsupported = FressError::syntax(ErrorCode::UnsupportedCacheType, 99);

    let errors: Vec<FressError> = vec![msg, unmatched_code, unsupported];

    wasm::to_js(errors)
}


// #[derive(Debug, Serialize)]
#[derive(Debug)]
struct CustomError {
    field_0: String,
}

impl std::error::Error for CustomError {
    fn description(&self) -> &str {
        "A custom Error"
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A custom Error")
    }
}

impl Serialize for CustomError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map_state = serializer.serialize_map(None)?;

        map_state.serialize_key("type")?;
        map_state.serialize_value("test_lib_error")?;

        map_state.serialize_key("field_0")?;
        map_state.serialize_value(&self.field_0)?;

        map_state.end()
    }
}

#[no_mangle]
pub extern "C" fn get_custom_error() -> *mut u8
{
    let err: CustomError = CustomError{field_0: "some message".to_string()};

    let res: Result<(), CustomError> = Err(err);

    wasm::to_js(res)
}


#[no_mangle]
pub extern "C" fn induce_panic()
{
    assert_eq!(2 + 2, 5)
}

