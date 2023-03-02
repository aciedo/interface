use rkyv::{Deserialize, Archive, Serialize};
use uuid::Uuid;
use bytecheck::CheckBytes;
use webauthn_proto::{CreationChallengeResponse, RegisterPublicKeyCredential, RequestChallengeResponse, PublicKeyCredential};

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct RegisterNumberReq {
    /// The name of the user to register.
    pub name: String,
    /// The phone number to register.
    pub number: String,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct RegisterNumberRes {
    /// A value to multiply the code by for VerifyNumberReq
    pub multiplier: i64,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct VerifyNumberReq {
    pub number: String,
    pub code: i64,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct VerifyNumberRes {
    /// The user's profile id
    pub id: Uuid,
    /// WebAuthn challenge
    pub opts: CreationChallengeResponse,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct RegisterPasskeyReq {
    /// The profile id
    pub id: Uuid,
    /// The challenge
    pub challenge: Vec<u8>,
    /// The response from the browser
    pub response: RegisterPublicKeyCredential,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct AuthChallengeReq {
    /// The profile id
    pub id: Uuid,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct AuthChallengeRes {
    /// WebAuthn challenge
    pub opts: RequestChallengeResponse,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct AuthVerifyReq {
    /// The profile id
    pub id: Uuid,
    /// The challenge
    pub challenge: Vec<u8>,
    /// The response from the browser
    pub response: PublicKeyCredential,
}

#[derive(Archive, Deserialize, Serialize, Debug, Clone)]
#[archive_attr(derive(CheckBytes))]
pub struct AuthVerifyRes {
    /// The session ID to use for future requests
    pub id: Uuid,
}