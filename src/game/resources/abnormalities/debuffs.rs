use strum::EnumString;

use crate::utils;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString)]
pub enum Debuff {
    Poison,
    Venom,
    Fire,
    Thunder,
    Water,
    Ice,
    Dragon,
    Bleed,
    Effluvia,
    DefDown,
    ResDown,
    Blast,
    BlastScourge,
}

impl Debuff {
    fn get_timer_offset(&self) -> isize {
        match self {
            Debuff::Poison => 0x5DC,
            Debuff::Venom => 0x5E0,
            Debuff::Fire => 0x5EC,
            Debuff::Thunder => 0x5F0,
            Debuff::Water => 0x5F4,
            Debuff::Ice => 0x5F8,
            Debuff::Dragon => 0x5FC,
            Debuff::Bleed => 0x600,
            Debuff::Effluvia => 0x608,
            Debuff::DefDown => 0x60C,
            Debuff::ResDown => 0x614,
            Debuff::Blast => 0x620,
            Debuff::BlastScourge => 0x63C,
        }
    }

    fn get_category_offset(&self) -> Option<isize> {
        match self {
            Debuff::BlastScourge => Some(0x62C),
            _ => None,
        }
    }
}

pub struct DebuffService {
    instance: usize,
}

impl DebuffService {
    pub fn new(instance: usize) -> Self {
        Self { instance }
    }

    pub fn get_timer(&self, debuff: Debuff) -> f32 {
        let category = self.get_category(debuff);
        if let Some(category) = category {
            if category != debuff {
                return 0.0;
            }
        }
        let timer_offset = debuff.get_timer_offset();
        utils::get_value_with_offset(self.instance as *const f32, &[timer_offset]).unwrap_or(0.0)
    }

    fn get_category(&self, debuff: Debuff) -> Option<Debuff> {
        let category_offset = debuff.get_category_offset()?;
        let category_value =
            utils::get_value_with_offset(self.instance as *const i32, &[category_offset])?;
        Some(match category_offset {
            0x62C => {
                if category_value == 0 {
                    Debuff::Blast
                } else {
                    Debuff::BlastScourge
                }
            }
            _ => panic!("Invalid category offset"),
        })
    }
}
