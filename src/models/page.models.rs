#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub id: i32,
    pub created: NaiveDateTime,
    pub updated: Option<NaiveDateTime>
}
#[derive(Insertable, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct NewPage {
    
}

#[derive(AsChangeSet, Deserialize, Serialize)]
#[table_name = "pages"]
pub struct UpdatePage {

}