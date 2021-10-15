use anyhow::{anyhow, Result};
use pabitell_lib::{AsAny, Description, Dumpable, Id, Item, ItemState, Named, Tagged, World};
use serde_json::{json, Value};
use std::any::Any;
use uuid::Uuid;

use crate::translations::get_message;

macro_rules! simple_item {
    ($class_name: ident, $name: literal, [$( $tag:expr ),* ]) => {
        #[derive(Debug, Default)]
        pub struct $class_name {
            id: Uuid,
            state: ItemState,
        }

        impl Id for $class_name {
            fn id(&self) -> &Uuid {
                &self.id
            }

            fn set_id(&mut self, id: Uuid) {
                self.id = id;
            }
        }

        impl Named for $class_name {
            fn name(&self) -> &'static str {
                $name
            }
        }

        impl Tagged for $class_name {
            fn get_tags(&self) -> Vec<String> {
                #[allow(unused_mut)]
                let mut res: Vec<String> = vec![];
                $(
                    res.push($tag.into());
                )*
                res
            }
        }

        impl Description for $class_name {
            fn long(&self, world: &dyn World) -> String {
                get_message(
                    &format!("{}-{}-long", world.name(), $name),
                    world.lang(),
                    None,
                )
            }

            fn short(&self, world: &dyn World) -> String {
                get_message(
                    &format!("{}-{}-short", world.name(), $name),
                    world.lang(),
                    None,
                )
            }
        }

        impl AsAny for $class_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }

        impl Item for $class_name {
            fn state(&self) -> &ItemState {
                &self.state
            }

            fn set_state(&mut self, state: ItemState) {
                self.state = state;
            }
        }

        impl Dumpable for $class_name {
            fn dump(&self) -> Value {
                json!(
                    {"state": self.state.dump(), "name": self.name()}
                )
            }
            fn load(&mut self, data: Value) -> Result<()> {
                if let Value::Object(mut object) = data {
                    let state_json = object.remove("state").ok_or_else(|| anyhow!("Wrong format of item '{}'", self.name()))?;
                    self.state.load(state_json)?;
                    Ok(())
                } else{
                    Err(anyhow!("Wrong format of item '{}'", self.name()))
                }
            }
        }

    };
}

simple_item!(SandCake, "sand_cake", []);

simple_item!(Flour, "flour", ["ingredient", "accepted", "batch1"]);
simple_item!(Milk, "milk", ["ingredient", "accepted", "batch1"]);
simple_item!(Egg, "egg", ["ingredient", "accepted", "batch1"]);

simple_item!(Suggar, "suggar", ["ingredient", "accepted", "batch2"]);
simple_item!(Salt, "salt", ["ingredient", "accepted", "batch2"]);
simple_item!(Butter, "butter", ["ingredient", "accepted", "batch2"]);
simple_item!(Jam, "jam", ["ingredient", "rejected", "batch2"]);
simple_item!(Cheese, "cheese", ["ingredient", "accepted", "batch2"]);

simple_item!(Bacon, "bacon", ["ingredient", "accepted", "batch3"]);
simple_item!(Peanuts, "peanuts", ["ingredient", "accepted", "batch3"]);
simple_item!(Cucumber, "cucumber", ["ingredient", "accepted", "batch3"]);
simple_item!(Bones, "bones", ["ingredient", "accepted", "batch3"]);

simple_item!(FourMice, "four_mice", ["ingredient", "accepted", "batch4"]);
simple_item!(Sausages, "sausages", ["ingredient", "accepted", "batch4"]);
simple_item!(
    WhippedCream,
    "whipped_cream",
    ["ingredient", "accepted", "batch4"]
);
simple_item!(Onion, "onion", ["ingredient", "accepted", "batch4"]);
simple_item!(Chocolate, "chocolate", ["ingredient", "accepted", "batch4"]);
simple_item!(Sauce, "sauce", ["ingredient", "accepted", "batch4"]);

simple_item!(Garlic, "garlic", ["ingredient", "accepted", "batch5"]);
simple_item!(Pepper, "pepper", ["ingredient", "accepted", "batch5"]);
simple_item!(Lard, "lard", ["ingredient", "accepted", "batch5"]);
simple_item!(Candy, "candy", ["ingredient", "accepted", "batch5"]);
simple_item!(Greaves, "greaves", ["ingredient", "accepted", "batch5"]);
simple_item!(Cinnamon, "cinnamon", ["ingredient", "accepted", "batch5"]);
simple_item!(Porridge, "porridge", ["ingredient", "accepted", "batch5"]);
simple_item!(
    CottageCheese,
    "cottage_cheese",
    ["ingredient", "accepted", "batch5"]
);

simple_item!(
    GingerBread,
    "ginger_bread",
    ["ingredient", "accepted", "batch6"]
);
simple_item!(Vinegar, "vinegar", ["ingredient", "accepted", "batch6"]);
simple_item!(Cocoa, "cocoa", ["ingredient", "accepted", "batch6"]);
simple_item!(Cabbadge, "cabbadge", ["ingredient", "accepted", "batch6"]);
simple_item!(
    GooseHead,
    "goose_head",
    ["ingredient", "accepted", "batch6"]
);
simple_item!(Raisins, "raisins", ["ingredient", "accepted", "batch6"]);
simple_item!(Bread, "bread", ["ingredient", "rejected", "batch6"]);

simple_item!(Marbles, "marbles", ["toy"]);
simple_item!(Ball, "ball", ["toy"]);
simple_item!(Dice, "dice", ["toy"]);

simple_item!(BadDog, "bad_dog", ["animal"]);

simple_item!(Soup, "soup", ["meal"]);
simple_item!(Meat, "meat", ["meal"]);
simple_item!(Pie, "pie", ["meal"]);
simple_item!(Dumplings, "dumplings", ["meal"]);
