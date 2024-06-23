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
    #[record(pattern = "48 8B 81 D0 00 00 00 48 85 C0 74 0F", offset = 0)]
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

    #[derive(AddressRecord)]
    #[record(pattern = "48 83 EC 28 48 8B 89 58 76 00 00 48 85 C9", offset = 0)]
    pub struct Hit;

    #[derive(AddressRecord)]
    #[record(pattern = "4D 8B D8 4D 85 C0 75 ?? 4C 8B ?? ?? ?? ?? ?? 45 33 C0 4C 8D ?? ?? 45 8B D0 66 90", offset = -5)]
    pub struct RemoveCatSkill;

    #[derive(AddressRecord)]
    #[record(pattern = "8B 84 ?? ?? ?? ?? ?? C6 44 ?? ?? ?? C7 44 ?? ?? ?? ?? ?? ?? 89 44 ?? ?? 0F B6 ?? ?? ?? ?? ?? ?? 88 44 ?? ?? 8B 84", offset = -13)]
    pub struct DrawDamage;
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
}
