CREATE TABLE payment (
    id INTEGER PRIMARY KEY NOT NULL,
    amount DOUBLE NOT NULL,
    payment_date TIMESTAMP NOT NULL
);

INSERT INTO payment SELECT payment_no as id, date as payment_date, amount * 0.05 FROM income;
