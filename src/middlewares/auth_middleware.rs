use actix_service::Service;
use actix_web::http::{StatusCode, header::HeaderMap};
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use futures::future::{ok, Ready};
use futures::Future;
use jsonwebtoken::{Validation, Algorithm, TokenData, decode, DecodingKey};
use sea_orm::{EntityTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::marker::PhantomData;
use std::pin::Pin;

use crate::config::Config;
use crate::database::create_pool;
use crate::models::user_model::Entity as UserEntity;
use crate::shared::utils::errors::{MyError, HttpError};

pub struct JwtMiddleware;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub iat: i64,
    pub exp: i64,
    pub iss: String,
    pub aud: String,
}

impl<S, B, E> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = E> + 'static,
    S::Future: 'static,
    E: From<MyError> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = E;
    type Transform = JwtTransform<S, E>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtTransform { service, phantom: PhantomData })
    }
}

pub struct JwtTransform<S, E> {
    service: S,
    phantom: PhantomData<E>
}

impl<S, B, E> Service<ServiceRequest> for JwtTransform<S, E>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = E> + 'static,
    S::Future: 'static,
    E: From<MyError> + 'static,
{
    type Response = ServiceResponse<B>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Error = E;

    fn poll_ready(&self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
            let headers = req.headers().clone();
            let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let db = create_pool().await.map_err(E::from)?;

            let configs = Config::from_env().map_err(MyError::from)?;
            let secret_key = configs.secret_key;
            let jwt_issuer = configs.jwt_issuer;
            let jwt_audience = configs.jwt_audience;

            let required_claims: Vec<&str> = vec!["exp", "nbf"]; 

            let mut validation = Validation::new(Algorithm::HS256);
            validation.leeway = 60;
            validation.set_audience(&[jwt_audience]);
            validation.set_issuer(&[jwt_issuer]);
            validation.set_required_spec_claims(&required_claims);
            validation.validate_exp = true;
            validation.validate_nbf = false;
            
            if validate_jwt(&headers, &secret_key, &validation, &db).await.is_ok() {
                Ok(res)
            } else {
                let error = MyError::WebError(HttpError {
                    status: StatusCode::UNAUTHORIZED,
                    message: "Unauthorized".to_string(),
                });
                Err(E::from(error))
            }
        })
    }
}

async fn validate_jwt(headers: &HeaderMap, secret_key: &str, validation: &Validation, db: &DatabaseConnection) -> Result<(), MyError> {
    if let Some(token_header) = headers.get("Authorization") {
        let token_str = token_header.to_str().unwrap_or("");

        let decoding_key = DecodingKey::from_secret(secret_key.as_ref());

        let token_data : TokenData<Claims> = decode(token_str, &decoding_key, &validation).map_err(MyError::from)?;

        let uid = token_data.claims.sub.parse::<Uuid>()
            .map_err(|_| MyError::WebError(HttpError {
                status: StatusCode::UNAUTHORIZED,
                message: "Invalid subject".to_string(),
            }))?;

        let found_user = UserEntity::find_by_id(uid)
            .one(db).await
            .map_err(|_| MyError::WebError(HttpError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Database error".to_string(),
            }))?;

        if let Some(_found_user) = found_user {
        // Authentication success
            return Ok(());
        } else {
        // Authentication failed
            return Err(MyError::UserNotFound);
        }   

    } else {
        return Err(MyError::WebError(HttpError {
            status: StatusCode::UNAUTHORIZED,
            message: "No Authorization header".to_string(),
        }));
    }
}
