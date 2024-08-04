use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::EmptyMap;

#[derive(Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DexNav {
    character: HashMap<String, CharDexNav>,
    formula: EmptyMap,
    enemy: EnemyDexNav,
    team_v2: EmptyMap,
}

impl DexNav {
    pub fn set_char(&mut self, char_id: String) {
        let inst_id = char_id.split('_').nth(1).unwrap_or("0").parse().unwrap_or(0);
        let char_dex_nav = CharDexNav::new(inst_id);
        self.character.insert(char_id, char_dex_nav);
    }

    pub fn set_enemy(&mut self, enemy_id: String) {
        self.enemy.set_enemy(enemy_id)
    }
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CharDexNav {
    char_inst_id: u32,
    count: u8,
}

impl CharDexNav {
    pub fn new(id: u32) -> Self {
        Self { char_inst_id: id, count: 6 }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct EnemyDexNav {
    enemies: HashMap<String, u8>,
}

impl EnemyDexNav {
    pub fn set_enemy(&mut self, enemy_id: String) {
        self.enemies.insert(enemy_id, 1);
    }
}