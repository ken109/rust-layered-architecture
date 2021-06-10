use crate::schema::{tasks};
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable};

#[derive(Deserialize, Insertable, Queryable)]
#[table_name="tasks"]
pub struct NewTask {
    pub content: String,

}

#[derive(Clone, Debug, Serialize, Deserialize, Queryable)]
pub struct Task {
    pub id: i64,
    pub content: String,
    pub done: bool,
}
