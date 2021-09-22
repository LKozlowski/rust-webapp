use warp::{Reply, Rejection, Filter};


type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let healthz_route = warp::path!("healthz").and_then(healtz_handler);
    let routes = healthz_route.with(warp::cors().allow_any_origin());

    println!("started server at localhost:8000");
    warp::serve(routes).run(([0,0,0,0], 8000)).await;
}


async fn healtz_handler() -> Result<impl Reply> {
    Ok("OK")
}

