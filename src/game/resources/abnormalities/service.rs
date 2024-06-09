use std::str::FromStr;

use crate::utils;

use super::{
    consumables::ConsumableService, debuffs::DebuffService, Consumable, Debuff, HUEAbnormality,
    PalicoAbnormality, SkillAbnormality,
};

const BASE_ADDRESS: *const usize = 0x145011760 as *const usize;
const OFFSETS: &[isize] = &[0x50, 0x7D20];

pub struct AbnormalityService {
    instance: usize,
    consumable_service: ConsumableService,
    debuff_service: DebuffService,
}

impl AbnormalityService {
    pub fn new() -> Option<Self> {
        let instance = utils::get_ptr_with_offset(BASE_ADDRESS, OFFSETS).map(|ptr| ptr as usize)?;
        Some(Self {
            instance,
            consumable_service: ConsumableService::new(instance),
            debuff_service: DebuffService::new(instance),
        })
    }

    /// 获取Buff/Debuff的持续时间
    pub fn get_timer(&self, abnormality: Abnormality) -> f32 {
        match abnormality {
            Abnormality::HUE(hue_abnormality) => self.get_hue_timer(hue_abnormality),
            Abnormality::Palico(palico_abnormality) => self.get_palico_timer(palico_abnormality),
            Abnormality::Skill(skill_abnormality) => self.get_skill_timer(skill_abnormality),
            Abnormality::Consumable(consumable) => self.get_consumable_timer(consumable),
            Abnormality::Debuff(debuff) => self.get_debuff_timer(debuff),
        }
    }

    /// 获取狩猎笛Buff的持续时间
    fn get_hue_timer(&self, hue_abnormality: HUEAbnormality) -> f32 {
        let offset = hue_abnormality as isize;
        utils::get_value_with_offset(self.instance as *const f32, &[offset]).unwrap_or(0.0)
    }

    /// 获取猫笛Buff的持续时间
    fn get_palico_timer(&self, palico_abnormality: PalicoAbnormality) -> f32 {
        let offset = palico_abnormality as isize;
        utils::get_value_with_offset(self.instance as *const f32, &[offset]).unwrap_or(0.0)
    }

    /// 获取技能Buff的持续时间
    fn get_skill_timer(&self, skill_abnormality: SkillAbnormality) -> f32 {
        let offset = skill_abnormality as isize;
        utils::get_value_with_offset(self.instance as *const f32, &[offset]).unwrap_or(0.0)
    }

    /// 获取消耗品Buff的持续时间
    fn get_consumable_timer(&self, consumable: Consumable) -> f32 {
        self.consumable_service.get_timer(consumable)
    }

    /// 获取Debuff的持续时间（或计数器等其他计量单位）
    ///
    /// 例如爆炸异常，翻滚会减值，静止会缓慢增加。到达一定值后，会触发爆炸。
    fn get_debuff_timer(&self, debuff: Debuff) -> f32 {
        self.debuff_service.get_timer(debuff)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Abnormality {
    HUE(HUEAbnormality),
    Palico(PalicoAbnormality),
    Skill(SkillAbnormality),
    Consumable(Consumable),
    Debuff(Debuff),
}

impl std::str::FromStr for Abnormality {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_str_inner(s).ok_or("Invalid Abnormality".to_string())
    }
}

impl Abnormality {
    fn from_str_inner(s: &str) -> Option<Self> {
        if s.starts_with("HUE::") {
            Some(Abnormality::HUE(
                HUEAbnormality::from_str(s.strip_prefix("HUE::")?).ok()?,
            ))
        } else if s.starts_with("Palico::") {
            Some(Abnormality::Palico(
                PalicoAbnormality::from_str(s.strip_prefix("Palico::")?).ok()?,
            ))
        } else if s.starts_with("Skill::") {
            Some(Abnormality::Skill(
                SkillAbnormality::from_str(s.strip_prefix("Skill::")?).ok()?,
            ))
        } else if s.starts_with("Consumable::") {
            Some(Abnormality::Consumable(
                Consumable::from_str(s.strip_prefix("Consumable::")?).ok()?,
            ))
        } else if s.starts_with("Debuff::") {
            Some(Abnormality::Debuff(
                Debuff::from_str(s.strip_prefix("Debuff::")?).ok()?,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abnormality_from_str() {
        assert_eq!(
            Abnormality::from_str("HUE::SelfImprovement"),
            Ok(Abnormality::HUE(HUEAbnormality::SelfImprovement))
        );
        assert_eq!(
            Abnormality::from_str("HUE::SelfImprovement"),
            Ok(Abnormality::HUE(HUEAbnormality::SelfImprovement))
        );
        assert_eq!(
            Abnormality::from_str("Consumable::DashJuice"),
            Ok(Abnormality::Consumable(Consumable::DashJuice))
        );
    }
}
