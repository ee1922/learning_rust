CREATE TABLE IF NOT EXISTS item
(
    id SERIAL PRIMARY KEY NOT NULL,
    table_id INT,
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    prep_time INT DEFAULT 10,
    item_name VARCHAR(255)
);
