use std::borrow::Cow;

use candid::{Decode, Encode};
use ic_cdk::{
    export::{
        candid::{CandidType, Deserialize},
    },
};
use ic_stable_structures::Storable;

use crate::profile::ProfileUpdated;

//use crate::profile::ProfileId;


#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) enum Event {
    ProfileEvent(ProfileEvent),
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub(crate) enum ProfileEvent {
    ProfileUpdated(ProfileUpdated),
}


impl Storable for Event {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

