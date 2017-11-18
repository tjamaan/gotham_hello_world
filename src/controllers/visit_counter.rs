use mime;
use hyper::StatusCode;
use hyper::server::{Request, Response};
use gotham::http::response::create_response;
use gotham::state::{State, FromState};
use gotham::middleware::session::SessionData;
use session::MySession;

fn visit_counter_html(counter: u32) -> String {
    format!(r#"<!doctype html>
        <html>
        <head>
        <title>Visit counter</title>
        </head>
        <body>
        <h1>
        You visited this page {} times!
        </h1>
        </body>
        </html>"#,
        counter
    )
}

pub fn visit_counter(mut state: State, request: Request) -> (State, Response) {
    let counter;
    {
        let session = SessionData::<MySession>::borrow_mut_from(&mut state);
        counter = session.counter;
        session.counter += 1;
    }

    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((visit_counter_html(counter).into(), mime::TEXT_HTML_UTF_8))
    );

    (state, response)
}