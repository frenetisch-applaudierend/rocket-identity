use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Roles {
    pub roles: HashSet<String>,
}

impl Roles {
    pub fn new() -> Self {
        Self {
            roles: HashSet::new(),
        }
    }

    pub fn add_role(&mut self, role: &str) {
        self.roles.insert(role.into());
    }

    pub fn remove_role(&mut self, role: &str) {
        self.roles.remove(role);
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.contains(role)
    }
}
