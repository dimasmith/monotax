-- change amount field type to double
CREATE TABLE income_sqlx (
            date DATETIME NOT NULL,
            amount DOUBLE NOT NULL,
            payment_no INTEGER NOT NULL UNIQUE,
            description TEXT,
            year SMALLINT NOT NULL,
            quarter TINYINT NOT NULL,
            tax_paid BOOLEAN NOT NULL DEFAULT false,
            PRIMARY KEY (date, amount)
        );

INSERT INTO income_sqlx SELECT * FROM income;

DROP TABLE income;

ALTER TABLE income_sqlx RENAME TO income;
