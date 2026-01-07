//use axum::{Json, extract, extract::Query, response::IntoResponse};
//use chrono::Utc;
//use reqwest::*;
//use serde::{Deserialize, Serialize};
//use serde_xml_rs::from_str;
//use std::collections::HashMap;

use rusqlite::{Connection, Result};
//use std::error::Error;

#[derive(Debug)]
struct Ratex {
    date: i32,
    target_value: f64,
}

#[derive(Debug, Default)]
struct Rate {
    date: i32,
    jpy: f64,
    czk: f64,
    dkk: f64,
    gbp: f64,
    huf: f64,
    pln: f64,
    ron: f64,
    sek: f64,
    chf: f64,
    isk: f64,
    nok: f64,
    aud: f64,
    brl: f64,
    cad: f64,
    cny: f64,
    hkd: f64,
    idr: f64,
    ils: f64,
    inr: f64,
    krw: f64,
    mxn: f64,
    myr: f64,
    nzd: f64,
    php: f64,
    sgd: f64,
    thb: f64,
    zar: f64,
}

pub async fn last(target_code: &str) -> Result<Vec<f64>> {
    let mut v: Vec<f64> = vec![];
    let conn = Connection::open("rate.db").unwrap();

    let mut stmt = conn
        .prepare(&format!(
            "SELECT date, {0} FROM rate ORDER BY date desc LIMIT 1 ",
            target_code
        ))
        .unwrap();

    let rate_iter = stmt
        .query_map([], |row| {
            Ok(Ratex {
                date: row.get(0)?,
                target_value: row.get(1)?,
            })
        })
        .unwrap();

    for rate in rate_iter {
        let rx = rate.unwrap();
        v.push(f64::from(rx.date));
        v.push(rx.target_value);
    }

    Ok(v)
}

pub async fn insert() -> Result<()> {
    let mut rate = Rate::default();

    /*
    conn.execute(
        "INSERT INTO rate (date, THB) VALUES (?1, ?2)",
        (&d.date, &d.THB),
    )
    .unwrap();


    let now = Utc::now();
    let today: i32 = now.format("%Y%m%d00").to_string().parse().unwrap();

    }

    // let x = process_request().await;
    //println!("{:?}", x.await);
    */

    Ok(())
}

pub async fn new() -> Result<()> {
    let conn = Connection::open("rate.db").unwrap();
    conn.execute(
        "CREATE TABLE rate (
date INTEGER PRIMARY KEY,
jpy real not null default 0.0,
czk real not null default 0.0,
dkk real not null default 0.0,
gbp real not null default 0.0,
huf real not null default 0.0,
pln real not null default 0.0,
ron real not null default 0.0,
sek real not null default 0.0,
chf real not null default 0.0,
isk real not null default 0.0,
nok real not null default 0.0,
try real not null default 0.0,
aud real not null default 0.0,
brl real not null default 0.0,
cad real not null default 0.0,
cny real not null default 0.0,
hkd real not null default 0.0,
idr real not null default 0.0,
ils real not null default 0.0,
inr real not null default 0.0,
krw real not null default 0.0,
mxn real not null default 0.0,
myr real not null default 0.0,
nzd real not null default 0.0,
php real not null default 0.0,
sgd real not null default 0.0,
thb real not null default 0.0,
zar real not null default 0.0,
rub real not null default 0.0

        )",
        (), // empty list of parameters.
    )?;

    Ok(())
}
