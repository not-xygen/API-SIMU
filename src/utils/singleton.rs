use sqlx::mysql::{MySqlPoolOptions, MySqlPool};

// lazy_static! {
//     static ref DATABASE_POOL: Mutex<Option<MySqlPool>> = Mutex::new(None);
//     static ref INIT: Once = Once::new();
// }

// pub async fn get_database_pool() -> &'static Mutex<Option<MySqlPool>> {
//     INIT.call_once(|| {
//         let _ = dotenv::dotenv();
//     });

//     let mut pool = DATABASE_POOL.lock().unwrap();

//     if pool.is_none() {
//         let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         let new_pool = match MySqlPool::connect(&database_url).await {
//             Ok(pool) => {
//                 println!("✅ Connection to the database is successful!");
//                 pool
//             }
//             Err(err) => {
//                 println!("❌ Failed to connect to the database: {:?}", err);
//                 std::process::exit(1);
//             }
//         };
//         *pool = Some(new_pool);
//     }

//     &*DATABASE_POOL
// }

pub async fn get_database_pool() -> Result<MySqlPool, sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    Ok(pool)
}


// test
// #[test]
// fn test_get_database_pool_test() {
//     get_database_pool();
// }
