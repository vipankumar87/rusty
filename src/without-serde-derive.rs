#![allow(dead_code, unused_imports)]

use serde::{
    ser::{Serialize, SerializeStruct, Serializer},
    de::{Deserialize, Deserializer, Visitor, MapAccess},
};
use std::fmt;

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

/* ---------- Serialize (MANUAL) ---------- */
impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("name", &self.name)?;
        state.end()
    }
}

/* ---------- Deserialize (MANUAL) ---------- */
impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Id,
            Name,
        }

        struct UserVisitor;

        impl<'de> Visitor<'de> for UserVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct User")
            }

            fn visit_map<V>(self, mut map: V) -> Result<User, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut name = None;

                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "id" => id = Some(map.next_value()?),
                        "name" => name = Some(map.next_value()?),
                        _ => {
                            let _: serde::de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;
                let name = name.ok_or_else(|| serde::de::Error::missing_field("name"))?;

                Ok(User { id, name })
            }
        }

        deserializer.deserialize_struct(
            "User",
            &["id", "name"],
            UserVisitor,
        )
    }
}

pub fn test() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
    };

    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("{}", json);

    let parsed: User = serde_json::from_str(&json).unwrap();
    println!("{:?}", parsed);
}
