use std::marker::PhantomData;

use rocket::{Orbit, Request, Rocket};

use crate::{UserStore, UserStoreScope};

use super::{DieselConnectionProvider, DieselUserStoreScope};

#[derive(Debug)]
pub struct DieselUserStore<P: DieselConnectionProvider> {
    _marker: PhantomData<std::sync::Mutex<P>>,
}

#[rocket::async_trait]
impl<P: DieselConnectionProvider> UserStore for DieselUserStore<P> {
    async fn create_request_scope<'r>(&self, req: &'r Request<'_>) -> Box<dyn UserStoreScope> {
        let provider = P::create_from_request(req).await.unwrap();
        Box::new(DieselUserStoreScope::new(provider))
    }

    async fn create_global_scope(&self, rocket: &Rocket<Orbit>) -> Option<Box<dyn UserStoreScope>> {
        let provider = P::create_from_rocket(rocket).await.unwrap();
        Some(Box::new(DieselUserStoreScope::new(provider)))
    }
}
