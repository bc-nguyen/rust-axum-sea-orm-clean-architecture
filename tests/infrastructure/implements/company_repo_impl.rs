#[cfg(test)]
mod company_repo_test_suite {
    use lib::{
        application::dtos::company::ReqQueryCompanyDto,
        domain::organization::repositories::CompanyRepository,
        infrastructure::db::{RepositoryProvider, entities::companies},
    };
    use sea_orm::{DatabaseBackend, DbErr, MockDatabase, Transaction};
    use uuid::Uuid;

    #[tokio::test]
    async fn query_faild_when_no_data() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

        let result = {
            let provider = RepositoryProvider::new(&db);
            let company_repo = provider.company_repo();
            company_repo.query(&ReqQueryCompanyDto { name: None }).await
        };

        assert!(result.is_err());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "companies"."id", "companies"."name" FROM "companies""#,
                []
            ),]
        );

        Ok(())
    }

    #[tokio::test]
    async fn query_return_all_data_when_no_name() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![
                companies::Model {
                    id: Uuid::new_v4(),
                    name: "test-1".to_owned(),
                },
                companies::Model {
                    id: Uuid::new_v4(),
                    name: "test-2".to_owned(),
                },
            ]])
            .into_connection();

        let result = {
            let provider = RepositoryProvider::new(&db);
            let company_repo = provider.company_repo();
            company_repo.query(&ReqQueryCompanyDto { name: None }).await
        };

        assert!(result.is_ok());

        let items = result.unwrap();
        assert!(items.len() == 2);
        assert_eq!(items[0].name, "test-1");
        assert_eq!(items[1].name, "test-2");

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "companies"."id", "companies"."name" FROM "companies""#,
                []
            ),]
        );

        Ok(())
    }

    #[tokio::test]
    async fn query_return_filtered_data_by_name() -> Result<(), DbErr> {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![companies::Model {
                id: Uuid::new_v4(),
                name: "test-1".to_owned(),
            }]])
            .into_connection();

        let result = {
            let provider = RepositoryProvider::new(&db);
            let company_repo = provider.company_repo();
            company_repo
                .query(&ReqQueryCompanyDto {
                    name: Some("test-1".to_owned()),
                })
                .await
        };

        assert!(result.is_ok());

        let items = result.unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "test-1");
        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "companies"."id", "companies"."name" FROM "companies" WHERE "companies"."name" LIKE $1"#,
                ["%test-1%".into()]
            ),]
        );

        Ok(())
    }

    #[tokio::test]
    async fn exists_return_true_when_found() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![companies::Model {
                id,
                name: "test-1".to_owned(),
            }]])
            .into_connection();

        let result = {
            let provider = RepositoryProvider::new(&db);
            let company_repo = provider.company_repo();
            company_repo.exists(id).await
        };

        assert!(result.is_ok());
        assert!(result.unwrap());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "companies"."id", "companies"."name" FROM "companies" WHERE "companies"."id" = $1 LIMIT $2"#,
                [id.into(), 1u64.into()]
            ),]
        );

        Ok(())
    }

    #[tokio::test]
    async fn exists_return_false_when_not_found() -> Result<(), DbErr> {
        let id = Uuid::new_v4();
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([Vec::<companies::Model>::new()])
            .into_connection();

        let result = {
            let provider = RepositoryProvider::new(&db);
            let company_repo = provider.company_repo();
            company_repo.exists(id).await
        };

        assert!(result.is_ok());
        assert!(!result.unwrap());

        assert_eq!(
            db.into_transaction_log(),
            [Transaction::from_sql_and_values(
                DatabaseBackend::Postgres,
                r#"SELECT "companies"."id", "companies"."name" FROM "companies" WHERE "companies"."id" = $1 LIMIT $2"#,
                [id.into(), 1u64.into()]
            ),]
        );

        Ok(())
    }
}
