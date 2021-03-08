use lambda_http::{lambda, IntoResponse, Request, RequestExt};
use lambda_runtime::{Context, error::HandlerError};
use serde_json;
use serde_derive::{Serialize, Deserialize};


#[derive(Deserialize, Serialize, Default)]
struct UserState {
    #[serde(default)]
    a: f32,
    #[serde(default)]
    b: f32,
}

#[derive(Deserialize, Serialize)]
enum InternalCode {
    Ok,
    AwsError,
}

#[derive(Deserialize, Serialize)]
struct ApiResponse {
    internal_code: InternalCode,
    user_state: UserState
}

fn main() {
    lambda!(update_state)
}


fn update_state(
    request: Request,
    _ctx: Context
) -> Result<impl IntoResponse, HandlerError> {
    let mut ud: UserState = request.payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    println!("{{a: {}, b: {}}}", ud.a, ud.b);

    ud.a = ud.a + 1.7;
    ud.b = 2.0 * 3.141592653589793 * ud.b;

    println!("{{a: {}, b: {}}}", ud.a, ud.b);

    let response = ApiResponse {
        internal_code: InternalCode::Ok,
        user_state: ud,
    };

    Ok(serde_json::to_string(&response).unwrap())
}
