use std::fs::File;
use iron::prelude::*;
use iron::status;
use iron::headers;

use handler::*;

pub fn server_handler(req: &mut Request) -> IronResult<Response> {
    let query_path = get_query("query_path", req).unwrap_or_else(|| String::from("./"));

    println!("{}", query_path);
    let query_path = Path::new(&query_path);
    if !query_path.exists() {
        return Ok(Response::with(status::NotFound));
    }

    if query_path.is_dir() {
        let paths = fs::read_dir(query_path).unwrap();
        let mut html = String::from(r#"<html><head><meta charset="UTF-8"></head><body><ul>"#);

        for p in paths {
            html += &path_to_html(p.unwrap().path().as_path()).unwrap_or_default();
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
