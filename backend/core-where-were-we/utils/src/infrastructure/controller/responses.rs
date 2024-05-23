pub struct OkHttpResponse<T> {
    status_code: usize,
    data: T,
}

pub struct ErrHttpResponse {
    status_code: usize,
    message: Option<String>
}

/// The status code enum
pub enum OkStatus {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
}

pub enum ErrorStatus {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    InternalServerError = 500
}

impl<T> OkHttpResponse<T> {
    pub fn new(status: OkStatus, value: T) -> Self {
        Self {
            status_code: status as usize,
            data: value
        }
    }
}

impl ErrHttpResponse {
    pub fn new(status: ErrorStatus, message: Option<&str>) -> Self {
        Self {
            status_code: status as usize,
            message: match message {
                Some(m) => Some(m.to_owned()),
                None => None
            }
        }
    }
}