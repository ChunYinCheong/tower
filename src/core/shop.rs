use serde::{Deserialize, Serialize};

use super::{CharacterItem, Game};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShopSystem {
    pub current: Option<CurrentShop>,
}

impl ShopSystem {
    pub fn close_shop(game: &mut Game) {
        game.shop_system.current = None;
    }

    pub fn buy_item(game: &mut Game, index: usize) {
        if let Some(current) = &mut game.shop_system.current {
            let id = game
                .characters
                .get(&current.shop_character_id)
                .and_then(|c| c.shop.as_ref())
                .map(|shop| &shop.items)
                .and_then(|items| items.get(index))
                .map(|si| si.item_id);
            if let Some(id) = id {
                if let Some(c) = game.characters.get_mut(&current.character_id) {
                    c.items.push(CharacterItem {
                        item_id: id,
                        quantity: 1,
                    });
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentShop {
    pub character_id: i32,
    pub shop_character_id: i32,
}
