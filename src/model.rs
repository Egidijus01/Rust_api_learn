// use parking_lot::RwLock;
// use serde::Serialize;
// use serde::Deserialize;
// use std::collections::HashMap;
// use std::sync::Arc;

// type Items = HashMap<String, i32>;

// #[derive(Debug, Deserialize, Serialize, Clone)]
// pub struct Item {
//     name: String,
//     quantity: i32,
// }

// #[derive(Clone)]
// pub struct Store {
//   pub grocery_list: Arc<RwLock<Items>>
// }

// impl Store {
//     pub fn new() -> Self {
//         Store {
//             grocery_list: Arc::new(RwLock::new(HashMap::new())),
//         }
//     }
// }


use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]

pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed: Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,

}

pub fn todo_db() -> DB {
    Arc::new(Mutex::new(Vec::new()))
}

#[derive(Debug, Deserialize)]


//schema.rs



pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,

}

pub struct UpdateTodoSchema{
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,


}