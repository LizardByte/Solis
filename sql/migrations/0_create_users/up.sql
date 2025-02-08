CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    password_salt TEXT NOT NULL,
    pin TEXT DEFAULT NULL,
    pin_salt TEXT DEFAULT NULL,
    admin BOOLEAN NOT NULL DEFAULT FALSE
);
