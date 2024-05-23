use strum::EnumString;

#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, EnumString)]
pub enum SkillAbnormality {
    DemonAmmo = 0x6CC,
    ArmorAmmo = 0x6D0,
    Fortify = 0x764,
    ProtectivePolish = 0x76C,
    AffinitySliding = 0x770,
    ElementAcceleration = 0x730,
    LATENTPOWER = 0x738,
    ADRENALINE = 0x754,
    CoolCat = 0x7C8,
    FROSTCRAFT = 0x788,
    OffensiveGuard = 0x79C,
    Coalescence = 0x7A0,
    EvasionMantle = 0xFC4,
    AffinittyBooster = 0xFC8,
}
