use std::fs;
use std::path::Path;
use std::time::{UNIX_EPOCH, SystemTime};

use error::HandlerError;

pub mod server;

fn get_query(key: &str, req: &::iron::Request) -> Option<String> {
    let val = req.extensions
        .get::<::router::Router>()
        .unwrap()
        .find(key)
        .map(|i| {
            ::url::percent_encoding::percent_decode(i.as_bytes())
                .decode_utf8()
                .map(|s| s.to_string())
        });
    if val.is_some() {
        val.unwrap().ok()
    } else {
        None
    }
}

fn system_time_to_epoch(system_time: &SystemTime) -> u64 {
    system_time
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn path_to_file_name(path: &Path) -> Result<&str, HandlerError> {
    let file_name = try!(path.file_name().ok_or_else(|| {
        HandlerError::StringError("Path encoding error".to_owned())
    }));
    file_name.to_str().ok_or_else(|| {
        HandlerError::StringError("Path encoding error".to_owned())
    })
}

fn path_to_html(path: &Path) -> Result<String, HandlerError> {
    use url::percent_encoding::{utf8_percent_encode, PATH_SEGMENT_ENCODE_SET};
    let meta = try!(fs::metadata(path));
    if path.is_dir() {
        Ok(format!(
            r#"<li type="circle"><a href="/{}" bookdate="{}">{}</a></li>"#,
            utf8_percent_encode(
                try!(path.to_str().ok_or_else(|| {
                    HandlerError::StringError("Path encoding error".to_owned())
                })),
                PATH_SEGMENT_ENCODE_SET
            ).to_string(),
            try!(meta.modified().map(|ref i| system_time_to_epoch(i))),
            try!(path_to_file_name(path))
        ))
    } else {
        Ok(format!(
            r#"<li><a href="/{}" booktitle="{}" booksize="{}"
                bookdate="{}">{}</a></li>"#,
            utf8_percent_encode(
                try!(path.to_str().ok_or_else(|| {
                    HandlerError::StringError("Path encoding error".to_owned())
                })),
                PATH_SEGMENT_ENCODE_SET
            ).to_string(),
            try!(path_to_file_name(path)),
            meta.len(),
            try!(meta.modified().map(|ref i| system_time_to_epoch(i))),
            try!(path_to_file_name(path))
        ))
    }
}
