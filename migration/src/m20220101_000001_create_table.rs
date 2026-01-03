use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = sea_orm::Statement::from_string(
            manager.get_database_backend(),
            "CREATE EXTENSION IF NOT EXISTS citext".to_string(),
        );
        manager.get_connection().execute(stmt).await?;

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Id)
                            .uuid()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(
                        ColumnDef::new(Users::Email)
                            .custom(Citext::Citext)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Users::Password).string().not_null())
                    .col(
                        ColumnDef::new(Users::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("NOW()")),
                    )
                    .col(
                        ColumnDef::new(Users::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::cust("NOW()")),
                    )
                    .to_owned(),
            )
            .await?;

        // Create trigger function to update updated_at timestamp
        let trigger_fn_stmt = sea_orm::Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE OR REPLACE FUNCTION update_updated_at_column()
            RETURNS TRIGGER AS $$
            BEGIN
                NEW.updated_at = NOW();
                RETURN NEW;
            END;
            $$ language 'plpgsql';
            "#
            .to_string(),
        );
        manager.get_connection().execute(trigger_fn_stmt).await?;

        // Create trigger on users table
        let trigger_stmt = sea_orm::Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TRIGGER update_users_updated_at
            BEFORE UPDATE ON users
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at_column();
            "#
            .to_string(),
        );
        manager.get_connection().execute(trigger_stmt).await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop trigger
        let drop_trigger_stmt = sea_orm::Statement::from_string(
            manager.get_database_backend(),
            "DROP TRIGGER IF EXISTS update_users_updated_at ON users".to_string(),
        );
        manager.get_connection().execute(drop_trigger_stmt).await?;

        // Drop trigger function
        let drop_fn_stmt = sea_orm::Statement::from_string(
            manager.get_database_backend(),
            "DROP FUNCTION IF EXISTS update_updated_at_column()".to_string(),
        );
        manager.get_connection().execute(drop_fn_stmt).await?;

        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await?;

        let stmt = sea_orm::Statement::from_string(
            manager.get_database_backend(),
            "DROP EXTENSION IF EXISTS citext".to_string(),
        );
        manager.get_connection().execute(stmt).await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Citext {
    Citext,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Email,
    Password,
    CreatedAt,
    UpdatedAt,
}
