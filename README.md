Sea-ORM is saving UUIDs as binary blobs in sqlite, even though the [documentation]
*seems* to suggest that it would save as text.

[documentation]: https://www.sea-ql.org/SeaORM/docs/generate-entity/column-types/#type-mappings


```rust
#[sea_orm::model]
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
}
```

```bash
$ ./run.sh
> cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/sea-orm-sqlite-uuid-issue`
[2025-12-17T00:06:46Z DEBUG sea_orm::driver::sqlx_sqlite] SELECT sqlite_version()
[2025-12-17T00:06:46Z INFO  sea_orm_sqlite_uuid_issue] Syncing schema
[2025-12-17T00:06:46Z DEBUG sea_orm::driver::sqlx_sqlite] SELECT "name" FROM "sqlite_master" WHERE "type" = 'table' AND "name" <> 'sqlite_sequence'
[2025-12-17T00:06:46Z DEBUG sea_orm::driver::sqlx_sqlite] SELECT "name" FROM "sqlite_master" WHERE "type" = 'table' AND "name" <> 'sqlite_sequence'
[2025-12-17T00:06:46Z DEBUG sea_orm::driver::sqlx_sqlite] CREATE TABLE "user" ( "id" uuid_text NOT NULL PRIMARY KEY, "name" varchar NOT NULL )
[2025-12-17T00:06:46Z INFO  sea_orm_sqlite_uuid_issue] Create user Bob with a profile
[2025-12-17T00:06:46Z DEBUG sea_orm::database::transaction] INSERT INTO "user" ("id", "name") VALUES ('019b29a1-786b-7b51-8132-9b425900e526', 'Bob') RETURNING "id", "name"

> sqlite3 db.sqlite .dump
PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE IF NOT EXISTS "user" ( "id" uuid_text NOT NULL PRIMARY KEY, "name" varchar NOT NULL );
INSERT INTO user VALUES(X'019b29a1786b7b5181329b425900e526','Bob');
COMMIT;
```
