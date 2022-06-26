use super::SignatureScheme;

use crate::core::hash::Hash;
use ed25519_dalek::{Signer, Verifier};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Ed25519<H: Hash>(std::marker::PhantomData<H>);

pub struct PrivateKey(pub ed25519_dalek::Keypair);

// Why not derivable?
impl Clone for PrivateKey {
    fn clone(&self) -> Self {
        PrivateKey(ed25519_dalek::Keypair::from_bytes(&self.0.to_bytes()).unwrap())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicKey(pub ed25519_dalek::PublicKey);

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Signature(pub ed25519_dalek::Signature);

impl<H: Hash> SignatureScheme for Ed25519<H> {
    type Pub = PublicKey;
    type Priv = PrivateKey;
    type Sig = Signature;
    fn generate_keys(seed: &[u8]) -> (PublicKey, PrivateKey) {
        let mut x = H::hash(seed);
        x.as_mut()[31] &= 0x7f;
        let secret = ed25519_dalek::SecretKey::from_bytes(x.as_ref()).unwrap();
        let public = ed25519_dalek::PublicKey::from(&secret);
        let keypair = ed25519_dalek::Keypair { public, secret };
        (PublicKey(public), PrivateKey(keypair))
    }
    fn sign(sk: &PrivateKey, message: &[u8]) -> Signature {
        Signature(sk.0.sign(message))
    }
    fn verify(pk: &PublicKey, message: &[u8], sig: &Signature) -> bool {
        pk.0.verify(message, &sig.0).is_ok()
    }
}

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x")?;
        for byte in self.0.as_bytes().iter().rev() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum ParsePublicKeyError {
    #[error("public key invalid")]
    Invalid,
}

impl FromStr for PublicKey {
    type Err = ParsePublicKeyError;
    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        if s.len() != 66 || !s.to_lowercase().starts_with("0x") {
            return Err(ParsePublicKeyError::Invalid);
        }
        s = &s[2..];
        let bytes = hex::decode(s)
            .map_err(|_| ParsePublicKeyError::Invalid)?
            .into_iter()
            .rev()
            .collect::<Vec<_>>();
        Ok(PublicKey(
            ed25519_dalek::PublicKey::from_bytes(&bytes)
                .map_err(|_| ParsePublicKeyError::Invalid)?,
        ))
    }
}
