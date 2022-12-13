// Jackson Coxson

use chrono::NaiveDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub start: NaiveDateTime,
    pub end: NaiveDateTime,
}

/// WindowBuilder is an abstraction for creating a Window.
/// You can build a window by specifying the start and end times,
/// or by specifying a length and either the start or end time.
/// If a length is specified but not a start or end time, the current
/// time will be used as the unspecified time.
/// ## Examples
///
/// ### Specify the start and length
/// ```
/// use moabdb::window::*;
/// use chrono::NaiveDateTime;
/// let window = WindowBuilder::new()
///     .start(NaiveDateTime::from_timestamp_opt(0, 0).unwrap())
///     .length(WindowLength::Days(1))
///     .build()
///     .unwrap();
/// assert_eq!(window.start, NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
/// assert_eq!(window.end, NaiveDateTime::from_timestamp_opt(86400, 0).unwrap());
/// ```
///
/// ### Specify the end and length
/// ```
/// use moabdb::window::*;
/// use chrono::NaiveDateTime;
/// let window = WindowBuilder::new()
///     .end(NaiveDateTime::from_timestamp_opt(86400, 0).unwrap())
///     .length(WindowLength::Days(1))
///     .build()
///     .unwrap();
/// assert_eq!(window.start, NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
/// assert_eq!(window.end, NaiveDateTime::from_timestamp_opt(86400, 0).unwrap());
/// ```
///
/// ### Specify the length only
/// ```
/// use moabdb::window::*;
/// use chrono::NaiveDateTime;
/// let window = WindowBuilder::new()
///     .length(WindowLength::Days(1))
///     .build()
///     .unwrap();
/// assert_eq!(window.start, chrono::Utc::now().naive_utc() - chrono::Duration::days(1));
/// assert_eq!(window.end, chrono::Utc::now().naive_utc());
/// ```
///
/// ### Specify the start and end
/// ```
/// use moabdb::window::*;
/// use chrono::NaiveDateTime;
/// let window = WindowBuilder::new()
///     .start(NaiveDateTime::from_timestamp_opt(0, 0).unwrap())
///     .end(NaiveDateTime::from_timestamp_opt(86400, 0).unwrap())
///     .build()
///     .unwrap();
/// assert_eq!(window.start, NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
/// assert_eq!(window.end, NaiveDateTime::from_timestamp_opt(86400, 0).unwrap());
/// ```
pub struct WindowBuilder {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub length: Option<WindowLength>,
}

pub enum WindowLength {
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
    Weeks(i64),
    Months(i64),
    Years(i64),
}

impl WindowBuilder {
    /// Create a new WindowBuilder
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
            length: None,
        }
    }
    /// Set the start time of the request window
    pub fn start(mut self, start: NaiveDateTime) -> Self {
        self.start = Some(start);
        self
    }
    /// Set the end time of the request window
    pub fn end(mut self, end: NaiveDateTime) -> Self {
        self.end = Some(end);
        self
    }
    /// Set the length of the request window
    pub fn length(mut self, length: WindowLength) -> Self {
        self.length = Some(length);
        self
    }
    /// Build the window
    pub fn build(self) -> Result<Window, String> {
        if self.start.is_some() && self.end.is_some() {
            if self.start.unwrap() > self.end.unwrap() {
                return Err("Start time must be before end time".to_string());
            }
            return Ok(Window {
                start: self.start.unwrap(),
                end: self.end.unwrap(),
            });
        }
        if self.start.is_some() && self.length.is_some() {
            let start = self.start.unwrap();
            let length = self.length.unwrap();
            let end = match length {
                WindowLength::Seconds(s) => start + chrono::Duration::seconds(s),
                WindowLength::Minutes(m) => start + chrono::Duration::minutes(m),
                WindowLength::Hours(h) => start + chrono::Duration::hours(h),
                WindowLength::Days(d) => start + chrono::Duration::days(d),
                WindowLength::Weeks(w) => start + chrono::Duration::weeks(w),
                WindowLength::Months(m) => start + chrono::Duration::days(m * 30),
                WindowLength::Years(y) => start + chrono::Duration::days(y * 365),
            };
            return Ok(Window { start, end });
        }
        if self.end.is_some() && self.length.is_some() {
            let end = self.end.unwrap();
            let length = self.length.unwrap();
            let start = match length {
                WindowLength::Seconds(s) => end - chrono::Duration::seconds(s),
                WindowLength::Minutes(m) => end - chrono::Duration::minutes(m),
                WindowLength::Hours(h) => end - chrono::Duration::hours(h),
                WindowLength::Days(d) => end - chrono::Duration::days(d),
                WindowLength::Weeks(w) => end - chrono::Duration::weeks(w),
                WindowLength::Months(m) => end - chrono::Duration::days(m * 30),
                WindowLength::Years(y) => end - chrono::Duration::days(y * 365),
            };
            return Ok(Window { start, end });
        }
        if self.length.is_some() {
            // Get the current time
            let now = chrono::Local::now().naive_local();
            let length = self.length.unwrap();
            let start = match length {
                WindowLength::Seconds(s) => now - chrono::Duration::seconds(s),
                WindowLength::Minutes(m) => now - chrono::Duration::minutes(m),
                WindowLength::Hours(h) => now - chrono::Duration::hours(h),
                WindowLength::Days(d) => now - chrono::Duration::days(d),
                WindowLength::Weeks(w) => now - chrono::Duration::weeks(w),
                WindowLength::Months(m) => now - chrono::Duration::days(m * 30),
                WindowLength::Years(y) => now - chrono::Duration::days(y * 365),
            };
            return Ok(Window { start, end: now });
        }

        Err("Must provide either start and end or start and length".to_string())
    }
}

impl Default for WindowBuilder {
    fn default() -> Self {
        Self::new()
    }
}
