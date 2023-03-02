use rkyv::{Archive, Deserialize, Serialize};
use bytecheck::CheckBytes;
#[cfg(feature = "scylla")]
use scylla::{FromRow, ValueList};
use uuid::Uuid;

// only derive ValueList and FromRow if we're not compiling for wasm
#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "scylla", derive(FromRow, ValueList))]
#[archive_attr(derive(CheckBytes))]
pub struct Profile {
    /// The user's UUID
    pub id: Uuid,
    pub avatar: Option<String>,
    /// hash(hash(phone) + pepper)
    /// This is always 32 bytes long.
    pub hashed_phone: Vec<u8>,
    pub name: String,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct GetProfileByIdReq {
    pub id: Uuid,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct GetProfileByIdRes {
    pub profile: Option<Profile>,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct UploadContactsReq {
    pub contacts: Vec<Vec<u8>>,
}

/// A [Contact] contains additional, user-provided information about someone.
/// The person may or _may not_ be a registered user of the service. Users can
/// optionally sync their contacts using end-to-end encryption.
/// This allows them to see the names of their contacts across devices.
///
/// Users choose a device to be the "master" device, which will be used to
/// sync the contacts. This is set by default to the device that was used
/// to create the account and doesn't change unless the user explicitly
/// changes it.
///
/// None of the fields on this pub struct are visible to the server.
#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct Contact {
    /// The person's name, loosely formatted.
    pub name: String,
    /// The person's optional nickname.
    pub nickname: Option<String>,
    /// The person's phone number.
    pub phone: String,
    /// The person's optional avatar, JPEG encoded. Max size is 256KiB.
    pub avatar: Option<Vec<u8>>,
}
