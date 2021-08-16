use pabitell_lib::{Description, Named, Narrator, World, WorldBuilder};
use skim::prelude::*;

#[cfg(feature = "with_doggie_and_kitie_cake")]
fn make_story_doggie_and_kitie_cake() -> Option<(Box<dyn World>, Box<dyn Narrator>)> {
    let mut world: Box<dyn World> =
        Box::new(doggie_and_kitie_cake::CakeWorldBuilder::make_world().unwrap());
    world.setup();
    let mut narrator: Box<dyn Narrator> =
        Box::new(doggie_and_kitie_cake::narrator::Cake::default());

    Some((world, narrator))
}

#[cfg(not(feature = "with_doggie_and_kitie_cake"))]
fn make_story_doggie_and_kitie_cake() -> Option<(Box<dyn World>, Box<dyn Narrator>)> {
    None
}

#[derive(Clone)]
struct PabitellItem {
    code: String,
    short: String,
    long: String,
}

impl SkimItem for PabitellItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.code)
    }

    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::new_string(self.short.clone(), vec![])
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(self.long.clone())
    }
}

fn select_story(lang: &str) -> Option<PabitellItem> {
    let mut stories = vec![];
    if cfg!(feature = "with_doggie_and_kitie_cake") {
        let (mut world, _) = make_story_doggie_and_kitie_cake().unwrap();
        world.set_lang(lang);
        let description = world.description();
        stories.push(PabitellItem {
            code: "doggie_and_kitie_cake".into(),
            short: description.short(world.as_ref()),
            long: description.long(world.as_ref()),
        });
    }
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for story in stories.into_iter() {
        let _ = tx_item.send(Arc::new(story));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    if selected_items.is_empty() {
        None
    } else {
        Some(
            (*selected_items[0])
                .as_any()
                .downcast_ref::<PabitellItem>()?
                .clone(),
        )
    }
}

fn select_language(available_languages: Vec<String>) -> Option<String> {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for lang in available_languages.into_iter() {
        let _ = tx_item.send(Arc::new(lang));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        (*selected_items[0])
            .as_any()
            .downcast_ref::<String>()?
            .to_string(),
    )
}

#[derive(Clone, Copy)]
enum View {
    MENU,
    ITEMS,
    CHARACTERS,
    SCENES,
    EVENTS,
    EXIT,
}

impl SkimItem for View {
    fn text(&self) -> Cow<str> {
        match self {
            Self::MENU => Cow::Borrowed("menu"),
            Self::ITEMS => Cow::Borrowed("items"),
            Self::CHARACTERS => Cow::Borrowed("characters"),
            Self::SCENES => Cow::Borrowed("scenes"),
            Self::EVENTS => Cow::Borrowed("events"),
            Self::EXIT => Cow::Borrowed("exit"),
        }
    }
}

fn main_menu(world: &dyn World) -> Option<View> {
    println!("{}", world.description().short(world));

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in [
        View::ITEMS,
        View::CHARACTERS,
        View::SCENES,
        View::EVENTS,
        View::EXIT,
    ] {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    if selected_items.is_empty() {
        None
    } else {
        Some(
            (*selected_items[0])
                .as_any()
                .downcast_ref::<View>()?
                .clone(),
        )
    }
}

fn select_characters(world: &dyn World) -> Option<Vec<PabitellItem>> {
    let characters: Vec<PabitellItem> = world
        .characters()
        .values()
        .map(|e| PabitellItem {
            code: e.name().to_string(),
            short: e.short(world).to_string(),
            long: e.long(world).to_string(),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for character in characters.into_iter() {
        let _ = tx_item.send(Arc::new(character));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| {
                (*e).as_any()
                    .downcast_ref::<PabitellItem>()
                    .unwrap()
                    .clone()
            })
            .collect(),
    )
}

fn select_scenes(world: &dyn World) -> Option<Vec<PabitellItem>> {
    let scenes: Vec<PabitellItem> = world
        .scenes()
        .values()
        .map(|e| PabitellItem {
            code: e.name().to_string(),
            short: e.short(world).to_string(),
            long: e.long(world).to_string(),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .multi(true) // TODO how does multi work...
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for scene in scenes.into_iter() {
        let _ = tx_item.send(Arc::new(scene));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| {
                (*e).as_any()
                    .downcast_ref::<PabitellItem>()
                    .unwrap()
                    .clone()
            })
            .collect(),
    )
}

fn select_items(world: &dyn World) -> Option<Vec<PabitellItem>> {
    let items: Vec<PabitellItem> = world
        .items()
        .values()
        .map(|e| PabitellItem {
            code: e.name().to_string(),
            short: e.short(world).to_string(),
            long: e.long(world).to_string(),
        })
        .collect();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .multi(true) // TODO how does multi work...
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for item in items.into_iter() {
        let _ = tx_item.send(Arc::new(item));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| {
                (*e).as_any()
                    .downcast_ref::<PabitellItem>()
                    .unwrap()
                    .clone()
            })
            .collect(),
    )
}

#[derive(Clone)]
struct EventItem {
    idx: usize,
    short: String,
    long: String,
}

impl SkimItem for EventItem {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(&self.short)
    }

    fn display<'a>(&'a self, context: DisplayContext<'a>) -> AnsiString<'a> {
        AnsiString::new_string(self.short.clone(), vec![])
    }
    fn preview(&self, _context: PreviewContext) -> ItemPreview {
        ItemPreview::AnsiText(self.long.clone())
    }
}

fn select_event(world: &dyn World, narrator: &dyn Narrator) -> Option<Vec<EventItem>> {
    let events = narrator
        .available_events(world)
        .iter()
        .enumerate()
        .map(|(idx, e)| EventItem {
            idx,
            short: e.short(world),
            long: e.long(world),
        })
        .collect::<Vec<EventItem>>();

    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .preview(Some(""))
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for event in events.into_iter() {
        let _ = tx_item.send(Arc::new(event));
    }
    drop(tx_item); // so that skim could know when to stop waiting for more items.

    let selected_items = Skim::run_with(&options, Some(rx_item)).map(|out| out.selected_items)?;
    Some(
        selected_items
            .into_iter()
            .map(|e| (*e).as_any().downcast_ref::<EventItem>().unwrap().clone())
            .collect(),
    )
}

pub fn main() {
    let story = select_story("en-US").unwrap();
    println!("story: {}", story.short);
    let (mut world, narrator) = match story.code.as_str() {
        "doggie_and_kitie_cake" => make_story_doggie_and_kitie_cake().unwrap(),
        _ => unreachable!(),
    };
    let lang = select_language(
        world
            .available_languages()
            .iter()
            .map(|e| e.to_string())
            .collect(),
    )
    .unwrap();
    println!("lang: {}", lang);

    let mut state = View::MENU;
    let mut selected_characters: Vec<PabitellItem> = vec![];
    let mut selected_items: Vec<PabitellItem> = vec![];
    let mut selected_scenes: Vec<PabitellItem> = vec![];
    loop {
        match state {
            View::MENU => match main_menu(world.as_ref()) {
                Some(View::ITEMS) => state = View::ITEMS,
                Some(View::CHARACTERS) => state = View::CHARACTERS,
                Some(View::SCENES) => state = View::SCENES,
                Some(View::EVENTS) => state = View::EVENTS,
                Some(View::EXIT) => break,
                _ => break,
            },
            View::CHARACTERS => {
                if let Some(characters) = select_characters(world.as_ref()) {
                    selected_characters = characters;
                }
                state = View::MENU;
            }
            View::SCENES => {
                if let Some(scenes) = select_scenes(world.as_ref()) {
                    selected_scenes = scenes;
                }
                state = View::MENU;
            }
            View::ITEMS => {
                if let Some(items) = select_items(world.as_ref()) {
                    selected_items = items;
                }
                state = View::MENU;
            }
            View::EVENTS => {
                if !selected_characters.is_empty() {
                    if let Some(scene) = world
                        .characters()
                        .get(&selected_characters[0].code)
                        .unwrap()
                        .scene()
                    {
                        let scene = world.scenes().get(scene).unwrap();
                        println!("\n{}\n\n", scene.long(world.as_ref()));
                    }
                }
                if let Some(events) = select_event(world.as_mut(), narrator.as_ref()) {
                    if !events.is_empty() {
                        let idx = events[0].idx;
                        let mut events = narrator.available_events(world.as_ref());
                        println!("{}", events[idx].long(world.as_ref()));
                        events[idx].trigger(world.as_mut());
                        continue;
                    }
                }
                state = View::MENU;
            }
            View::EXIT => break,
        }
        println!(
            "Selected Character: {}",
            selected_characters
                .iter()
                .map(|e| e.short.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "Selected Scenes: {}",
            selected_scenes
                .iter()
                .map(|e| e.short.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        println!(
            "Selected Items: {}",
            selected_items
                .iter()
                .map(|e| e.short.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
    }
}
