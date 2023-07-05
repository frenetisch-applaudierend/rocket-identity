use std::collections::HashSet;

#[derive(Debug, Default, Clone)]
pub struct Roles {
    pub roles: HashSet<String>,
}

impl Roles {
    pub fn new() -> Self {
        Self {
            roles: HashSet::new(),
        }
    }

    pub fn add(&mut self, role: &str) {
        self.roles.insert(role.into());
    }

    pub fn remove(&mut self, role: &str) {
        self.roles.remove(role);
    }

    pub fn contains(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
}

impl Roles {
    pub(crate) fn from_inner(roles: HashSet<String>) -> Self {
        Self { roles }
    }

    pub(crate) fn into_inner(self) -> HashSet<String> {
        self.roles
    }
}
