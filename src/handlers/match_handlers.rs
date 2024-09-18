use chrono::NaiveDateTime;
use salvo::prelude::*;
use uuid::Uuid;

use crate::get_db_pool;

#[handler]
pub async fn create_match(req: &mut Request, res: &mut Response) {
    let matchInfoReq: MatchInfoRequest = req.parse_json::<MatchInfoRequest>().await.unwrap();
    let uuid = Uuid::new_v4();
    let pool = get_db_pool();
    let result = sqlx::query!(
        "INSERT INTO rs_activity (id, name, cover, start_time, end_time, holding_date, location) VALUES (?, ?, ?, ?, ?, ?, ?)",
        uuid.to_string(),
        matchInfoReq.name,
        matchInfoReq.cover,
        matchInfoReq.start_time,
        matchInfoReq.end_time,
        matchInfoReq.holding_date,
        matchInfoReq.location
    )
    .execute(pool)
    .await;
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
