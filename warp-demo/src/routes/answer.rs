use warp::http::StatusCode;
use warp::Rejection;
use warp::Reply;

use crate::store::Store;
use crate::types::answer::NewAnswer;

pub async fn add_answer(store: Store, new_answer: NewAnswer) -> Result<impl Reply, Rejection> {
    match store.add_answer(new_answer).await {
        Ok(_) => Ok(warp::reply::with_status("Answer added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
