use super::Encryptor;
use aead::{generic_array::GenericArray, Aead, Error, NewAead, Payload};
use generic_array::typenum::{U0, U16, U24, U32, U48};
use rustchacha20poly1305::XChaCha20Poly1305 as SysXChaCha20Poly1305;
#[cfg(feature = "serialization")]
use serde::de::{Deserialize, Deserializer, Error as DError, Visitor};
#[cfg(feature = "serialization")]
use serde::ser::{Serialize, Serializer};
use zeroize::Zeroize;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct XChaCha20Poly1305 {
    key: GenericArray<u8, U32>,
}

impl Encryptor for XChaCha20Poly1305 {
    type MinSize = U48;
}

impl NewAead for XChaCha20Poly1305 {
    type KeySize = U32;

    fn new(key: GenericArray<u8, Self::KeySize>) -> Self {
        Self { key }
    }
}

impl Aead for XChaCha20Poly1305 {
    type NonceSize = U24;
    type TagSize = U16;
    type CiphertextOverhead = U0;

    fn encrypt<'msg, 'aad>(
        &self,
        nonce: &GenericArray<u8, Self::NonceSize>,
        plaintext: impl Into<Payload<'msg, 'aad>>,
    ) -> Result<Vec<u8>, Error> {
        let aead = SysXChaCha20Poly1305::new(self.key.clone());
        let ciphertext = aead.encrypt(nonce, plaintext)?;
        Ok(ciphertext)
    }

    fn decrypt<'msg, 'aad>(
        &self,
        nonce: &GenericArray<u8, Self::NonceSize>,
        ciphertext: impl Into<Payload<'msg, 'aad>>,
    ) -> Result<Vec<u8>, Error> {
        let aead = SysXChaCha20Poly1305::new(self.key.clone());
        let plaintext = aead.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }
}

default_impl!(XChaCha20Poly1305);
drop_impl!(XChaCha20Poly1305);
#[cfg(feature = "serialization")]
serialize_impl!(XChaCha20Poly1305, XChaCha20Poly1305Visitor);

#[cfg(test)]
mod tests {
    tests_impl!(XChaCha20Poly1305);
}
