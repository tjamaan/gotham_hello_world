use mime;
use hyper;
use hyper::StatusCode;
use hyper::server::{Request, Response};
use gotham;
use gotham::http::response::create_response;
use gotham::state::State;
use gotham::state::{StateData, FromState};

// the struct representing our extracted path. we derive some standard traits needed by gotham.
#[derive(StateData, FromState, PathExtractor, StaticResponseExtender)]
pub struct CubePathExtractor {
    pub number: f64,
}

// return the html of the page given an answer
fn cube_html(number_string: String, answer_string: String) -> String {
    format!(
        r#"<!doctype html>
        <html>
        <head>
        <title>Cube</title>
        </head>
        <body>
        <h1>
        Use "/cube/your number" to cube your number.
        </h1>
        <div>
        The cube of {} is {}
        </div>
        </body>
        </html>"#,
        number_string,
        answer_string
    )
}

pub fn cube(state: State, request: Request) -> (State, Response) {
    let number;
    let cubed;

    // here we are declaring a new scope because we only want to borrow state for a little bit and not for the whole function
    {
        // path is declared in this scope and will not exist after it
        let path = CubePathExtractor::borrow_from(&state);
        // get the number and its cube. f64 is a primitive type.
        // Here it's getting copied
        number = path.number;
        cubed = number * number * number;
    }

    let response = create_response(
        &state,
        StatusCode::Ok,
        Some(
            (cube_html(format!("{}", number), format!("{}", cubed)).into(),
             mime::TEXT_HTML_UTF_8)
        )
    );

    (state, response)
}