CREATE TABLE IF NOT EXISTS income (
            date DATETIME NOT NULL,
            amount DECIMAL(10,2) NOT NULL,
            payment_no INTEGER NOT NULL UNIQUE,
            description TEXT,
            year INTEGER NOT NULL,
            quarter INTEGER NOT NULL,
            tax_paid BOOL DEFAULT false,
            PRIMARY KEY (date, amount)
        );