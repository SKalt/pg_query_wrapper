use prost::Message;
use std::{
    ffi::{CStr, CString, IntoStringError, NulError},
    usize,
};

mod bindings;
mod pbuf;

/// Safely copy a slice of bytes from a raw pointer to a C char array.
fn read_buf_from_ptr(ptr: *mut ::std::os::raw::c_char) -> Result<Vec<u8>, Failure> {
    if ptr == std::ptr::null_mut::<::std::os::raw::c_char>() {
        return Err(Failure::NullPtr);
    } else {
        let cstr = unsafe { CStr::from_ptr(ptr) };
        let borrowed_bytes = cstr.to_bytes();
        // important! Allocate new memory that we own to the borrowed bytes can be freed
        let mut owned_bytes = vec![0u8; borrowed_bytes.len()];
        owned_bytes.copy_from_slice(borrowed_bytes);
        return Ok(owned_bytes);
    }
}

/// Copy an owned String from a pointer to a C char array.
fn read_string_from_ptr(ptr: *mut ::std::os::raw::c_char) -> Result<String, Failure> {
    let bytes = read_buf_from_ptr(ptr)?;
    let result = std::str::from_utf8(bytes.as_slice())?;
    return Ok(String::from(result));
}

/// an owned version of the C equivalent in pg_query.h.
#[derive(Debug)]
pub struct PgQueryError {
    pub message: String,
    pub funcname: String,
    pub filename: String,
    pub lineno: i32,
    pub cursorpos: i32,
    pub context: String,
}

impl PgQueryError {
    fn from_original(original: *mut bindings::PgQueryError) -> Result<Self, Failure> {
        if original != std::ptr::null_mut::<bindings::PgQueryError>() {
            let orig = unsafe { *original };
            let message = read_string_from_ptr(orig.message)?;
            let funcname = read_string_from_ptr(orig.funcname)?;
            let filename = read_string_from_ptr(orig.filename)?;
            let context = read_string_from_ptr(orig.context)?;
            let lineno = orig.lineno.clone();
            let cursorpos = orig.cursorpos.clone();
            return Ok(PgQueryError {
                message,
                funcname,
                filename,
                context,
                lineno,
                cursorpos,
            });
        } else {
            return Err(Failure::NullPtr);
        }
    }
}

/// All the ways that I know of that this library can fail.
#[derive(Debug)]
pub enum Failure {
    PgQueryError(PgQueryError),
    NulError(NulError),
    IntoStringError(std::ffi::IntoStringError),
    NullPtr,
    DecodeError(prost::DecodeError),
    Utf8Error(std::str::Utf8Error),
}

impl From<NulError> for Failure {
    fn from(e: NulError) -> Self {
        Self::NulError(e)
    }
}
impl From<IntoStringError> for Failure {
    fn from(e: IntoStringError) -> Self {
        Self::IntoStringError(e)
    }
}
impl From<PgQueryError> for Failure {
    fn from(e: PgQueryError) -> Self {
        Self::PgQueryError(e)
    }
}

impl From<prost::DecodeError> for Failure {
    fn from(e: prost::DecodeError) -> Self {
        Self::DecodeError(e)
    }
}
impl From<std::str::Utf8Error> for Failure {
    fn from(e: std::str::Utf8Error) -> Self {
        Self::Utf8Error(e)
    }
}

/// see pb_query_normalize_query().
pub fn normalize_query(query: &str) -> Result<String, Failure> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_normalize(input.as_ptr());
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let err_result = PgQueryError::from_original(result.error);
            bindings::pg_query_free_normalize_result(result);
            match err_result {
                Err(error) => return Err(error),
                Ok(error) => return Err(Failure::PgQueryError(error)),
            };
        } else {
            let normalized_result = read_string_from_ptr(result.normalized_query);
            bindings::pg_query_free_normalize_result(result);
            match normalized_result {
                Err(error) => return Err(error),
                Ok(normalized_query) => return Ok(normalized_query),
            };
        }
    }
}

/// equivalent to pb_query_scan() in pg_query.h.
pub fn scan(query: &str) -> Result<pbuf::ScanResult, Failure> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_scan(input.as_ptr());
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let err_result = PgQueryError::from_original(result.error);
            bindings::pg_query_free_scan_result(result);
            match err_result {
                Err(error) => return Err(error),
                Ok(error) => return Err(Failure::PgQueryError(error)),
            };
        } else {
            // resultpbuf.data is a pointer to a character array
            match read_buf_from_ptr(result.pbuf.data) {
                Err(error) => {
                    bindings::pg_query_free_scan_result(result); // the original buffer gets freed here
                    return Err(error);
                }
                Ok(buf) => {
                    match pbuf::ScanResult::decode(buf.as_slice()) {
                        Err(error) => {
                            bindings::pg_query_free_scan_result(result); // the original buffer gets freed here
                            return Err(Failure::DecodeError(error));
                        }
                        Ok(scan_result) => {
                            bindings::pg_query_free_scan_result(result); // the original buffer gets freed here
                            return Ok(scan_result);
                        }
                    };
                }
            };
            // assert_eq!(
            //     buf.len() as u32, buf_holder.len,
            //     "copied pbuf.data buffer length {} != nominal length {}",
            //     buf.len(), buf_holder.len
            // );
        }
    }
}

/// equivalent to `pb_query_parse()` in pb_query.h. Results in a JSON string of the AST.
pub fn parse_to_json(query: &str) -> Result<String, Failure> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_parse(input.as_ptr());
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let err_result = PgQueryError::from_original(result.error);
            bindings::pg_query_free_parse_result(result);
            match err_result {
                Err(error) => Err(error),
                Ok(error) => Err(Failure::PgQueryError(error)),
            }
        } else if result.parse_tree == std::ptr::null_mut::<::std::os::raw::c_char>() {
            bindings::pg_query_free_parse_result(result);
            return Err(Failure::NullPtr);
        } else {
            let result_json = read_string_from_ptr(result.parse_tree)?;
            bindings::pg_query_free_parse_result(result);
            return Ok(result_json);
        }
    }
}

/// parses *sql* to its protobuf equivalent
pub fn parse_to_protobuf(query: &str) -> Result<pbuf::ParseResult, Failure> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_parse_protobuf(input.as_ptr());
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let error_result = PgQueryError::from_original(result.error);
            bindings::pg_query_free_protobuf_parse_result(result);
            match error_result {
                Err(error) => return Err(error),
                Ok(error) => return Err(Failure::PgQueryError(error)),
            }
        } else {
            // buf_holder.data is a pointer to a character array
            match read_buf_from_ptr(result.parse_tree.data) {
                Err(read_err) => return Err(read_err),
                Ok(buf) => {
                    let parse_result = pbuf::ParseResult::decode(buf.as_slice());
                    bindings::pg_query_free_protobuf_parse_result(result);
                    // assert_eq!(
                    //     buf.len() as u32, buf_holder.len,
                    //     "copied pbuf.data buffer length {} != nominal length {}",
                    //     buf.len(), buf_holder.len
                    // );
                    match parse_result {
                        Err(error) => return Err(Failure::DecodeError(error)),
                        Ok(parsed) => return Ok(parsed),
                    };
                }
            };
        }
    }
}

#[test]
fn test_parsing_to_pbuf() {
    assert!(parse_to_protobuf("select 1;").is_ok())
}

/// equivalent to pg_query_parse_plpgsql
pub fn parse_plpgsql(query: &str) -> Result<String, Failure> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_parse_plpgsql(input.as_ptr());
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let error_result = PgQueryError::from_original(result.error);
            bindings::pg_query_free_plpgsql_parse_result(result);
            match error_result {
                Err(error) => return Err(error),
                Ok(error) => return Err(Failure::PgQueryError(error)),
            }
        } else if result.plpgsql_funcs == std::ptr::null_mut::<::std::os::raw::c_char>() {
            bindings::pg_query_free_plpgsql_parse_result(result);
            return Err(Failure::NullPtr);
        } else {
            let result_json = read_string_from_ptr(result.plpgsql_funcs)?;
            bindings::pg_query_free_plpgsql_parse_result(result);
            return Ok(result_json);
        }
    }
}

pub fn split_statements_with_scanner(query: &str) -> Result<Vec<&str>, Failure> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_split_with_scanner(input.as_ptr());
        println!("{:?}", result);
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let err_result = PgQueryError::from_original(result.error);
            bindings::pg_query_free_split_result(result);
            match err_result {
                Err(error) => Err(error),
                Ok(error) => Err(Failure::PgQueryError(error)),
            }
        } else {
            let n_stmts = result.n_stmts as usize;
            let mut y = Vec::<&str>::with_capacity(n_stmts);
            for offset in 0..n_stmts {
                let ptr = result.stmts.add(offset);
                let stmt = *ptr.read();
                let start = stmt.stmt_location as usize;
                let end = start + stmt.stmt_len as usize;
                let split_stmt = &query[start..end];
                y.push(split_stmt);
                // not sure the start..end slice'll hold up for non-utf8 charsets
            }
            bindings::pg_query_free_split_result(result);
            return Ok(y);
        }
    }
}

#[test]
fn test_splitting_with_scanner() {
    let actual = split_statements_with_scanner("select 1; select 2;");
    assert!(actual.is_ok());
    let expected = vec!["select 1", " select 2"];
    assert_eq!(actual.unwrap(), expected,);
}

pub fn split_statements_with_parser(query: &str) -> Result<Vec<&str>, Failure> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_split_with_parser(input.as_ptr());
        println!("{:?}", result);
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let err_result = PgQueryError::from_original(result.error);
            bindings::pg_query_free_split_result(result);
            match err_result {
                Err(error) => Err(error),
                Ok(error) => Err(Failure::PgQueryError(error)),
            }
        } else {
            let n_stmts = result.n_stmts as usize;
            let mut y = Vec::<&str>::with_capacity(n_stmts);
            for offset in 0..n_stmts {
                let ptr = result.stmts.add(offset);
                let stmt = *ptr.read();
                let start = stmt.stmt_location as usize;
                let end = start + stmt.stmt_len as usize;
                let split_stmt = &query[start..end];
                y.push(split_stmt);
                // not sure this'll ^^^^^^^^ hold up for non-utf8 charsets
            }
            bindings::pg_query_free_split_result(result);
            return Ok(y);
        }
    }
}

#[test]
fn test_splitting_with_parser() {
    let actual = split_statements_with_parser("select 1; select 2;");
    assert!(actual.is_ok());
    let expected = vec!["select 1", " select 2"];
    assert_eq!(actual.unwrap(), expected,);
}
// FingerprintResult
// pg_query_free_split_result
// pg_query_free_deparse_result
// pg_query_free_protobuf_parse_result
// pg_query_free_fingerprint_result
