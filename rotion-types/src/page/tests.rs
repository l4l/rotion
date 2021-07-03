use super::*;

#[test]
fn parse_empty_page() {
    const S: &str = r#"{
      "object": "page",
      "id": "b55c9c91-384d-452b-81db-d1ef79372b75",
      "created_time": "2020-03-17T19:10:04.968Z",
      "last_edited_time": "2020-03-17T21:49:37.913Z",
      "parent": {
        "type": "database_id",
        "database_id": "48f8fee9-cd79-4180-bc2f-ec0398253067"
      },
      "archived": false,
      "url": "https://www.notion.so/Avocado-b55c9c91384d452b81dbd1ef79372b75",
      "properties": {}
    }
    "#;
    serde_json::from_str::<Page>(S).unwrap();
}

// #[test]
// fn parse_sample_page() {
//     const S: &str = include_str!("sample_page.json");
//     serde_json::from_str::<Page>(S).unwrap();
// }
