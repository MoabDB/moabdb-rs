// Jackson Coxson

#[derive(Debug)]
pub enum MoabError {
    ServerInternalError,
    ServerTimeoutError,
    DecodeError,
    RequestError,
    TransportError,
    NotFound,
    Unauthorized,
    UnknownError,
}
