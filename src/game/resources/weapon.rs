#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WeaponType {
    /// 大剑
    GreatSowrd = 0,
    /// 片手剑
    SwordAndShield = 1,
    /// 双刀
    DualBlades = 2,
    /// 太刀
    LongSword = 3,
    /// 大锤
    Hammer = 4,
    /// 狩猎笛
    HuntingHorn = 5,
    /// 长枪
    Lance = 6,
    /// 铳枪
    Gunlance = 7,
    /// 斩斧
    SwitchAxe = 8,
    /// 盾斧
    ChargeBlade = 9,
    /// 操虫棍
    InsectGlaive = 10,
    /// 弓
    Bow = 11,
    /// 重弩炮
    HeavyBowgun = 12,
    /// 轻弩炮
    LightBowgun = 13,
}

impl PartialEq<i32> for WeaponType {
    fn eq(&self, other: &i32) -> bool {
        match self {
            WeaponType::GreatSowrd => *other == 0,
            WeaponType::SwordAndShield => *other == 1,
            WeaponType::DualBlades => *other == 2,
            WeaponType::LongSword => *other == 3,
            WeaponType::Hammer => *other == 4,
            WeaponType::HuntingHorn => *other == 5,
            WeaponType::Lance => *other == 6,
            WeaponType::Gunlance => *other == 7,
            WeaponType::SwitchAxe => *other == 8,
            WeaponType::ChargeBlade => *other == 9,
            WeaponType::InsectGlaive => *other == 10,
            WeaponType::Bow => *other == 11,
            WeaponType::HeavyBowgun => *other == 12,
            WeaponType::LightBowgun => *other == 13,
        }
    }
}

impl WeaponType {
    pub fn from_i32(id: i32) -> Option<Self> {
        match id {
            0 => Some(WeaponType::GreatSowrd),
            1 => Some(WeaponType::SwordAndShield),
            2 => Some(WeaponType::DualBlades),
            3 => Some(WeaponType::LongSword),
            4 => Some(WeaponType::Hammer),
            5 => Some(WeaponType::HuntingHorn),
            6 => Some(WeaponType::Lance),
            7 => Some(WeaponType::Gunlance),
            8 => Some(WeaponType::SwitchAxe),
            9 => Some(WeaponType::ChargeBlade),
            10 => Some(WeaponType::InsectGlaive),
            11 => Some(WeaponType::Bow),
            12 => Some(WeaponType::HeavyBowgun),
            13 => Some(WeaponType::LightBowgun),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> i32 {
        *self as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weapon_type() {
        let longsword_id: i32 = 3;
        assert_eq!(WeaponType::LongSword, longsword_id);
        assert_eq!(WeaponType::LongSword.as_i32(), 3);
        assert_eq!(WeaponType::from_i32(3), Some(WeaponType::LongSword));
        assert_eq!(WeaponType::from_i32(14), None);
    }
}
