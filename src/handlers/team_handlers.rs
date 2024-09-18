use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use once_cell::sync::OnceCell;
use salvo::prelude::*;
use sqlx::{types::time::PrimitiveDateTime, MySqlPool};

use crate::get_db_pool;

extern crate once_cell;

#[handler]
pub async fn get_user_team(req: &mut Request, res: &mut Response) {
    let user_id: &str = req.param::<&str>("id").unwrap();

    let pool = get_db_pool();

    let rows = sqlx::query!(
        "SELECT id as 'id!', user_id as 'user_id!', team_id as 'team_id!', join_time as 'join_time!' FROM rs_user_team WHERE user_id = ?",
        user_id
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        tracing::error!("Query failed: {:?}", e);
        salvo::Error::other(e)
    }).expect("hhhhh");

    let user_teams: Vec<UserTeam> = rows
        .into_iter()
        .map(|row| UserTeam {
            id: row.id,
            user_id: row.user_id,
            team_id: row.team_id,
            join_time: NaiveDateTime::from(CustomDateTime::from(row.join_time)),
        })
        .collect();

    res.render(Json(user_teams));
}

#[derive(serde::Serialize)]
struct UserTeam {
    id: i64,
    user_id: String,
    team_id: String,
    join_time: NaiveDateTime,
}
struct CustomDateTime(PrimitiveDateTime);

impl From<PrimitiveDateTime> for CustomDateTime {
    fn from(pdt: PrimitiveDateTime) -> Self {
        CustomDateTime(pdt)
    }
}

impl From<CustomDateTime> for NaiveDateTime {
    fn from(cdt: CustomDateTime) -> Self {
        NaiveDateTime::new(
            NaiveDate::from_ymd_opt(
                cdt.0.year(),
                cdt.0.date().month() as u32,
                cdt.0.day() as u32,
            )
            .unwrap(),
            NaiveTime::from_hms_opt(
                cdt.0.hour() as u32,
                cdt.0.minute() as u32,
                cdt.0.second() as u32,
            )
            .unwrap(),
        )
    }
}
