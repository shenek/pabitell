use pabitell_lib::{Description, Narrator, World, WorldBuilder};
use std::sync::Arc;
use yew::prelude::*;

use crate::{narrator, translations, world::CakeWorld, world::CakeWorldBuilder};

use super::{actions::Actions, character_combo::CharacterCombo};

pub enum Msg {
    ToggleNavbar,
    UpdateSelectedCharacter(Option<String>),
    TriggerEvent(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Void,
    QR,
    Use,
    Give,
    Hint,
}

pub struct App {
    world: CakeWorld,
    selected_character: Option<String>,
    page: Page,
    navbar_active: bool,
}

#[derive(Clone, Debug, PartialEq, Default, Properties)]
pub struct Props {}

impl Component for App {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        log::info!("Creating new world");
        let mut world = CakeWorldBuilder::make_world().unwrap();
        world.setup();
        world.set_lang("cs");

        Self {
            world: world,
            selected_character: None,
            page: Page::Void,
            navbar_active: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateSelectedCharacter(selected_character) => {
                self.selected_character = selected_character;
            }
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
            }
            Msg::TriggerEvent(idx) => {
                let narrator = narrator::Cake::default();
                let mut events = narrator.available_events(&self.world);
                let event = &mut events[idx];
                event.trigger(&mut self.world);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let narrator = narrator::Cake::default();
        let events = narrator.available_events(&self.world);

        let link = ctx.link();
        let trigger_event_callback = link.callback(|idx| Msg::TriggerEvent(idx));

        html! {
            <>
                <section class="hero is-small is-light">
                  <div class="hero-body">
                        <p class="title">
                            {self.world.description().short(&self.world)}
                        </p>
                    </div>
                </section>
                { self.view_nav(ctx) }
                <main>
                    { self.view_scene(ctx) }
                    <Actions
                      events={ events.iter().enumerate().map(|(idx, e)| (idx, e.action_text(&self.world))).collect::<Vec<(usize, String)>>() }
                      trigger_event={ trigger_event_callback }
                    />
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        <a href="https://github.com/shenek/pabitell/"> { "Pabitell" }</a>
                    </div>
                </footer>
            </>
        }
    }
}

impl App {
    fn view_scene(&self, ctx: &Context<Self>) -> Html {
        if let Some(character) = self.selected_character.as_ref() {
            let character = self.world.characters().get(character).unwrap();
            let scene_name = character.scene().as_ref().unwrap();
            let scene = self.world.scenes().get(scene_name).unwrap();
            html! {
                <section class="section">
                    <h1 class="title">{ scene.short(&self.world) }</h1>
                    <p class="subtitle">{ scene.long(&self.world) }</p>
                </section>
            }
        } else {
            html! {
                <section class="section">
                </section>
            }
        }
    }

    fn view_nav(&self, ctx: &Context<Self>) -> Html {
        let Self { world, .. } = self;
        let link = ctx.link();

        let mut available_characters = vec![(
            None,
            translations::get_message("narrator", world.lang(), None),
        )];
        let set_character_callback = link
            .callback(|(selected_character, _)| Msg::UpdateSelectedCharacter(selected_character));
        world.characters().iter().for_each(|(key, character)| {
            available_characters.push((Some(key.to_string()), character.short(world)))
        });

        let active_class = if self.navbar_active { "is-active" } else { "" };

        let use_text = translations::get_message("use", world.lang(), None);
        let hint_text = translations::get_message("hint", world.lang(), None);
        let qr_code_text = translations::get_message("qr_code", world.lang(), None);
        let give_text = translations::get_message("give", world.lang(), None);

        html! {
            <nav class="navbar is-dark" role="navigation" aria-label="main navigation">
              <div class="navbar-brand">
                <a class="navbar-item" href="">
                </a>

                <a
                  role="button"
                  class={classes!("navbar-burger", "burger", active_class)}
                  aria-label="menu"
                  aria-expanded="false"
                  data-target="pabitell-navbar"
                  onclick={link.callback(|_| Msg::ToggleNavbar)}
                >
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                  <span aria-hidden="true"></span>
                </a>
              </div>

              <div id="pabitell-navbar" class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                  <a class="navbar-item">{ qr_code_text }</a>
                  <a class="navbar-item">{ use_text }</a>
                  <a class="navbar-item">{ give_text }</a>
                  <a class="navbar-item">{ hint_text }</a>
                </div>

                <div class="navbar-end">
                  <div class="navbar-item">
                    <CharacterCombo
                      available_characters={ available_characters }
                      set_character={ set_character_callback }
                    />
                  </div>
                </div>
              </div>
            </nav>
        }
    }
}
