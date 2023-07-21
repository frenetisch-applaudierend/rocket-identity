use std::marker::PhantomData;

use rocket::{Orbit, Request, Rocket};

use crate::{UserStore, UserStoreScope};

use super::DieselScopeProvider;

pub struct DieselUserStore<P: DieselScopeProvider> {
    _marker: PhantomData<std::sync::Mutex<P>>,
}

#[rocket::async_trait]
impl<P: DieselScopeProvider> UserStore for DieselUserStore<P> {
    async fn create_request_scope<'r>(&self, req: &'r Request<'_>) -> Box<dyn UserStoreScope> {
        Box::new(P::create_from_request(req).await.unwrap())
    }

    async fn create_global_scope(&self, rocket: &Rocket<Orbit>) -> Option<Box<dyn UserStoreScope>> {
        Some(Box::new(P::create_from_rocket(rocket).await.unwrap()))
    }
}

impl<P: DieselScopeProvider> core::fmt::Debug for DieselUserStore<P> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DieselUserStore").finish()
    }
}
