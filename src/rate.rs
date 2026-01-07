use crate::database;
//use crate::exchange;
use axum::{Json, extract::Query, response::IntoResponse};
//use chrono::Utc;
use database::{insert, last, new};
//use exchange::ecb;
//use rusqlite::{Connection, Result};
use serde::Serialize;
use std::collections::HashMap;
//use std::error::Error;
#[derive(Debug, Serialize)]
struct Rated {
    target_code: String,
    source_code: String,
    source_value: f64,
    target_value: f64,
    msg: String,
    date: String,
}

pub async fn daily(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    //let y = ecb().await;

    let mut r = Rated {
        target_code: "".to_string(),
        source_code: "".to_string(),
        source_value: 0.0,
        target_value: 0.0,
        date: "".to_string(),
        msg: "".to_string(),
    };
    if params.contains_key("t") && params.contains_key("s") && params.contains_key("v") {
        r.source_code = params["s"].to_string();
        r.target_code = params["t"].to_string();
        r.source_value = params["v"].parse().unwrap();
    }
    let _d = new().await;
    let _l = last(&r.target_code).await;
    let _l = insert().await;
    //println!("{:?}", l);

    Json(r)
}
