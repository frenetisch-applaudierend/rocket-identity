use std::collections::HashMap;

pub struct TodoService {
    pub db: HashMap<uuid::Uuid, TodoList>,
}

impl TodoService {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }

    pub fn get_all_lists(&self) -> Vec<&TodoList> {
        self.db.values().collect()
    }

    pub fn create_list(&mut self, name: String) -> &TodoList {
        let id = uuid::Uuid::new_v4();

        let list = TodoList {
            id: id.clone(),
            name,
            items: Vec::new(),
        };

        self.db.insert(id, list);

        self.db.get(&id).unwrap()
    }
}

pub struct TodoList {
    pub id: uuid::Uuid,
    pub name: String,
    pub items: Vec<TodoItem>,
}

pub struct TodoItem {
    pub id: uuid::Uuid,
    pub name: String,
    pub done: bool,
}
