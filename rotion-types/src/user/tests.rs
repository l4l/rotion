use super::*;

impl Eq for User {}

#[test]
fn parse_sample_user() {
    const S: &str = r#"{
  "object": "user",
  "id": "d40e767c-d7af-4b18-a86d-55c61f1e39a4",
  "type": "person",
    "person": {
        "email": "avo@example.org"
    },
  "name": "Avocado Lovelace",
  "avatar_url": "https://secure.notion-static.com/e6a352a8-8381-44d0-a1dc-9ed80e62b53d.jpg"
}"#;
    let parsed = serde_json::from_str::<User>(S).unwrap();
    let expected = User {
        id: "d40e767c-d7af-4b18-a86d-55c61f1e39a4".parse().unwrap(),
        kind: UserKind::Person {
            person: Person {
                email: "avo@example.org".into(),
            },
        },
        name: Some("Avocado Lovelace".into()),
        avatar_url: Some(
            "https://secure.notion-static.com/e6a352a8-8381-44d0-a1dc-9ed80e62b53d.jpg".into(),
        ),
    };
    assert_eq!(parsed, expected);
}
