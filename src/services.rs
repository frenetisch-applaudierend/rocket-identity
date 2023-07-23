use std::sync::Arc;

use rocket::{Orbit, Request, Rocket};

use crate::{
    config::MissingAuthPolicy, stores::UserStore, AuthenticationSchemes, PasswordHasher,
    UserRepository,
};

#[rocket::async_trait]
pub trait Services {
    async fn user_repository(&self) -> UserRepository;

    fn authentication_schemes(&self) -> &AuthenticationSchemes;
}

pub(crate) trait InternalServices {
    fn user_store(&self) -> &dyn UserStore;

    fn password_hasher(&self) -> &Arc<dyn PasswordHasher>;

    fn missing_auth_policy(&self) -> MissingAuthPolicy;
}

#[rocket::async_trait]
impl<'r> Services for Request<'r> {
    async fn user_repository(&self) -> UserRepository {
        let user_store = self.user_store();
        let password_hasher = self.password_hasher();

        let scope = user_store.create_request_scope(self).await;

        UserRepository::new(scope, password_hasher.clone())
    }

    fn authentication_schemes(&self) -> &AuthenticationSchemes {
        self.rocket().authentication_schemes()
    }
}

impl<'r> InternalServices for Request<'r> {
    fn user_store(&self) -> &dyn UserStore {
        self.rocket().user_store()
    }

    fn password_hasher(&self) -> &Arc<dyn PasswordHasher> {
        self.rocket().password_hasher()
    }

    fn missing_auth_policy(&self) -> MissingAuthPolicy {
        self.rocket().missing_auth_policy()
    }
}

#[rocket::async_trait]
impl Services for Rocket<Orbit> {
    async fn user_repository(&self) -> UserRepository {
        let user_store = self.user_store();
        let password_hasher = self.password_hasher();

        let scope = user_store
            .create_global_scope(self)
            .await
            .expect("Configured UserStore does not support global scopes");

        UserRepository::new(scope, password_hasher.clone())
    }

    fn authentication_schemes(&self) -> &AuthenticationSchemes {
        self.state()
            .expect("Missing required AuthenticationSchemes")
    }
}

impl InternalServices for Rocket<Orbit> {
    fn user_store(&self) -> &dyn UserStore {
        &**self
            .state::<Box<dyn UserStore>>()
            .expect("Missing required UserStore")
    }

    fn password_hasher(&self) -> &Arc<dyn PasswordHasher> {
        self.state().expect("Missing required PasswordHasher")
    }

    fn missing_auth_policy(&self) -> MissingAuthPolicy {
        *self.state().expect("Missing required MissingAuthPolicy")
    }
}
