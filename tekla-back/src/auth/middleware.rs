// src/auth/middleware.rs - VERSIÓN DEFINITIVA
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, http,
};
use std::future::{ready, Ready};
use std::rc::Rc;

use crate::auth::jwt::Claims;
use crate::auth::jwt_service::JwtService;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;  // ¡Usamos S::Future directamente!

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clonar el JwtService si existe
        let jwt_service = req.app_data::<actix_web::web::Data<JwtService>>()
            .map(|data| data.clone());
        
        // Verificar token si existe
        if let Some(jwt_service) = jwt_service {
            if let Some(auth_header) = req.headers().get(http::header::AUTHORIZATION) {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..];
                        
                        // Intentar validar el token (no bloqueante)
                        if let Ok(claims) = jwt_service.validate_access_token(token) {
                            // Insertar claims en las extensiones si el token es válido
                            req.extensions_mut().insert(claims);
                        }
                    }
                }
            }
        }
        
        // Pasar la request al servicio original
        self.service.call(req)
    }
}

// Extractor para obtener los claims del request
use actix_web::{FromRequest, HttpRequest};

//pub struct AuthenticatedUser(pub Claims);
pub struct AuthenticatedUser {
    pub user_id: String,
    pub email: String,
    pub role: crate::auth::jwt::UserRole,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        if let Some(claims) = req.extensions().get::<Claims>() {
            ready(Ok(AuthenticatedUser {
                user_id: claims.sub.clone(),
                email: claims.email.clone(),
                role: claims.role,
            }))
        } else {
            ready(Err(actix_web::error::ErrorUnauthorized("No autenticado")))
        }
    }
}