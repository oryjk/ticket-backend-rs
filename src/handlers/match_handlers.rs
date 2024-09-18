use chrono::NaiveDateTime;
use once_cell::sync::OnceCell;
use salvo::prelude::*;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::get_db_pool;

#[handler]
pub async fn create_match(req: &mut Request, res: &mut Response) {
    let matchInfoReq: MatchInfoRequest = req.parse_json::<MatchInfoRequest>().await.unwrap();
    let uuid = Uuid::new_v4();
    let pool = get_db_pool();
}

#[derive(serde::Deserialize)]
struct MatchInfoRequest {
    name: String,
    cover: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    holding_date: NaiveDateTime,
    location: String,
}
