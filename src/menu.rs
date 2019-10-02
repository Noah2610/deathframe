//! States can implement the `Menu` trait.
//! Provides simple ui helper methods.

use amethyst::assets::ProgressCounter;
use amethyst::ecs::{Entities, Entity, Join, ReadStorage, WorldExt, Write};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiCreator, UiEvent, UiEventType, UiTransform};
use amethyst::{StateData, Trans};

/// # Boilerplate
///```
/// # use amethyst::assets::ProgressCounter;
/// # use amethyst::ecs::{Entities, Entity, Join, ReadStorage, WorldExt, Write};
/// # use amethyst::shrev::{EventChannel, ReaderId};
/// # use amethyst::ui::{UiCreator, UiEvent, UiEventType, UiTransform};
/// # use amethyst::{State, StateData, StateEvent, Trans};
///
/// use deathframe::menu::{Menu, UiData};
///
/// type MyGameData = ();
/// type MyStateEvent = ();
///
/// #[derive(Default)]
/// struct ExampleState {
///     ui_data: UiData,
/// }
///
/// impl<MyGameData, MyStateEvent> Menu<MyGameData, MyStateEvent> for ExampleState {
///     fn event_triggered(
///         &mut self,
///         data: &mut StateData<MyGameData>,
///         event_name: String,
///     ) -> Option<Trans<MyGameData, MyStateEvent>> {
///         match event_name.as_ref() {
///             "btn_quit" => Some(Trans::Pop),
///             _ => None,
///         }
///     }
///
///     fn ui_ron_path(&self) -> &str {
///         "resources/my_ui.ron"
///     }
///
///     fn ui_data(&self) -> &UiData {
///         &self.ui_data
///     }
///
///     fn ui_data_mut(&mut self) -> &mut UiData {
///         &mut self.ui_data
///     }
/// }
///```

#[derive(Default)]
pub struct UiData {
    ui_entities:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

pub trait Menu<T, E> {
    /// Returns the path to the UI's RON configuration file.
    fn ui_ron_path(&self) -> &str;

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
    ) -> Option<Trans<T, E>>;

    /// Call this method to create the UI entities, specified in the UI's ron file.
    fn create_ui(&mut self, data: &mut StateData<T>) -> ProgressCounter {
        let mut progress = ProgressCounter::new();

        let menu_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(self.ui_ron_path(), &mut progress)
        });
        self.ui_data_mut().ui_entities.push(menu_entity);

        progress
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
                    if let UiEventType::ClickStop = event.event_type {
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
                            triggered_event = Some(name);
                        }
                    }
                }
            },
        );

        if let Some(event_name) = triggered_event {
            let trans_opt = self.event_triggered(data, event_name);
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
