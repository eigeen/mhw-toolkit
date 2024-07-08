use address_scanner::AddressProvider;
use once_cell::sync::Lazy;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type SharedAddressRepository = Arc<Mutex<AddressRepository>>;

static ADDRESS_REPOSITORY: Lazy<SharedAddressRepository> =
    Lazy::new(|| Arc::new(Mutex::new(AddressRepository::new())));

pub struct AddressRepository {
    cache: HashMap<TypeId, usize>,
}

impl AddressRepository {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get_instance() -> SharedAddressRepository {
        ADDRESS_REPOSITORY.clone()
    }

    pub fn get_address(&mut self, provider: impl AddressProvider) -> Result<usize, String> {
        if let Some(addr) = self.cache.get(&provider.type_id()) {
            return Ok(*addr);
        }

        match provider.get_address() {
            Ok(addr) => {
                self.cache.insert(provider.type_id(), addr);
                Ok(addr)
            }
            Err(e) => Err(format!(
                "Failed to get address of {}: {}",
                provider.name(),
                e
            )),
        }
    }
}

pub mod core {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(
        pattern = "48 83 EC 48 48 8B 05 ?? ?? ?? ?? 4C 8D 0D ?? ?? ?? ?? BA 0A 00 00 00",
        offset = 0
    )]
    pub struct GetGameBuildRevision;
}

pub mod monster {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(pattern = "4C 89 B3 10 76 00 00", offset = -60)]
    pub struct Ctor;

    #[derive(AddressRecord)]
    #[record(pattern = "48 83 EC 20 48 8B B9 A0 09 00 00", offset = -20)]
    pub struct Dtor;

    #[derive(AddressRecord)]
    #[record(pattern = "48 85 C0 74 ?? 48 89 ?? ?? 48 8B ?? ?? ?? ?? ?? 48 89 ?? ?? 48 89 0A FF ?? ?? ?? ?? ?? 48 89 ?? ?? ?? ?? ?? C3", offset = -7)]
    pub struct SetTarget;

    #[derive(AddressRecord)]
    #[record(pattern = "48 89 ?? ?? ?? 56 57 41 54 48 ?? ?? ?? 48 8B ?? ?? ?? ?? ?? 49 8B F0 48 8B DA 48 8B F9 41", offset = -5)]
    pub struct ProcessThkSegment;
}

pub mod inline {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(
        pattern = "0F 57 F6 49 63 ?? ?? ?? ?? ?? ?? ?? ?? ?? ?? 85 D2 7E ?? 49 8B ?? ?? ?? ?? ?? 32 C0 48 85 C9 74 ?? 80 79 ?? ?? 0F 93 C0 EB ??",
        offset = 10
    )]
    pub struct WeaponATK;
}

pub mod player {
    use address_scanner::AddressRecord;

    // extern "C" fn(*const c_void, *const c_void) -> i64;
    #[derive(AddressRecord)]
    #[record(pattern = "8B ?? ?? ?? ?? ?? 48 85 C9 74 ?? E8 ?? ?? ?? ?? 33 C0 48 ?? ?? ?? C3", offset = -5)]
    pub struct Hit; // 0x141F50480

    #[derive(AddressRecord)]
    #[record(pattern = "4D 8B D8 4D 85 C0 75 ?? 4C 8B ?? ?? ?? ?? ?? 45 33 C0 4C 8D ?? ?? 45 8B D0 66 90", offset = -5)]
    pub struct RemoveCatSkill;

    #[derive(AddressRecord)]
    #[record(pattern = "8B 84 ?? ?? ?? ?? ?? C6 44 ?? ?? ?? C7 44 ?? ?? ?? ?? ?? ?? 89 44 ?? ?? 0F B6 ?? ?? ?? ?? ?? ?? 88 44 ?? ?? 8B 84", offset = -13)]
    pub struct DrawDamage;

    #[derive(AddressRecord)]
    #[record(pattern = "0F ?? ?? ?? ?? ?? ?? 73 ?? F3 0F ?? ?? ?? 0F 57 C9 F3 0F 5D C1 F3 0F ?? ?? ?? C3 0F 57 C0 0F 2F C8 72 ?? F3 0F ?? ?? ?? F3 0F 5D C1", offset = -5)]
    pub struct StealHealth;

    #[derive(AddressRecord)]
    #[record(pattern = "0F 57 C0 0F 2F ?? ?? ?? ?? ?? 0F ?? ?? ?? ?? ?? F3 ?? ?? ?? ?? ?? ?? ?? 48 ?? ?? ?? ?? ?? ?? F3 ?? ?? ?? ?? ?? ?? ?? 33 C9 49 89", offset = -10)]
    pub struct MuteCheck; // 0x141A4FCC0
}

pub mod chat {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(pattern = "81 08 10 00 00 48 ?? ?? ?? ?? ?? ?? 66 44 89 01 48 3B D0 74 ?? 44 89", offset = -5)]
    pub struct MessageSent;
}

pub mod quest {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(
        pattern = "40 53 57 41 57 48 83 EC 50 48 8B D9 45 0F B6 F8",
        offset = 0
    )]
    pub struct Accept;

    #[derive(AddressRecord)]
    #[record(pattern = "48 8B C4 55 48 81 EC F0 01 00 00 33 ED", offset = 0)]
    pub struct Enter;

    #[derive(AddressRecord)]
    #[record(pattern = "40 57 48 83 EC 60 83 79 38 02 48 8B F9", offset = 0)]
    pub struct Return;

    #[derive(AddressRecord)]
    #[record(pattern = "00 84 c0 0F 84 BE B0 9E 51 00", offset = -54)]
    pub struct Leave;

    #[derive(AddressRecord)]
    #[record(pattern = "F3 0F 2C C0 F3 0F 11 81 A4 31 01 00", offset = -67)]
    pub struct Abandon;

    #[derive(AddressRecord)]
    #[record(pattern = "48 81 EC 60 02 00 00 45 33 FF 48 8B D9", offset = -28)]
    pub struct Cancel;

    #[derive(AddressRecord)]
    #[record(pattern = "41 0F B6 F9 33 D2 41 8B F0 48 8B D9", offset = -37)]
    pub struct End;

    #[derive(AddressRecord)]
    #[record(pattern = "48 8B C4 53 55 48 81 EC 08 02 00 00", offset = 0)]
    pub struct DepartOn;

    #[derive(AddressRecord)]
    #[record(pattern = "41 56 48 83 EC 20 48 8D B1 A0 AE 00 00", offset = -20)]
    pub struct GetQuestname;

    #[derive(AddressRecord)]
    #[record(pattern = "48 ?? ?? ?? 65 ?? ?? ?? ?? ?? ?? ?? ?? 48 8B F1 44 ?? ?? ?? ?? ?? ?? 41 0F B6 E8 B9 ?? ?? ?? ?? 4C 63 F2 4E 8B 14 C8 41 8B 04 0A 39", offset = -5)]
    pub struct PlayerDeath; // 0x141B68E00
}

pub mod action {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(pattern = "48 63 0A 48 8D ?? ?? 48 ?? ?? ?? 46 3B 04 08 0F ?? ?? ?? ?? ?? 48 03 C9 49 8B ?? ?? ?? 4A ?? ?? ?? ?? 0F ?? ?? ?? ?? ?? 41 C6 ?? ?? ?? ?? ?? ?? 41 8B", offset = -7)]
    pub struct SetAction; // 0x140269C90
}

pub mod weapon {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(pattern = "48 ?? ?? ?? ?? ?? ?? 48 89 ?? ?? 45 8B E0 48 89 ?? ?? 48 8D ?? ?? ?? ?? ?? 4C 89 ?? ??", offset = -6)]
    pub struct Change;
}

pub mod steamwork {
    use address_scanner::AddressRecord;

    #[derive(AddressRecord)]
    #[record(pattern = "", offset = 0)]
    pub struct ChangeFuel; // 0x141349340

    #[derive(AddressRecord)]
    #[record(pattern = "BA ?? ?? ?? ?? 44 8D ?? ?? 41 FF D1 44 38 ?? ?? ?? ?? ?? 75 ?? 40 38 ?? ?? ?? ?? ?? 74 ?? BE ?? ?? ?? ?? 4B 8D 0C 64 8B D6 48 8B", offset = -2)]
    pub struct FailureJnzPatch; // 0x140666AFA
}
