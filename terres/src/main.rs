use futures::executor::block_on;
use sea_orm::*;
use sea_orm_migration::SchemaManager;

mod entities;
use entities::{prelude::*, *};

const DATABASE_URL: &str = "postgresql://postgres:superSecurePassword2209@localhost:5432";
const DB_NAME: &str = "new-holland";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = &match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", DB_NAME),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            ))
            .await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(())
}


async fn run_1() -> Result<(), DbErr> {
    let db = Database::connect(format!("{}/{}", DATABASE_URL, DB_NAME)).await?;

    let schema_manager = SchemaManager::new(&db);
    assert!(schema_manager.has_table("bakery").await?);
    assert!(schema_manager.has_table("chef").await?);

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(&db).await?;

    let happy_bakery: Option<bakery::Model> = Bakery::find_by_id(1).one(&db).await?;
    assert_eq!(happy_bakery.unwrap().name, "Happy Bakery");

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run_1()) {
        panic!("{}", err);
    }
    println!("Hello, world!");
}
