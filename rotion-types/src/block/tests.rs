use super::*;

#[test]
fn parse_sample_block() {
    let s = r#"{
        "object": "block",
        "id": "9bc30ad4-9373-46a5-84ab-0a7845ee52e6",
        "created_time": "2021-03-16T16:31:00.000Z",
        "last_edited_time": "2021-03-16T16:32:00.000Z",
        "has_children": false,
        "type": "heading_2",
        "heading_2": {
            "text": [
                {
                    "type": "text",
                    "text": {
                        "content": "Lacinato kale",
                        "link": null
                    },
                    "annotations": {
                        "bold": false,
                        "italic": false,
                        "strikethrough": false,
                        "underline": false,
                        "code": false,
                        "color": "default"
                    },
                    "plain_text": "Lacinato kale",
                    "href": null
                }
            ]
        }
    }"#;

    serde_json::from_str::<Block>(s).unwrap();
}

#[test]
fn parse_sample_blocks() {
    const S: &str = include_str!("sample_blocks.json");

    serde_json::from_str::<Vec<Block>>(S).unwrap();
}
