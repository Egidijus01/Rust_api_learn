// use model::{Store, Item};
// use warp::{Filter, filters::body::json};
// mod model;


// fn json_body() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
//     warp::body::content_length_limit(1024*16).and(warp::body::json())
// }


// #[tokio::main]
// async fn main() {
//     let store = Store::new();
//     let store_filter = warp::any().map(move || store.clone());

//     let add_items = warp::post()
//         .and(warp::path("v1"))
//         .and(warp::path("groceries"))
//         .and(warp::path::end())
//         .and(json_body())
//         .and(store_filter.clone())
//         .and_then(add_grocery_list_item);

//     let get_items = warp::get()
//     .and(warp::path("v1"))
//     .and(warp::path("groceries"))
//     .and(warp::path::end())
//     .and(store_filter.clone())
//     .and_then(get_grocery_list);

//     let routes = add_items.or(get_items);



//     warp::serve(routes)
//         .run(([127, 0, 0, 1], 3030))
//         .await;
// }

// async fn add_grocery_list_item(
//     item: Item,
//     store: Store
// ) -> Result<impl warp::Reply, warp::Rejection> {
//     let r = store.grocery_list.read();
//     Ok(warp::reply::json(&*r))
// }

// async fn get_grocery_list(
//     store: Store
//     ) -> Result<impl warp::Reply, warp::Rejection>{
//     let result = store.grocery_list.read();
//     Ok(warp::reply::json(&*result))
// }




use serde::Serialize;
use warp::{reply::json, Filter, Rejection, Reply};

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}



#[tokio::main]
async fn main() {
    

    let health_checker = warp::path!("api" / "healthchecker")
        .and(warp::get())
        .and_then(health_checker_handler);

    let routes = health_checker.with(warp::log("api"));

    println!("🚀 Server started successfully");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}