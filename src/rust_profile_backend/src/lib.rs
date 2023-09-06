use std::cell::RefCell;

use ic_cdk::{
    //export::Principal,
    query, update,
};
// use ic_cdk::{
//     export::{
//         candid::{CandidType, Deserialize},
//     },
// };
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableLog};
//use std::collections::BTreeMap;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};

use profile::Profile;
use profile::ProfileId;

mod profile;
mod events;
mod profile_update_logic;

type Memory = VirtualMemory<DefaultMemoryImpl>;

type EventStore = StableLog<events::Event, Memory, Memory>;

//type IdStore = BTreeMap<String, Principal>;
//type ProfileStore = BTreeMap<Principal, Profile>;
type ProfileStore = StableBTreeMap<ProfileId, Profile, Memory>;

thread_local! {
    //static PROFILE_STORE: RefCell<ProfileStore> = RefCell::default();
    //static ID_STORE: RefCell<IdStore> = RefCell::default();

    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static EVENT_STORE: RefCell<EventStore> = RefCell::new(
        StableLog::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        ).unwrap()
    );

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static PROFILE_STORE: RefCell<ProfileStore> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))),
        )
    );
}

#[query(name = "getSelf")]
fn get_self() -> Option<Profile> {
    let id = ic_cdk::api::caller();
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow().get(&ProfileId(id))
    })
}

// #[query]
// fn get(name: String) -> Profile {
//     ID_STORE.with(|id_store| {
//         PROFILE_STORE.with(|profile_store| {
//             id_store
//                 .borrow()
//                 .get(&name)
//                 .and_then(|id| profile_store.borrow().get(id).cloned()).unwrap_or_default()
//         })
//     })
// }

//
// dfx canister call rust_profile_backend update '("Luxi", "mountain dog", vec {"scars"; "toast"})'
//
// dfx canister call rust_profile_backend getSelf
//
#[update]
fn update(
    //profile: Profile
    name: String,
    description: String,
    keywords: Vec<String>,
) {
    let principal_id = ic_cdk::api::caller();
    // ID_STORE.with(|id_store| {
    //     id_store
    //         .borrow_mut()
    //         .insert(profile.name.clone(), principal_id);
    // });
    let profile_updated = profile_update_logic::verify(name, description, keywords);
    let profile = profile_update_logic::mutate(&profile_updated);
    EVENT_STORE.with(|event_store| {
        event_store.borrow_mut().append(&events::Event::ProfileEvent(events::ProfileEvent::ProfileUpdated(profile_updated))).unwrap();
    });
    PROFILE_STORE.with(|profile_store| {
        profile_store.borrow_mut().insert(
            ProfileId(principal_id),
            profile,
        );
    });
}


#[query(name = "getEvent")]
fn get_event(idx: u64) -> Option<events::Event> {
    EVENT_STORE.with(|event_store| {
        event_store.borrow().get(idx)
    })
}
