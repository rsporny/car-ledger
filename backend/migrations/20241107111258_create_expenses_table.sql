-- migrations/20211015123456_create_expenses_table.sql

CREATE TABLE expenses (
    id UUID PRIMARY KEY,
    amount DOUBLE PRECISION NOT NULL,
    description TEXT NOT NULL,
    date TEXT NOT NULL
);
