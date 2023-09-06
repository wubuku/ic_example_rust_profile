use crate::profile::{ProfileId, Profile, ProfileUpdated};

pub(crate) fn verify(
    name: String,
    description: String,
    keywords: Vec<String>,
) -> ProfileUpdated {
    ProfileUpdated {
        profile_id: ProfileId(ic_cdk::api::caller()),
        version: 0,
        name,
        description,
        keywords,
    }
}

pub(crate) fn mutate(profile_updated: &ProfileUpdated) -> Profile {
    let profile = Profile {
        name: profile_updated.name.clone(),
        description: profile_updated.description.clone(),
        keywords: profile_updated.keywords.clone(),
    };
    profile
}