#[doc = include_str!("../README.md")]

const API_URL: &str = "https://api.moabdb.com/request/v1/";

use polars::prelude::DataFrame;

pub mod credentials;
pub mod errors;
mod protocol;
pub mod window;

/// Get the equity data for a given ticker
/// # Arguments
/// * `ticker` - The ticker symbol of the equity
/// * `window` - The window of time to get data for. Build a window with the `WindowBuilder`
/// * `intraday` - Whether to get intraday data or daily data
/// * `credentials` - The credentials to use to authenticate the request. If None, the request will be unauthenticated
///
/// # Returns
/// A `DataFrame` containing the equity data
///
/// # Examples
/// ```rust
/// use moabdb::{get_equity, window::WindowBuilder, window::WindowLength, credentials::Credentials};
///
/// let window = WindowBuilder::new()
///     .length(WindowLength::Months(3))
///     .build()
///     .unwrap();
///
/// let df = get_equity("AAPL", window, false, None).unwrap();
/// println!("{:?}", df);
/// ```
///
pub fn get_equity(
    ticker: impl AsRef<str>,
    window: window::Window,
    intraday: bool,
    credentials: Option<credentials::Credentials>,
) -> Result<DataFrame, errors::MoabError> {
    use polars::prelude::{ParquetReader, SerReader};

    let datatype = if intraday {
        "intraday_stocks"
    } else {
        "daily_stocks"
    };
    let req = protocol::Request {
        symbol: ticker.as_ref().to_string(),
        start: window.start.timestamp() as u32,
        end: window.end.timestamp() as u32,
        datatype: datatype.to_string(),
        username: match credentials {
            Some(ref creds) => creds.username.to_string(),
            None => "".to_string(),
        },
        token: match credentials {
            Some(ref creds) => creds.token.to_string(),
            None => "".to_string(),
        },
    };
    let req = req.b64();

    let mut resp = match ureq::get(API_URL).set("x-req", &req).call() {
        Ok(resp) => resp.into_reader(),
        Err(_) => return Err(errors::MoabError::TransportError),
    };

    // Read the resp to end
    let mut buffer = Vec::new();
    match resp.read_to_end(&mut buffer) {
        Ok(_) => (),
        Err(_) => return Err(errors::MoabError::TransportError),
    }
    let resp = match String::from_utf8(buffer) {
        Ok(resp) => resp,
        Err(_) => return Err(errors::MoabError::TransportError),
    };

    let resp: protocol::Response = match resp.try_into() {
        Ok(resp) => resp,
        Err(_) => return Err(errors::MoabError::TransportError),
    };

    match resp.code {
        200 => (),
        400 => return Err(errors::MoabError::RequestError),
        401 => return Err(errors::MoabError::Unauthorized),
        404 => return Err(errors::MoabError::NotFound),
        500 => return Err(errors::MoabError::ServerInternalError),
        _ => return Err(errors::MoabError::UnknownError),
    }
    let df = ParquetReader::new(std::io::Cursor::new(resp.data));
    let df = match df.finish() {
        Ok(df) => df,
        Err(_) => return Err(errors::MoabError::TransportError),
    };

    Ok(df)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equity_sync() {
        let window = window::WindowBuilder::new()
            .length(window::WindowLength::Years(3))
            .build()
            .unwrap();
        let df = get_equity("AAPL", window, false, None).unwrap();
        println!("{:?}", df);
    }
}
