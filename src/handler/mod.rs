use std::fs;
use std::path::Path;
use std::time::{UNIX_EPOCH, SystemTime};

use error::HandlerError;

pub mod server;

fn get_path(req: &::iron::Request) -> Result<String, HandlerError> {
    let mut path = String::new();
    for seg in req.url.path() {
        path += "/";
        path += seg;
    }
    ::url::percent_encoding::percent_decode(path.as_bytes())
        .decode_utf8()
        .map(|s| s.to_string())
        .map_err(|_| {
            HandlerError::StringError("Failed to parse path".to_owned())
        })
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
    use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};
    let meta = try!(fs::metadata(path));
    let encoded_path = utf8_percent_encode(
        try!(path.to_str().map(|i| { &i[1..] }).ok_or_else(|| {
            HandlerError::StringError("Path encoding error".to_owned())
        })),
        DEFAULT_ENCODE_SET,
    ).to_string();
    if path.is_dir() {
        Ok(format!(
            r#"<li type="circle"><a href="{}" bookdate="{}">{}</a></li>"#,
            encoded_path,
            try!(meta.modified().map(|ref i| system_time_to_epoch(i))),
            try!(path_to_file_name(path))
        ))
    } else {
        Ok(format!(
            r#"<li><a href="{}" booktitle="{}" booksize="{}"
                bookdate="{}">{}</a></li>"#,
            encoded_path,
            try!(path_to_file_name(path)),
            meta.len(),
            try!(meta.modified().map(|ref i| system_time_to_epoch(i))),
            try!(path_to_file_name(path))
        ))
    }
}
