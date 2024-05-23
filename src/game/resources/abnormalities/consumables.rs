use strum::EnumString;

use crate::util;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString)]
pub enum Consumable {
    DashJuice,
    WigglyLitchy,
    AsteraJerky,
    MightSeed,
    MightPill,
    AdamantSeed,
    AdamantPill,
    DemonPowder,
    HardshellPowder,
    Demondrug,
    MegaDemondrug,
    Armorskin,
    MegaArmorskin,
    Cooldrink,
    Hotdrink,
    HealthRegen,
    ColdRes,
    SnowmanHead,
    Powercone,
    IceRes,
}

impl Consumable {
    fn get_timer_offset(&self) -> isize {
        match self {
            Consumable::DashJuice => 0x690,
            Consumable::WigglyLitchy => 0x694,
            Consumable::AsteraJerky => 0x698,
            Consumable::MightSeed => 0x6A0,
            Consumable::MightPill => 0x6A0,
            Consumable::AdamantSeed => 0x6B0,
            Consumable::AdamantPill => 0x6B0,
            Consumable::DemonPowder => 0x6C4,
            Consumable::HardshellPowder => 0x6C8,
            Consumable::Demondrug => 0x6D4,
            Consumable::MegaDemondrug => 0x6D4,
            Consumable::Armorskin => 0x6D0,
            Consumable::MegaArmorskin => 0x6D0,
            Consumable::Cooldrink => 0x6EC,
            Consumable::Hotdrink => 0x6F0,
            Consumable::HealthRegen => 0x6F8,
            Consumable::ColdRes => 0x6FC,
            Consumable::SnowmanHead => 0x708,
            Consumable::Powercone => 0x718,
            Consumable::IceRes => 0x71C,
        }
    }

    fn get_category_offset(&self) -> Option<isize> {
        match self {
            Consumable::MightSeed => Some(0x6A4),
            Consumable::MightPill => Some(0x6A4),
            Consumable::AdamantSeed => Some(0x6B4),
            Consumable::AdamantPill => Some(0x6B4),
            Consumable::Demondrug => Some(0x6D4),
            Consumable::MegaDemondrug => Some(0x6D4),
            Consumable::Armorskin => Some(0x6D8),
            Consumable::MegaArmorskin => Some(0x6D8),
            _ => None,
        }
    }

    fn is_infinite(&self) -> bool {
        matches!(
            self,
            Consumable::Demondrug
                | Consumable::MegaDemondrug
                | Consumable::Armorskin
                | Consumable::MegaArmorskin
        )
    }
}

pub struct ConsumableService {
    instance: usize,
}

impl ConsumableService {
    pub fn new(instance: usize) -> Self {
        Self { instance }
    }

    pub fn get_timer(&self, consumable: Consumable) -> f32 {
        let category = self.get_category(consumable);
        if let Some(category) = category {
            if category != consumable {
                return 0.0;
            }
        }
        if consumable.is_infinite() {
            // 如果是无限时间的Buff，则不需要获取计时器，只判断类别
            return f32::MAX;
        }
        let timer_offset = consumable.get_timer_offset();
        util::get_value_with_offset(self.instance as *const f32, &[timer_offset]).unwrap_or(0.0)
    }

    fn get_category(&self, consumable: Consumable) -> Option<Consumable> {
        let category_offset = consumable.get_category_offset()?;
        let category_value =
            util::get_value_with_offset(self.instance as *const i32, &[category_offset])?;
        Some(match category_offset {
            0x6A4 => match category_value {
                10 => Consumable::MightSeed,
                25 => Consumable::MightPill,
                _ => Consumable::MightSeed,
            },
            0x6B4 => match category_value {
                20 => Consumable::AdamantSeed,
                1 => Consumable::AdamantPill,
                _ => Consumable::AdamantSeed,
            },
            0x6D4 => match category_value {
                1 => Consumable::Demondrug,
                2 => Consumable::MegaDemondrug,
                _ => Consumable::Demondrug,
            },
            0x6D8 => match category_value {
                1 => Consumable::Armorskin,
                2 => Consumable::MegaArmorskin,
                _ => Consumable::Armorskin,
            },
            _ => panic!("Invalid category offset"),
        })
    }
}
