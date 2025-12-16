use log::info;

mod user {
    use sea_orm::entity::prelude::*;

    #[sea_orm::model]
    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "user")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: Uuid,
        pub name: String,
    }

    impl ActiveModelBehavior for ActiveModel {}
}


#[tokio::main]
async fn main() -> Result<(), sea_orm::DbErr> {
    use uuid::Uuid;

    let env = env_logger::Env::default().filter_or("RUST_LOG", "info,sea_orm=debug,sqlx=warn");
    env_logger::Builder::from_env(env).init();

    use sea_orm::{entity::*, query::*};
    let db = &sea_orm::Database::connect("sqlite://db.sqlite?mode=rwc").await?;

    db.get_schema_registry("sea_orm_sqlite_uuid_issue::*")
        .sync(db)
        .await?;

    info!("Create user Bob with a profile:");
    user::ActiveModel::builder()
        .set_name("Bob")
        .set_id(Uuid::now_v7())
        .insert(db)
        .await?;

    Ok(())
}
