use rocket::{Request, Orbit, Rocket};

use crate::auth::{scheme::AuthenticationSchemes, MissingAuthPolicy, UserRepository};

pub trait Services {
    fn user_repository(&self) -> &UserRepository;

    fn authentication_schemes(&self) -> &AuthenticationSchemes;
}

pub(crate) trait InternalServices {
    fn missing_auth_policy(&self) -> &MissingAuthPolicy;
}

impl<'r> Services for Request<'r> {
    fn user_repository(&self) -> &UserRepository {
        self.rocket()
            .state()
            .expect("Missing required UserRepository")
    }

    fn authentication_schemes(&self) -> &AuthenticationSchemes {
        self.rocket()
            .state()
            .expect("Missing required AuthenticationSchemes")
    }
}

impl<'r> InternalServices for Request<'r> {
    fn missing_auth_policy(&self) -> &MissingAuthPolicy {
        self.rocket()
            .state()
            .expect("Missing required MissingAuthPolicy")
    }
}

impl Services for Rocket<Orbit> {
    fn user_repository(&self) -> &UserRepository {
        self.state().expect("Missing required UserRepository")
    }

    fn authentication_schemes(&self) -> &AuthenticationSchemes {
        self.state()
            .expect("Missing required AuthenticationSchemes")
    }
}
