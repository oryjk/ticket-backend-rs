use chrono::NaiveDateTime;
use salvo::{http::HeaderValue, prelude::*, Request, Response};
use sqlx::mysql::MySqlQueryResult;
use sqlx::Error;
use uuid::Uuid;

use crate::get_db_pool;

#[handler]
pub async fn create_match(req: &mut Request, res: &mut Response) {
    let match_info_req: MatchInfoRequest = req.parse_json::<MatchInfoRequest>().await.unwrap();
    save_to_db(match_info_req).await;
    res.headers_mut()
        .insert("Content-Type", HeaderValue::from_static("application/json"));
}

async fn save_to_db(match_info_req: MatchInfoRequest) -> Result<MySqlQueryResult, Error> {
    let uuid = Uuid::new_v4();
    let pool = get_db_pool();
    let result = sqlx::query!(
        "INSERT INTO rs_activity (id, name, cover, start_time, end_time, holding_date, location, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        uuid.to_string(),
        match_info_req.name,
        match_info_req.cover,
        match_info_req.start_time,
        match_info_req.end_time,
        match_info_req.holding_date,
        match_info_req.location,
        match_info_req.status
    )
        .execute(pool)
        .await;

    result
}

#[derive(serde::Deserialize)]
struct MatchInfoRequest {
    name: String,
    cover: String,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    holding_date: NaiveDateTime,
    location: String,
    status: i8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DB_POOL;

    #[tokio::test]
    async fn it_work() {
        let make_db_pool = crate::make_db_pool().await;
        DB_POOL.set(make_db_pool).ok();

        let result = save_to_db(MatchInfoRequest {
            name: "".to_string(),
            cover: "".to_string(),
            start_time: Default::default(),
            end_time: Default::default(),
            holding_date: Default::default(),
            location: "".to_string(),

            status: 10i8,
        }).await;
        match result {
            Ok(_) => {
                assert!(true)
            }
            Err(e) => {
                eprintln!("Error occurred: {:?}", e);
                assert!(false)
            }
        }
    }

    #[test]
    fn mode_test() {
        let x = Some(10);
        let y: Option<i8> = None;

        if let Some(1000) = x {
            println!("Yes!!!Input is 1000")
        } else {
            println!("No!!!Input is {:?}", x)
        }
        ////////////////////////////////////
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        stack.push(1);
        while let Some(num) = stack.pop() {
            println!("{}", num)
        }
    }


    trait Draw {
        fn draw(&self);
    }
    struct Circle;
    impl Draw for Circle {
        fn draw(&self) {
            println!("I am Circle")
        }
    }
    struct Square;
    impl Draw for Square {
        fn draw(&self) {
            println!("I am Square")
        }
    }

    fn draw_shape(shape: &dyn Draw) {
        shape.draw()
    }

    #[test]
    fn draw_shape_test(){
        let circle = Circle;
        let square = Square;
        draw_shape(&circle);
        draw_shape(&square);
    }
}