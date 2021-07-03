use super::*;

#[test]
fn parse_empty_database() {
    const EMPTY_DB: &str = r#"
{
  "object": "database",
  "id": "668d797c-76fa-4934-9b05-ad288df2d136",
  "created_time": "2020-03-17T19:10:04.968Z",
  "last_edited_time": "2020-03-17T21:49:37.913Z",
  "title": [
    {
      "type": "text",
      "text": {
        "content": ""
      }
    }
  ],
  "parent": {
    "type": "page_id",
    "page_id": "48f8fee9-cd79-4180-bc2f-ec0398253067"
  },
  "properties": {}
}
"#;
    serde_json::from_str::<Database>(EMPTY_DB).unwrap();
}

#[test]
fn parse_sample_database() {
    serde_json::from_str::<Database>(include_str!("sample_db.json")).unwrap();
}
