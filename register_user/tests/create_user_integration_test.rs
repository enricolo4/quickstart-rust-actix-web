#[cfg(test)]
mod create_user_integration_test {
    use actix_web::{test, App};
    use actix_web::web::Data;
    use register_user::{create, CREATE_USER_PORT, GET_USER_PORT, USER_DATA_ACCESS_CONTAINER};
    use serde_json::json;
    use testcontainers_modules::postgres;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;
    use uuid::Uuid;
    use rest_server::user::controller::user_routes::user_routes::{get_all, get_by_id};
    use rest_server::user::dto::UserResponseDTO;

    #[actix_web::test]
    async fn test_create_user() {
        // Given(a postgres container)
        let postgres_container = postgres::Postgres::default().start().await.unwrap();
        let host_port = postgres_container.get_host_port_ipv4(5432).await.unwrap();
        let connection_string =
            format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres");

        std::env::set_var("REGISTER_USER_DATABASE_URL", connection_string);

        // And(a running app)
        let app = test::init_service(
            App::new()
                .app_data(Data::new(CREATE_USER_PORT.clone()))
                .service(create)
                .app_data(Data::new(GET_USER_PORT.clone()))
                .service(get_by_id)
                .service(get_all)
        ).await;

        // When(call create user endpoint)
        let new_user = json!({
            "name": "Test User",
            "email": "test@example.com"
        });

        let request = test::TestRequest::post()
            .uri("/users")
            .set_json(&new_user)
            .to_request();


        let response = test::call_service(&app, request).await;

        // Then(response success)
        assert!(response.status().is_success());

        let body = response.into_body();
        let response_body_bytes = actix_web::body::to_bytes(body).await.unwrap();
        let user_response_dto: UserResponseDTO = serde_json::from_slice(&response_body_bytes).unwrap();

        let user_data_access_port = USER_DATA_ACCESS_CONTAINER.clone();

        let user_id = Uuid::parse_str(&user_response_dto.id.as_str()).unwrap();

        // Then(save new user in database)
        let user = user_data_access_port.find_by_id(user_id).unwrap();

        assert_eq!(user_response_dto.name, "Test User");
        assert_eq!(user_response_dto.email, "test@example.com");
        assert_eq!(user.id, user_id);
        assert_eq!(user.name, user_response_dto.name);
        assert_eq!(user.email, user_response_dto.email);
    }
}
