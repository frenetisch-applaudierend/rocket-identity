CREATE TABLE tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    owner VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO tasks (owner, description) VALUES ("user1", "demo task");
INSERT INTO tasks (owner, description) VALUES ("user2", "demo task2");
