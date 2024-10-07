use crate::game::mt_types::{Model, MtObject, Resource};

use super::Entity;

#[derive(Clone)]
pub struct Monster {
    instance: usize,
}

impl MtObject for Monster {
    fn get_instance(&self) -> usize {
        self.instance
    }

    fn from_instance(ptr: usize) -> Self {
        Self { instance: ptr }
    }
}

impl Model for Monster {}

impl Entity for Monster {}

impl Monster {
    pub fn monster_type(&self) -> MonsterType {
        self.get_value_copy(0x12280)
    }

    pub fn variant(&self) -> u32 {
        self.get_value_copy(0x12288)
    }

    pub fn health(&self) -> &'static f32 {
        let ptr = self.get_value_copy::<usize>(0x7670) + 0x64;
        unsafe { (ptr as *const f32).as_ref().unwrap() }
    }

    pub fn max_health(&self) -> &'static f32 {
        let ptr = self.get_value_copy::<usize>(0x7670) + 0x60;
        unsafe { (ptr as *const f32).as_ref().unwrap() }
    }

    pub fn speed(&self) -> &'static f32 {
        self.get_value_ref(0x1D8A8)
    }

    pub fn set_speed(&self, speed: f32) {
        let val = self.get_value_mut(0x1D8A8);
        *val = speed;
    }

    pub fn ai_data(&self) -> usize {
        self.get_value_copy(0x12278)
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MonsterType {
    Anjanath = 0x00,
    Rathalos = 0x01,
    Aptonoth = 0x02,
    Jagras = 0x03,
    ZorahMagdaros = 0x04,
    Mosswine = 0x05,
    Gajau = 0x06,
    GreatJagras = 0x07,
    KestodonM = 0x08,
    Rathian = 0x09,
    PinkRathian = 0x0A,
    AzureRathalos = 0x0B,
    Diablos = 0x0C,
    BlackDiablos = 0x0D,
    Kirin = 0x0E,
    Behemoth = 0x0F,
    KushalaDaora = 0x10,
    Lunastra = 0x11,
    Teostra = 0x12,
    Lavasioth = 0x13,
    Deviljho = 0x14,
    Barroth = 0x15,
    Uragaan = 0x16,
    Leshen = 0x17,
    Pukei = 0x18,
    Nergigante = 0x19,
    XenoJiiva = 0x1A,
    KuluYaKu = 0x1B,
    TzitziYaKu = 0x1C,
    Jyuratodus = 0x1D,
    TobiKadachi = 0x1E,
    Paolumu = 0x1F,
    Legiana = 0x20,
    GreatGirros = 0x21,
    Odogaron = 0x22,
    Radobaan = 0x23,
    VaalHazak = 0x24,
    Dodogama = 0x25,
    KulveTaroth = 0x26,
    Bazelgeuse = 0x27,
    Apceros = 0x28,
    KelbiM = 0x29,
    KelbiF = 0x2A,
    Hornetaur = 0x2B,
    Vespoid = 0x2C,
    Mernos = 0x2D,
    KestodonF = 0x2E,
    Raphinos = 0x2F,
    Shamos = 0x30,
    Barnos = 0x31,
    Girros = 0x32,
    AncientLeshen = 0x33,
    Gastodon = 0x34,
    Noios = 0x35,
    Magmacore = 0x36,
    Magmacore2 = 0x37,
    Gajalaka = 0x38,
    SmallBarrel = 0x39,
    LargeBarrel = 0x3A,
    TrainingPole = 0x3B,
    TrainingWagon = 0x3C,
    Tigrex = 0x3D,
    Nargacuga = 0x3E,
    Barioth = 0x3F,
    SavageDeviljho = 0x40,
    Brachydios = 0x41,
    Glavenus = 0x42,
    AcidicGlavenus = 0x43,
    FulgurAnjanath = 0x44,
    CoralPukei = 0x45,
    RuinerNergigante = 0x46,
    ViperTobi = 0x47,
    NightshadePaolumu = 0x48,
    ShriekingLegiana = 0x49,
    EbonyOdogaron = 0x4A,
    BlackveilVaal = 0x4B,
    SeethingBazelgeuse = 0x4C,
    Beotodus = 0x4D,
    Banbaro = 0x4E,
    Velkhana = 0x4F,
    Namielle = 0x50,
    Shara = 0x51,
    Popo = 0x52,
    Anteka = 0x53,
    Wulg = 0x54,
    Cortos = 0x55,
    Boaboa = 0x56,
    Alatreon = 0x57,
    GoldRathian = 0x58,
    SilverRathalos = 0x59,
    YianGaruga = 0x5A,
    Rajang = 0x5B,
    FuriousRajang = 0x5C,
    BruteTigrex = 0x5D,
    Zinogre = 0x5E,
    StygianZinogre = 0x5F,
    RagingBrachy = 0x60,
    SafiJiiva = 0x61,
    Unavaliable = 0x62,
    ScarredYianGaruga = 0x63,
    FrostfangBarioth = 0x64,
    Fatalis = 0x65,
}
