use mime;
use hyper;
use hyper::StatusCode;
use hyper::server::{Request, Response};
use gotham;
use gotham::http::response::create_response;
use gotham::state::State;
use gotham::state::{StateData, FromState};

// the struct representing our query string. we derive some standard traits needed by gotham.
#[derive(StateData, FromState, QueryStringExtractor, StaticResponseExtender)]
pub struct CapitalizeQueryStringExtractor {
    pub text: String,
}

// return the html of the page given an answer
fn capitalize_html(capitalized_text: String) -> String {
    format!(
        r#"<!doctype html>
        <html>
        <head>
        <title>Capitalize</title>
        </head>
        <body>
        <h1>
        Use <pre>/capitalize?text=your text</pre> to capitalize your text.
        </h1>
        <div>
        {}
        </div>
        </body>
        </html>"#,
        capitalized_text
    )
}

pub fn capitalize(state: State, request: Request) -> (State, Response) {
    let uppercase_text;

    // here we are declaring a new scope because we only want to borrow state for a little bit and not for the whole function
    {
        // query_string is declared in this scope and will not exist after it
        let query_string = CapitalizeQueryStringExtractor::borrow_from(&state);
        // get the capitalized text from the query string text. to_uppercase() creates a copy of the text so we don't need
        // to be borrowing the state after this point
        uppercase_text = query_string.text.to_uppercase();
    }

    let response = create_response(
        &state,
        StatusCode::Ok,
        Some((capitalize_html(uppercase_text).into(), mime::TEXT_HTML_UTF_8))
    );

    (state, response)
}