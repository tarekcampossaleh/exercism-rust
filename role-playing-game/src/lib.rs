// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
use std::cmp::min;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                mana: if self.level < 10 { None } else { Some(100) },
                level: self.level,
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(x) => {
                if x >= mana_cost {
                    self.mana = Some(x - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                self.health -= min(self.health, mana_cost);
                0
            }
        }
    }
}
