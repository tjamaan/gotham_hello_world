use mime;
use hyper::StatusCode;
use hyper::server::{Request, Response};
use gotham::http::response::create_response;
use gotham::state::State;

static WELCOME_HTML: &str = r#"<!doctype html>
<html>
<head>
<title>Welcome to my site!</title>
</head>
<body>
<h1>
Welcome to my site 😁
</h1>
</body>
</html>"#;

pub fn welcome(state: State, request: Request) -> (State, Response) {
    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((WELCOME_HTML.into(), mime::TEXT_HTML_UTF_8))
    );

    (state, response)
}