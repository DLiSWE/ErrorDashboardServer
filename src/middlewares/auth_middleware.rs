use actix_service::Service;
use actix_web::http::StatusCode;
use actix_web::dev::{ServiceRequest, ServiceResponse, Transform};
use chrono::{DateTime, Utc};
use futures::future::{ok, Ready};
use futures::Future;
use jsonwebtoken::{Validation, Algorithm};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Poll, Context};

use crate::config::Config;
use crate::database::create_pool;
use crate::shared::utils::errors::{MyError, HttpError};
use crate::shared::utils::jwt::validate_jwt;

pub struct JwtMiddleware;

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub iat: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub exp: DateTime<Utc>,
    pub iss: String,
    pub aud: String,
    pub data: Option<JsonValue>
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

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
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
