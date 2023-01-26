use sea_orm::{Database, DbErr};
use futures::executor::block_on;

const DATABASE_URL: &str = "postgresql://postgres:superSecurePassword2209@localhost:5432/postgres";
const DB_NAME: &str = "postgres";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
    println!("Hello, world!");
}
