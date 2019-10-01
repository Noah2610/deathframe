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
/// use deathframe::menu::Menu;
///
/// struct ExampleState {
///     ui_entities:  Vec<Entity>,
///     ui_reader_id: Option<ReaderId<UiEvent>>,
///     to_main_menu: bool,
/// }
///
/// impl Menu for ExampleState {
///     fn event_triggered<'a, 'b>(
///         &mut self,
///         event_name: String,
///     ) -> Option<Trans<T, E> {
///         match event_name.as_ref() {
///             "pause_button" => Some(Trans::Pop),
///             "quit_button" => {
///                 self.to_main_menu = true;
///                 Some(Trans::Pop)
///             }
///             _ => None,
///         }
///     }
///
///     fn ui_ron_path(&self) -> &str {
///         UI_RON_PATH
///     }
///
///     fn ui_entities(&self) -> &Vec<Entity> {
///         &self.ui_entities
///     }
///
///     fn ui_entities_mut(&mut self) -> &mut Vec<Entity> {
///         &mut self.ui_entities
///     }
///
///     fn ui_reader_id(&self) -> &Option<ReaderId<UiEvent>> {
///         &self.ui_reader_id
///     }
///
///     fn ui_reader_id_mut(&mut self) -> &mut Option<ReaderId<UiEvent>> {
///         &mut self.ui_reader_id
///     }
/// }
///```

pub trait Menu<T, E> {
    /// Returns the path to the UI's RON configuration file.
    fn ui_ron_path(&self) -> &str;

    /// Returns a reference to the Vec of UI entities.
    fn ui_entities(&self) -> &Vec<Entity>;

    /// Returns a mutable reference to the Vec of UI entities.
    fn ui_entities_mut(&mut self) -> &mut Vec<Entity>;

    /// Returns a reference to an Option of ReaderId.
    fn ui_reader_id(&self) -> &Option<ReaderId<UiEvent>>;

    /// Returns a mutable reference to an Option of ReaderId.
    fn ui_reader_id_mut(&mut self) -> &mut Option<ReaderId<UiEvent>>;

    fn event_triggered<'a, 'b>(
        &mut self,
        data: &mut StateData<T>,
        event_name: String,
    ) -> Option<Trans<T, E>>;

    fn create_ui(&mut self, data: &mut StateData<T>) -> ProgressCounter {
        let mut progress = ProgressCounter::new();

        let menu_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(self.ui_ron_path(), &mut progress)
        });
        self.ui_entities_mut().push(menu_entity);

        progress
    }

    fn delete_ui(&mut self, data: &mut StateData<T>) {
        data.world.delete_entities(self.ui_entities()).unwrap();
        self.ui_entities_mut().clear();
    }

    fn update_ui_events<'a, 'b>(
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
                    .ui_reader_id_mut()
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
