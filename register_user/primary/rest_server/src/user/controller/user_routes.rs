#[actix_web::scope("/users")]
pub mod user_routes {
    use crate::user::dto::{UserToCreateRequestDTO, UserToResponseDTO, UsersResponseDTO, UsersToResponseDTO};
    use actix_web::web::{Data, Json, Path};
    use actix_web::{get, post, HttpResponse, Responder};
    use domain::user::ports::primary::{CreateUserPort, GetUserPort};
    use std::sync::Arc;
    use uuid::Uuid;

    #[post("")]
    pub async fn create(
        create_user_port: Data<Arc<dyn CreateUserPort>>,
        user_to_create_request_dto: Json<UserToCreateRequestDTO>,
    ) -> impl Responder {
        let _response = create_user_port
            .create(user_to_create_request_dto.0.to_model())
            .to_user_response_dto();

        Json(_response)
    }

    #[get("/{id}")]
    pub async fn get_by_id(
        get_user_port: Data<Arc<dyn GetUserPort>>,
        id: Path<String>,
    ) -> impl Responder {
        let _id = Uuid::parse_str(id.into_inner().as_str()).unwrap();
        let _response = match get_user_port.get_by_id(_id) {
            Some(user) => user.to_users_response_dto(),
            None => UsersResponseDTO::default()
        };

        Json(_response)
    }

    #[get("")]
    pub async fn get_all(get_user_port: Data<Arc<dyn GetUserPort>>) -> HttpResponse {
        let _response = get_user_port
            .get_all()
            .to_users_response_dto();

        HttpResponse::Ok().json(_response)
    }
}
