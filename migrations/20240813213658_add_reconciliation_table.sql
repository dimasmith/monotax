-- Add migration script here
create table reconciliation (
    id blob primary key not null,
    income_id integer not null references income(payment_no),
    payment_id integer not null references payment(id),
    amount double not null default 0,
    reconciliation_date timestamp not null default current_timestamp,
    reconciled varchar(20) check (reconciled in ('fully', 'partially')) not null default 'fully'
);
