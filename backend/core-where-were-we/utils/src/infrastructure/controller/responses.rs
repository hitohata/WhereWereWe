
struct HttpResponse<T> {
    status_code: usize,
    data: T,
}

/// The status code enum
pub enum StatusCode {
    OK,
    Created,
    Accepted,
    NoContent,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Unexpected
}

/// mapper
fn to_code(status_code: StatusCode) -> usize {
    match status_code {
        StatusCode::OK => 200,
        StatusCode::Created => 201,
        StatusCode::Accepted => 202,
        StatusCode::NoContent => 204,
        StatusCode::BadRequest => 400,
        StatusCode::Unauthorized => 401,
        StatusCode::Forbidden => 403,
        StatusCode::NotFound => 404,
        StatusCode::Unexpected => 500
    }
}

pub fn response<T>(status_code: StatusCode, value: T) -> HttpResponse<T> {
    HttpResponse {
        status_code: to_code(status_code),
        data: value
    }
}

