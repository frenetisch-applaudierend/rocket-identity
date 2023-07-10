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

    pub fn from_inner(roles: HashSet<String>) -> Self {
        Self { roles }
    }

    pub fn into_inner(self) -> HashSet<String> {
        self.roles
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

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.roles.iter().map(|s| s.as_str())
    }
}

impl From<Vec<String>> for Roles {
    fn from(roles: Vec<String>) -> Self {
        Self {
            roles: roles.into_iter().collect(),
        }
    }
}

impl From<Vec<&str>> for Roles {
    fn from(roles: Vec<&str>) -> Self {
        Self {
            roles: roles.into_iter().map(|s| s.to_owned()).collect(),
        }
    }
}
