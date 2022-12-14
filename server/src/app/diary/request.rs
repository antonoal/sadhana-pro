use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::Value as JsonValue;

#[derive(Deserialize, Debug)]
pub struct DiaryDayUpsertRequest {
    pub diary_day: Vec<DiaryDayEntryUpsert>,
    pub cob_date: NaiveDate,
}

#[derive(Deserialize, Debug)]
pub struct DiaryDayEntryUpsert {
    pub practice: String,
    pub value: Option<JsonValue>,
}
