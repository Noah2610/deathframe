use amethyst::ecs::EntityBuilder;
use amethyst::prelude::Builder;
use regex::Regex;

use super::prelude::*;

pub mod prelude {
    pub use super::add_component_to_entity_by_name;
}

#[rustfmt::skip]
pub fn add_component_to_entity_by_name<'a, T>(
    mut entity: EntityBuilder<'a>,
    component_name: T,
) -> EntityBuilder<'a> where T: ToString {
    let re = Regex::new(r"(?P<name>\w+)(?P<params>\{.*\})?").unwrap();

    if let Some(capture) = re.captures(&component_name.to_string()) {
        match (
            capture.name("name").map(|x| x.as_str()).unwrap_or(""),
            capture.name("params").map(|x| x.as_str()).unwrap_or(""),
        ) {
            ("CheckCollision", _) => entity = entity.with(CheckCollision),
            ("Collision", _)      => entity = entity.with(Collision::default()),
            ("Push", _)           => entity = entity.with(Push),
            ("Pushable", _)       => entity = entity.with(Pushable),
            ("Solid", data)       => {
                if data.is_empty() {
                    entity = entity.with(Solid::<()>::default());
                } else {
                    if let Ok(deserialized) = serde_json::from_str::<Solid<String>>(data) {
                        entity = entity.with(deserialized);
                    } else {
                        panic!(format!(
                            "Couldn't deserialize JSON data for \
                             Solid:\n{:#?}",
                            data
                        ))
                    }
                }
            }
            ("DecreaseVelocity", data) => {
                if let Ok(deserialized) =
                    serde_json::from_str::<DecreaseVelocity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for \
                         DecreaseVelocity:\n{:#?}",
                        data
                    ))
                }
            }
            ("Gravity", data) => {
                if let Ok(deserialized) = serde_json::from_str::<Gravity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for Gravity:\n{:#?}",
                        data
                    ))
                }
            }
            ("MaxVelocity", data) => {
                if let Ok(deserialized) =
                    serde_json::from_str::<MaxVelocity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for \
                         MaxVelocity:\n{:#?}",
                        data
                    ))
                }
            }
            ("Velocity", mut data) => {
                if data.is_empty() {
                    data = "{}"
                }
                if let Ok(deserialized) = serde_json::from_str::<Velocity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for Velocity:\n{:#?}",
                        data
                    ))
                }
            }
            _ => (),
        }
    }

    entity
}
