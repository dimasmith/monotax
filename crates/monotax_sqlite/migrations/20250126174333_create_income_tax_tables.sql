-- Add migration script here
create table income_tax (
    id TEXT PRIMARY KEY NOT NULL,
    title VARCHAR(1000) NOT NULL UNIQUE
);

create table income_tax_rate (
    income_tax_id TEXT NOT NULL,
    rate DOUBLE NOT NULL,
    start_date DATE NOT NULL,    
    FOREIGN KEY (income_tax_id) REFERENCES income_tax(id)
);
