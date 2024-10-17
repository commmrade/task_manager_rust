
use sqlx::{Error, MySql};


struct Db {
    pool : sqlx::Pool<MySql>
}

impl Db
{
    async fn new(url : String) -> Result<Self, Error> {
        let pool = sqlx::MySqlPool::connect(&url).await?;

        Ok(Self { pool })
    }
}