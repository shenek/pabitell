use pabitell_lib::{
    conditions, data, translations::get_available_locales, Character, Description, Event, Id, Item,
    ItemState, Named, Narrator, Scene, World, WorldBuilder,
};
use serde_json::Value;

use crate::{characters, events, world::DollWorld};

#[derive(Default, Debug)]
pub struct Doll;

impl Narrator for Doll {
    fn available_events(&self, world: &dyn World) -> Vec<Box<dyn Event>> {
        let mut res = vec![];
        let doggie = world
            .characters()
            .get("doggie")
            .unwrap()
            .as_any()
            .downcast_ref::<characters::Doggie>()
            .unwrap();

        let kitie = world
            .characters()
            .get("kitie")
            .unwrap()
            .as_any()
            .downcast_ref::<characters::Kitie>()
            .unwrap();

        match (doggie.scene().as_ref(), kitie.scene().as_ref()) {
            (Some(d), Some(k)) if d == "home" && k == "home" => {
                let scene = world.scenes().get("home").unwrap();
                match scene.dialog().unwrap() {
                    0 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "doggie",
                        "home",
                        0,
                    ))) as Box<dyn Event>),
                    1 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "kitie",
                        "home",
                        1,
                    )))),
                    2 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "doggie",
                        "home",
                        2,
                    ))) as Box<dyn Event>),
                    3 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "kitie",
                        "home",
                        3,
                    ))) as Box<dyn Event>),
                    4 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "doggie",
                        "home",
                        4,
                    ))) as Box<dyn Event>),
                    5 => {
                        ["doggie", "kitie"].iter().for_each(|c| {
                            res.push(Box::new(events::make_move(
                                data::MoveData::new("move_to_walk", c, "walk"),
                                c,
                                "home",
                                Some(5),
                                false,
                            )) as Box<dyn Event>);
                        });
                    }
                    6 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "kitie",
                        "home",
                        6,
                    ))) as Box<dyn Event>),
                    7 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "doggie",
                        "home",
                        7,
                    ))) as Box<dyn Event>),
                    8 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "kitie",
                        "home",
                        8,
                    ))) as Box<dyn Event>),
                    9 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "doggie",
                        "home",
                        9,
                    ))) as Box<dyn Event>),
                    10 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "kitie",
                        "home",
                        10,
                    ))) as Box<dyn Event>),
                    11 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "doggie",
                        "home",
                        11,
                    ))) as Box<dyn Event>),
                    12 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "kitie",
                        "home",
                        12,
                    ))) as Box<dyn Event>),
                    13 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "doggie",
                        "home",
                        13,
                    ))) as Box<dyn Event>),
                    14 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_in_home",
                        "kitie",
                        "home",
                        14,
                    ))) as Box<dyn Event>),
                    15 => {
                        res.push(Box::new(events::make_move(
                            data::MoveData::new("move_to_doggie_search", "doggie", "doggie_search"),
                            "doggie",
                            "home",
                            Some(15),
                            false,
                        )) as Box<dyn Event>);
                    }
                    16 => {
                        res.push(Box::new(events::make_move(
                            data::MoveData::new("move_to_kitie_search", "kitie", "kitie_search"),
                            "kitie",
                            "home",
                            Some(16),
                            false,
                        )) as Box<dyn Event>);
                    }
                    17 => {
                        for c in &["doggie", "kitie"] {
                            let items = world
                                .items()
                                .values()
                                .filter(|v| {
                                    v.get_tags().contains(&format!("{}_pick", c))
                                        && v.state() == &ItemState::Owned(c.to_string())
                                })
                                .collect::<Vec<_>>();
                            for item in items {
                                res.push(Box::new(events::make_lay_down(data::UseItemData::new(
                                    "lay_down",
                                    c,
                                    item.name(),
                                ))) as Box<dyn Event>);
                            }
                        }
                    }
                    18 => {} // final dialog
                    _ => unimplemented!(),
                }
            }
            (Some(d), Some(k)) if d == "walk" && k == "home" => {
                if world.items().get("doll").unwrap().state() == &ItemState::Unassigned {
                    // way back
                    res.push(Box::new(events::make_move(
                        data::MoveData::new("move_back_home", "doggie", "home"),
                        "doggie",
                        "walk",
                        Some(7),
                        true,
                    )) as Box<dyn Event>);
                } else {
                    res.push(Box::new(events::make_move(
                        data::MoveData::new("move_to_walk", "kitie", "walk"),
                        "kitie",
                        "home",
                        Some(5),
                        false,
                    )) as Box<dyn Event>);
                }
            }
            (Some(d), Some(k)) if d == "home" && k == "walk" => {
                if world.items().get("doll").unwrap().state() == &ItemState::Unassigned {
                    // way back
                    res.push(Box::new(events::make_move(
                        data::MoveData::new("move_back_home", "kitie", "home"),
                        "kitie",
                        "walk",
                        Some(7),
                        true,
                    )) as Box<dyn Event>);
                } else {
                    res.push(Box::new(events::make_move(
                        data::MoveData::new("move_to_walk", "doggie", "walk"),
                        "doggie",
                        "home",
                        Some(5),
                        false,
                    )) as Box<dyn Event>);
                }
            }
            (Some(d), Some(k)) if d == "walk" && k == "walk" => {
                let scene = world.scenes().get("walk").unwrap();
                match scene.dialog().unwrap() {
                    0 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_on_walk",
                        "doggie",
                        "walk",
                        0,
                    ))) as Box<dyn Event>),
                    1 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_on_walk",
                        "kitie",
                        "walk",
                        1,
                    )))),
                    2 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_on_walk",
                        "doggie",
                        "walk",
                        2,
                    )))),
                    3 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_on_walk",
                        "kitie",
                        "walk",
                        3,
                    )))),
                    4 => {
                        res.push(Box::new(events::make_find_doll(data::UseItemData::new(
                            "found_doll",
                            "kitie",
                            "doll",
                        ))));
                        res.push(Box::new(events::make_find_doll(data::UseItemData::new(
                            "found_doll",
                            "doggie",
                            "doll",
                        ))));
                    }
                    5 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_on_walk",
                        "doggie",
                        "walk",
                        5,
                    )))),
                    6 => res.push(Box::new(events::make_talk(data::TalkData::new(
                        "talk_on_walk",
                        "doggie",
                        "walk",
                        6,
                    )))),
                    7 => {
                        ["doggie", "kitie"].iter().for_each(|c| {
                            res.push(Box::new(events::make_move(
                                data::MoveData::new("move_back_home", c, "home"),
                                c,
                                "walk",
                                Some(7),
                                true,
                            )) as Box<dyn Event>);
                        });
                    }
                    _ => unimplemented!(),
                }
            }
            (Some(d), Some(_)) if d == "doggie_search" => {
                let mut items = world
                    .items()
                    .values()
                    .filter(|v| {
                        v.get_tags().contains(&"doggie_pick".to_owned())
                            && v.state() == &ItemState::InScene("doggie_search".to_string())
                    })
                    .collect::<Vec<_>>();
                if items.is_empty() {
                    res.push(Box::new(events::make_move(
                        data::MoveData::new("move_back_home", "doggie", "home"),
                        "doggie",
                        "doggie_search",
                        None,
                        true,
                    )) as Box<dyn Event>);
                } else {
                    for item in items {
                        res.push(Box::new(events::make_pick(data::PickData::new(
                            "pick",
                            "doggie",
                            item.name(),
                        ))) as Box<dyn Event>);
                    }
                }
            }
            (Some(_), Some(k)) if k == "kitie_search" => {
                let items = world
                    .items()
                    .values()
                    .filter(|v| {
                        v.get_tags().contains(&"kitie_pick".to_owned())
                            && v.state() == &ItemState::InScene("kitie_search".to_string())
                    })
                    .collect::<Vec<_>>();
                if items.is_empty() {
                    res.push(Box::new(events::make_move(
                        data::MoveData::new("move_back_home", "kitie", "home"),
                        "kitie",
                        "kitie_search",
                        None,
                        true,
                    )) as Box<dyn Event>);
                } else {
                    for item in items {
                        res.push(Box::new(events::make_pick(data::PickData::new(
                            "pick",
                            "kitie",
                            item.name(),
                        ))) as Box<dyn Event>);
                    }
                }
            }
            _ => unreachable!(),
        }

        res
    }

    fn parse_event(&self, value: &Value) -> Option<Box<dyn Event>> {
        // TODO validate characters, items, scenes
        match &value["name"] {
            Value::String(name) if name == "talk_in_home" => {
                /*
                if let Value::String(character) = &value["character"] {
                    let data = data::MoveData::new(name, character, "kitchen");
                    Some(Box::new(events::make_move_to_kitchen(data)))
                } else {
                    None
                }
                */
                None
            }
            Value::String(name) if name == "move_to_walk" => {
                /*
                 */
                None
            }
            _ => None,
        }
    }
}
