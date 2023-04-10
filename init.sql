CREATE TABLE IF NOT EXISTS transactions (
    id SERIAL PRIMARY KEY,
    value FLOAT NOT NULL,
    business_name TEXT NOT NULL,
    category TEXT NOT NULL
);
