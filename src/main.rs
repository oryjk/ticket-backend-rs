use once_cell::sync::OnceCell;
use salvo::prelude::*;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

mod handlers;

pub static DB_POOL: OnceCell<MySqlPool> = OnceCell::new();

pub fn get_db_pool() -> &'static MySqlPool {
    DB_POOL.get().unwrap()
}

async fn make_db_pool() -> MySqlPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect("mysql://root:beifa888@www.oryjk.cn:3306/registration_system")
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    // 初始化日志记录
    tracing_subscriber::fmt().init();

    let make_db_pool = make_db_pool().await;
    DB_POOL.set(make_db_pool).ok();

    let router =
        Router::new()
            .push(Router::with_path("/rs/team/<id>").get(handlers::team_handlers::get_user_team))
            .push(Router::new().path("/rs/admin").push(
                Router::with_path("create_match").post(handlers::match_handlers::create_match),
            ));
    let acceptor = TcpListener::new("127.0.0.1:7878").bind().await;
    Server::new(acceptor).serve(router).await;
}
