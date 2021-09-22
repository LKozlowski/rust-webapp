use warp::{Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let healthz_route = warp::path!("healthz").and_then(healtz_handler);
    let hello_world = warp::path::end().map(|| {
        format!(
            "Hello, World!\n\nCurrent Time: {}",
            chrono::offset::Utc::now()
        )
    });
    let routes = warp::get().and(hello_world.or(healthz_route));
    println!("started server at localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn healtz_handler() -> Result<impl Reply> {
    Ok("OK")
}
