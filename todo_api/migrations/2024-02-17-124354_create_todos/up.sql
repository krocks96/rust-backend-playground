CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE
);
