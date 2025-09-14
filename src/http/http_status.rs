#[derive(Clone)]
pub enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}
