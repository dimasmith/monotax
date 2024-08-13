-- Add migration script here
create table reconciliation (
    id binary(16) primary key,
    income_id integer not null references income(id),
    payment_id integer not null references payment(id),
    amount double not null default 0,
    reconciliation_date timestamp not null default current_timestamp,
    reconciled varchar(20) check (reconciled in ('fully', 'partially')) not null default 'fully'
);
