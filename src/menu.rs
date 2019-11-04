//! States can implement the `Menu` trait.
//! Provides simple ui helper methods.
//!
//! # Boilerplate
//!```
//! # use amethyst::assets::ProgressCounter;
//! # use amethyst::ecs::{Entities, Entity, Join, ReadStorage, WorldExt, Write};
//! # use amethyst::shrev::{EventChannel, ReaderId};
//! # use amethyst::ui::{UiCreator, UiEvent, UiEventType, UiTransform};
//! # use amethyst::{State, StateData, StateEvent, Trans};
//!
//! use deathframe::menu::{Menu, UiData};
//!
//! type MyGameData = ();
//! type MyStateEvent = ();
//!
//! #[derive(Default)]
//! struct ExampleState {
//!     ui_data: UiData,
//! }
//!
//! impl<MyGameData, MyStateEvent> Menu<MyGameData, MyStateEvent> for ExampleState {
//!     fn event_triggered(
//!         &mut self,
//!         data: &mut StateData<MyGameData>,
//!         event_name: String,
//!     ) -> Option<Trans<MyGameData, MyStateEvent>> {
//!         match event_name.as_ref() {
//!             "btn_quit" => Some(Trans::Pop),
//!             _ => None,
//!         }
//!     }
//!
//!     fn ui_data(&self) -> &UiData {
//!         &self.ui_data
//!     }
//!
//!     fn ui_data_mut(&mut self) -> &mut UiData {
//!         &mut self.ui_data
//!     }
//! }
//!```

use amethyst::assets::ProgressCounter;
use amethyst::ecs::{Entities, Entity, Join, ReadStorage, WorldExt, Write};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiCreator, UiEvent, UiTransform};
use amethyst::{StateData, Trans};

pub mod prelude {
    pub use super::Menu;
    pub use super::UiData;
}

#[derive(Default)]
pub struct UiData {
    ui_entities:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

pub trait Menu<T, E> {
    /// Returns a shared reference to the UiData.
    fn ui_data(&self) -> &UiData;

    /// Returns a mutable reference to the UiData.
    fn ui_data_mut(&mut self) -> &mut UiData;

    /// This method is called when an event is triggered.
    /// It is passed the `event_name` as a `String`.
    /// Internally, this method is called from `update_ui_events`,
    /// see `update_ui_events` for more information.
    fn event_triggered(
        &mut self,
        data: &mut StateData<T>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<T, E>>;

    /// Call this method to create the UI entities, specified in the UI's ron file.
    fn create_ui<S>(
        &mut self,
        data: &mut StateData<T>,
        ron_path: S,
    ) -> ProgressCounter
    where
        S: ToString,
    {
        let mut progress = ProgressCounter::new();

        let menu_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(ron_path.to_string(), &mut progress)
        });
        self.push_ui_entity(menu_entity);

        progress
    }

    fn push_ui_entity(&mut self, entity: Entity) {
        self.ui_data_mut().ui_entities.push(entity);
    }

    /// Deletes the created UI entities.
    fn delete_ui(&mut self, data: &mut StateData<T>) {
        data.world
            .delete_entities(&self.ui_data().ui_entities)
            .unwrap();
        self.ui_data_mut().ui_entities.clear();
    }

    /// This method should be called every tick.
    /// I usually call this from my state's `fixed_update` method.
    /// This method will call the `event_triggered` method, if a UI element is clicked.
    fn update_ui_events(
        &mut self,
        data: &mut StateData<T>,
    ) -> Option<Trans<T, E>> {
        let mut triggered_event = None;

        data.world.exec(
            |(entities, mut events, ui_transforms): (
                Entities,
                Write<EventChannel<UiEvent>>,
                ReadStorage<UiTransform>,
            )| {
                let reader_id = self
                    .ui_data_mut()
                    .ui_reader_id
                    .get_or_insert_with(|| events.register_reader());

                for event in events.read(reader_id) {
                    let target_entity_id = event.target.id();
                    if let Some(name) = (&entities, &ui_transforms)
                        .join()
                        .find_map(|(entity, transform)| {
                            if entity.id() == target_entity_id {
                                Some(transform.id.to_string())
                            } else {
                                None
                            }
                        })
                    {
                        triggered_event = Some((name, event.clone()));
                    }
                }
            },
        );

        if let Some((event_name, event)) = triggered_event {
            let trans_opt = self.event_triggered(data, event_name, event);
            if trans_opt.is_some() {
                trans_opt
            } else {
                None
            }
        } else {
            None
        }
    }
}
