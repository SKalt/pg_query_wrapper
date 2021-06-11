use std::ffi::{CStr, CString, IntoStringError, NulError};

mod bindings;
pub struct PgQueryError {
    pub message: String,
    pub funcname: String,
    pub filename: String,
    pub lineno: i32,
    pub cursorpos: i32,
    pub context: String,
}

impl PgQueryError {
    fn from_original(original: *mut bindings::PgQueryError) -> Result<Self, MyError> {
        if original != std::ptr::null_mut::<bindings::PgQueryError>() {
            let message: String;
            let funcname: String;
            let filename: String;
            let context: String;
            let lineno: i32;
            let cursorpos: i32;
            unsafe {
                let orig = *original;
                message = CString::from_raw(orig.message).into_string()?;
                funcname = CString::from_raw(orig.funcname).into_string()?;
                filename = CString::from_raw(orig.filename).into_string()?;
                context = CString::from_raw(orig.context).into_string()?;
                lineno = orig.lineno;
                cursorpos = orig.cursorpos;
            }
            return Ok(PgQueryError{
                message,
                funcname,
                filename,
                lineno,
                cursorpos,
                context
            })
        } else {
            return Err(MyError::NullPtr)
        }
    }
}

pub enum MyError {
    PgQueryError(PgQueryError),
    NulError(NulError),
    IntoStringError(std::ffi::IntoStringError),
    NullPtr,
}
impl From<NulError> for MyError {
    fn from(e: NulError) -> Self {
        Self::NulError(e)
    }
}
impl From<IntoStringError> for MyError {
    fn from(e: IntoStringError) -> Self {
        Self::IntoStringError(e)
    }
}
impl From<PgQueryError> for MyError {
    fn from(e: PgQueryError) -> Self {
        Self::PgQueryError(e)
    }
}


struct ScanResult {
    // pub pbuf: PgQueryProtobuf,
    // pub stderr_buffer: *mut ::std::os::raw::c_char,
    // pub error: *mut PgQueryError,

}
// pub struct PgQueryProtobuf {
//     pub len: ::std::os::raw::c_uint,
//     pub data: *mut ::std::os::raw::c_char,
// }

pub fn normalize_query(query: &str) -> Result<String, MyError> {
    let input = CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_normalize(input.as_ptr());
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let error = PgQueryError::from_original(result.error)?;
            bindings::pg_query_free_normalize_result(result);
            return Err(MyError::PgQueryError(error))
        } else {
            let normalized_query = CString::from_raw(result.normalized_query).into_string()?;
            bindings::pg_query_free_normalize_result(result);
            return Ok(normalized_query)
        }
    }
}


pub fn scan(query: &str) { //  -> Result<_, MyError>
    let input: CString::new(query)?;
    unsafe {
        let result = bindings::pg_query_scan(input);
        if result.error != std::ptr::null_mut::<bindings::PgQueryError>() {
            let error = PgQueryError::from_original(result.error)?;
            bindings::pg_query_free_scan_result(result);
            return Err(error)
        } else {
            
            result.pbuf.data;
            result.pbuf.len
        }
    }
}


// FingerprintResult
// pg_query_free_scan_result
// pg_query_free_parse_result
// pg_query_free_split_result
// pg_query_free_deparse_result
// pg_query_free_protobuf_parse_result
// pg_query_free_plpgsql_parse_result
// pg_query_free_fingerprint_result
fn main() {
    println!("Hello, world!");
}
