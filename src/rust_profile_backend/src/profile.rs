use std::borrow::Cow;

use candid::Principal;
use ic_cdk::{
    export::{
        candid::{CandidType, Decode, Deserialize, Encode},
    },
};
use ic_stable_structures::{
    BoundedStorable,//storable::Bound,
    Storable,
};

const PRINCIPAL_MAX_LENGTH_IN_BYTES: u32 = 29;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
#[derive(Debug, CandidType, Deserialize)]
pub(crate) struct ProfileId(pub Principal);

impl Storable for ProfileId {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(self.0.as_slice().to_vec())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Self(Principal::from_slice(bytes.as_ref()))
    }
}

impl BoundedStorable for ProfileId {
    const MAX_SIZE: u32 = PRINCIPAL_MAX_LENGTH_IN_BYTES;
    const IS_FIXED_SIZE: bool = false;
}

const PROFILE_MAX_SIZE: u32 = 200;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub(crate) struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}

impl Storable for Profile {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Profile {
    const MAX_SIZE: u32 = PROFILE_MAX_SIZE;
    const IS_FIXED_SIZE: bool = false;

    // const BOUND: Bound = Bound::Bounded {
    //     max_size: MAX_VALUE_SIZE,
    //     is_fixed_size: false,
    // };
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) struct ProfileUpdated {
    pub profile_id: ProfileId,
    pub version: u64,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}

