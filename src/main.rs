use lambda_runtime::{handler_fn, Context, Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    pub name: String,
}

#[derive(Serialize)]
struct Response {
    pub message: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(my_handler);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn my_handler(event: Request, _: Context) -> Result<Response, Error> {
    let name = event.name;
    let message = format!("Hello, {}!", name);
    Ok(Response { message })
}