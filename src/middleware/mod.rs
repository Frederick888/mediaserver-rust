use iron::prelude::*;
use iron::status;
use iron::headers;
use iron::{BeforeMiddleware, AfterMiddleware};
use error::HandlerError;

fn unauthorized_response() -> Response {
    let mut response = Response::with(status::Unauthorized);
    response
        .headers
        .set_raw("WWW-Authenticate", vec![b"Basic realm=\"main\"".to_vec()]);
    response
}

pub struct BasicAuth;

impl BeforeMiddleware for BasicAuth {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        fn auth(username: &str, password: &str) -> bool {
            username == ::get_args().value_of("username").unwrap_or_else(|| "user") &&
                password ==
                    ::get_args()
                        .value_of("password")
                        .unwrap_or_else(|| "comicglass")
        }

        match req.headers.get::<headers::Authorization<headers::Basic>>() {
            Some(
                &headers::Authorization(headers::Basic {
                    ref username,
                    password: Some(ref password),
                }),
            ) => {
                if auth(username, password) {
                    Ok(())
                } else {
                    Err(IronError {
                        error: Box::new(HandlerError::AuthError(
                            Some(username.to_owned()),
                            Some(password.to_owned()),
                        )),
                        response: unauthorized_response(),
                    })
                }
            }
            Some(
                &headers::Authorization(headers::Basic {
                    ref username,
                    password: None,
                }),
            ) => {
                Err(IronError {
                    error: Box::new(HandlerError::AuthError(Some(username.to_owned()), None)),
                    response: unauthorized_response(),
                })
            }
            None => {
                Err(IronError {
                    error: Box::new(HandlerError::AuthError(None, None)),
                    response: unauthorized_response(),
                })
            }
        }
    }
}

pub struct LogAfterRequest;

impl AfterMiddleware for LogAfterRequest {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let root = ::get_logger();
        info!(root, "LogAfterRequest";
            "status" => res.status.unwrap_or_else(|| status::Unregistered(0u16)).to_string(),
            "url" => req.url.to_string()
        );
        Ok(res)
    }

    fn catch(&self, req: &mut Request, err: IronError) -> IronResult<Response> {
        let root = ::get_logger();
        {
            let res = &err.response;
            error!(root, "LogAfterRequest";
                "status" => res.status.unwrap_or_else(|| status::Unregistered(0u16)).to_string(),
                "url" => req.url.to_string(),
                "message" => err.to_string()
            );
        }
        Ok(err.response)
    }
}
