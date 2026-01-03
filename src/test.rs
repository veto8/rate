use axum::{Json, response::IntoResponse};
use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Rate {
    date: i32,
    JPY: f64,
    CZK: f64,
    DKK: f64,
    GBP: f64,
    HUF: f64,
    PLN: f64,
    RON: f64,
    SEK: f64,
    CHF: f64,
    ISK: f64,
    NOK: f64,
    TRY: f64,
    AUD: f64,
    BRL: f64,
    CAD: f64,
    CNY: f64,
    HKD: f64,
    IDR: f64,
    ILS: f64,
    INR: f64,
    KRW: f64,
    MXN: f64,
    MYR: f64,
    NZD: f64,
    PHP: f64,
    SGD: f64,
    THB: f64,
    ZAR: f64,
}
pub async fn test(x: u8) -> impl IntoResponse {
    let conn = Connection::open("rate.db").unwrap();

    conn.execute(
        "CREATE TABLE rate (
date INTEGER PRIMARY KEY,
JPY REAL NOT NULL DEFAULT 0.0,
CZK REAL NOT NULL DEFAULT 0.0,
DKK REAL NOT NULL DEFAULT 0.0,
GBP REAL NOT NULL DEFAULT 0.0,
HUF REAL NOT NULL DEFAULT 0.0,
PLN REAL NOT NULL DEFAULT 0.0,
RON REAL NOT NULL DEFAULT 0.0,
SEK REAL NOT NULL DEFAULT 0.0,
CHF REAL NOT NULL DEFAULT 0.0,
ISK REAL NOT NULL DEFAULT 0.0,
NOK REAL NOT NULL DEFAULT 0.0,
TRY REAL NOT NULL DEFAULT 0.0,
AUD REAL NOT NULL DEFAULT 0.0,
BRL REAL NOT NULL DEFAULT 0.0,
CAD REAL NOT NULL DEFAULT 0.0,
CNY REAL NOT NULL DEFAULT 0.0,
HKD REAL NOT NULL DEFAULT 0.0,
IDR REAL NOT NULL DEFAULT 0.0,
ILS REAL NOT NULL DEFAULT 0.0,
INR REAL NOT NULL DEFAULT 0.0,
KRW REAL NOT NULL DEFAULT 0.0,
MXN REAL NOT NULL DEFAULT 0.0,
MYR REAL NOT NULL DEFAULT 0.0,
NZD REAL NOT NULL DEFAULT 0.0,
PHP REAL NOT NULL DEFAULT 0.0,
SGD REAL NOT NULL DEFAULT 0.0,
THB REAL NOT NULL DEFAULT 0.0,
ZAR REAL NOT NULL DEFAULT 0.0,
RUB REAL NOT NULL DEFAULT 0.0
        )",
        (), // empty list of parameters.
    );

    /*
        let me = Person {
            id: 0,
            name: "Steven2".to_string(),
            data: None,
        };
        conn.execute(
            "INSERT INTO person (name, data) VALUES (?1, ?2)",
            (&me.name, &me.data),
        )
        .unwrap();

        let mut stmt = conn.prepare("SELECT id, name, data FROM person").unwrap();
        let person_iter = stmt
            .query_map([], |row| {
                Ok(Person {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    data: row.get(2)?,
                })
            })
            .unwrap();

        for person in person_iter {
            println!("Found person {:?}", person.unwrap());
    }
        */

    let r = serde_json::json!([
        {
            "test": "OK",
        }
    ]);
    Json(r)
}
