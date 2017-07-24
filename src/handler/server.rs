use std::fs::File;
use iron::prelude::*;
use iron::status;
use iron::headers;

use handler::*;
use error::HandlerError;

pub fn server_handler(req: &mut Request) -> IronResult<Response> {
    let raw_query_path = format!(".{}", try!(get_path(req)));
    let query_path = Path::new(&raw_query_path);

    if query_path.is_dir() {
        let mut valid_paths = vec![];
        {
            use std::cmp;
            let paths = try!(fs::read_dir(query_path).map_err(HandlerError::from));
            for path in paths {
                if let Ok(p) = path {
                    valid_paths.push(p.path().as_path().to_owned());
                }
            }
            valid_paths.sort_by(|a, b| -> cmp::Ordering {
                if a.is_dir() ^ b.is_dir() {
                    if a.is_dir() {
                        cmp::Ordering::Less
                    } else {
                        cmp::Ordering::Greater
                    }
                } else {
                    a.cmp(b)
                }
            });
        }

        let mut html = format!(
            r#"<html><head>
            <meta charset="UTF-8"><title>{}</title>
            </head><body><h1>{}</h1><ul>"#,
            &raw_query_path[1..],
            &raw_query_path[1..]
        );
        for path in valid_paths {
            html += &path_to_html(&path).unwrap_or_default();
        }
        html += r#"</ul></body></html>"#;

        let mut response = Response::with((status::Ok, html));
        response
            .headers
            .set(headers::ContentType("text/html".parse().unwrap()));
        Ok(response)
    } else if query_path.is_file() {
        let file = match File::open(query_path) {
            Ok(f) => f,
            Err(_) => {
                return Ok(Response::with(status::Forbidden));
            }
        };
        let mut response = Response::with((status::Ok, file));
        response.headers.set(headers::ContentDisposition {
            disposition: headers::DispositionType::Attachment,
            parameters: vec![
                headers::DispositionParam::Filename(
                    headers::Charset::Iso_8859_1,
                    None,
                    try!(path_to_file_name(query_path)).to_owned().into_bytes(),
                ),
            ],
        });
        Ok(response)
    } else {
        Ok(Response::with(status::NotFound))
    }
}
