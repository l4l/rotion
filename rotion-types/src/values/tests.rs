use super::*;

impl Eq for RichText {}

#[test]
fn rich_text_sample() {
    const S: &str = r#"[
          {
            "type": "text",
            "text": {
              "content": "Some more text with "
            }
          },
          {
            "type": "text",
            "text": {
              "content": "some"
            },
            "annotations": {
              "italic": true
            }
          },
          {
            "type": "text",
            "text": {
              "content": " "
            }
          },
          {
            "type": "text",
            "text": {
              "content": "fun"
            },
            "annotations": {
              "bold": true
            }
          },
          {
            "type": "text",
            "text": {
              "content": " "
            }
          },
          {
            "type": "text",
            "text": {
              "content": "formatting"
            },
            "annotations": {
              "color": "pink"
            }
          }
        ]"#;
    let rich_text = serde_json::from_str::<RichTexts>(S).unwrap();
    let expected: RichTexts = vec![
        RichText::Text {
            text: TextInner {
                content: "Some more text with ".into(),
                link: None,
            },
            common: Default::default(),
        },
        RichText::Text {
            text: TextInner {
                content: "some".into(),
                link: None,
            },
            common: RichTextCommon {
                annotations: Some(Annotation {
                    italic: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
        },
        RichText::Text {
            text: TextInner {
                content: " ".into(),
                link: None,
            },
            common: Default::default(),
        },
        RichText::Text {
            text: TextInner {
                content: "fun".into(),
                link: None,
            },
            common: RichTextCommon {
                annotations: Some(Annotation {
                    bold: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
        },
        RichText::Text {
            text: TextInner {
                content: " ".into(),
                link: None,
            },
            common: Default::default(),
        },
        RichText::Text {
            text: TextInner {
                content: "formatting".into(),
                link: None,
            },
            common: RichTextCommon {
                annotations: Some(Annotation {
                    color: Some(Color::Pink),
                    ..Default::default()
                }),
                ..Default::default()
            },
        },
    ];
    assert_eq!(rich_text, expected);
}

#[test]
fn rich_text_different_types() {
    const S: &str = r#"[
        {
          "type": "text",
          "text": {
            "content": "Text goes here ",
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
          "plain_text": "Text goes here ",
          "href": null
        },
        {
          "type": "mention",
          "mention": {
            "type": "user",
            "user": {
              "object": "user",
              "id": "00000000-0000-0000-0000-000000000000",
              "name": "sample nickname",
              "avatar_url": "https://example.com/photo.jpg",
              "type": "person",
              "person": {
                "email": "email@example.com"
              }
            }
          },
          "annotations": {
            "bold": false,
            "italic": false,
            "strikethrough": false,
            "underline": false,
            "code": false,
            "color": "default"
          },
          "plain_text": "@sample nickname",
          "href": null
        }
      ]"#;

    let rich_text = serde_json::from_str::<RichTexts>(S).unwrap();

    assert!(
        matches!(rich_text[0], RichText::Text { .. })
            && matches!(rich_text[1], RichText::Mention { .. })
    );
}
