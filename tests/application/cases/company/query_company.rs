#[cfg(test)]
mod query_company_test_suite {
    use std::sync::Arc;

    use axum::http::StatusCode;
    use lib::{
        application::{
            SecureCase, cases::company::QueryCompanyUseCase, dtos::company::ReqQueryCompanyDto,
        },
        infrastructure::{
            db::{DbContext, entities::companies},
            helpers::token::JwtHelper,
        },
        presentation::{guards::UserInfo, http::AppState, middlewares::validator::QueryParams},
    };
    use sea_orm::{DatabaseBackend, DbErr, MockDatabase};
    use uuid::Uuid;

    #[tokio::test]
    async fn return_found_companies_on_success() -> Result<(), DbErr> {
        // Given
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

        let state = AppState {
            db_context: Arc::new(DbContext::new(Arc::new(db))),
            jwt_helper: Arc::new(JwtHelper::new("secret".to_owned())),
        };

        let uc = QueryCompanyUseCase::new(Arc::new(state));

        let user = UserInfo {
            id: "logon-user".to_owned(),
        };

        let dto = ReqQueryCompanyDto { name: None };

        // When
        let result = uc.execute(QueryParams(dto), user).await;

        // Then
        assert!(result.is_ok());

        let res = result.unwrap();
        assert_eq!(res.status, StatusCode::OK);
        assert_eq!(res.data.len(), 2);
        assert_eq!(res.data[0].name, "test-1");
        assert_eq!(res.data[1].name, "test-2");

        Ok(())
    }
}
