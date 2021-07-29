//! Module to build recipients for the various types of COSE messages, examples of it can be seen on each type of
//! COSE message structure.
//!
//! This structure is also used to build counter signatures that can be present in any type of COSE
//! message.
//!
//! # Example
//!
//! This example shows a cose-sign message with 3 counter signatures present in it, one of them is
//! counter signed externally to the crate.
//!
//! ```
//! use cose::sign;
//! use cose::recipients;
//! use cose::keys;
//! use cose::headers;
//! use cose::algs;
//! use openssl::bn::BigNum;
//! use openssl::bn::BigNumContext;
//! use openssl::ec::EcPoint;
//! use openssl::ec::{EcGroup, EcKey};
//! use openssl::hash::MessageDigest;
//! use openssl::pkey::PKey;
//! use openssl::sign::{Signer, Verifier};
//! use openssl::nid::Nid;
//!
//! fn main() {
//!     let msg = b"signed message".to_vec();
//!     let kid = b"kid2".to_vec();
//!     let alg = algs::EDDSA;
//!      
//!     // Prepare cose-sing1 headers
//!     let mut sign1 = sign::CoseSign::new();
//!     sign1.header.alg(alg, true, false);
//!     sign1.header.kid(kid, true, false);
//!
//!     // Add payload
//!     sign1.payload(msg);
//!
//!     // Prepare cose-key
//!     let mut key = keys::CoseKey::new();
//!     key.kty(keys::EC2);
//!     key.alg(algs::EDDSA);
//!     key.crv(keys::ED25519);
//!     key.x(
//!         hex::decode("d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a")
//!             .unwrap(),
//!     );
//!     key.d(
//!         hex::decode("9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60")
//!             .unwrap(),
//!     );
//!     key.key_ops(vec![keys::KEY_OPS_SIGN, keys::KEY_OPS_VERIFY]);
//!
//!     // Add cose-key
//!     sign1.key(&key).unwrap();
//!     // Generate signature before adding counter signatures
//!     sign1.gen_signature(None).unwrap();
//!
//!     // Prepare counter signature 1 headers
//!     let mut counter1 = recipients::CoseRecipient::new_counter_sig();
//!     counter1.header.kid([0].to_vec(), true, false);
//!     counter1.header.alg(algs::ES256, true, false);
//!
//!     // Prepare counter 1 cose-key
//!     let mut counter1_key= keys::CoseKey::new();
//!     counter1_key.kty(keys::EC2);
//!     counter1_key.alg(algs::ES256);
//!     counter1_key.crv(keys::P_256);
//!     counter1_key.x(
//!         hex::decode("98f50a4ff6c05861c8860d13a638ea56c3f5ad7590bbfbf054e1c7b4d91d6280")
//!             .unwrap(),
//!     );
//!     counter1_key.y(
//!         hex::decode("f01400b089867804b8e9fc96c3932161f1934f4223069170d924b7e03bf822bb")
//!             .unwrap(),
//!     );
//!     counter1_key.d(
//!         hex::decode("02d1f7e6f26c43d4868d87ceb2353161740aacf1f7163647984b522a848df1c3")
//!             .unwrap(),
//!     );
//!     counter1_key.key_ops(vec![keys::KEY_OPS_SIGN]);
//!
//!     // Add counter 1 cose-key
//!     counter1.key(&counter1_key).unwrap();
//!
//!     // Counter sign with counter 1 the cose-sign1 signature
//!     sign1.counter_sig(None, &mut counter1).unwrap();
//!     // Add counter 1 to cose-sign1
//!     sign1.add_counter_sig(counter1).unwrap();
//!
//!     // Prepare counter 2 cose-key
//!     let mut counter2 = recipients::CoseRecipient::new_counter_sig();
//!     counter2.header.alg(algs::ES256, true, false);
//!     counter2.header.kid([1].to_vec(), true, false);
//!
//!     // Prepare counter 2 cose-key
//!     let mut counter2_key = keys::CoseKey::new();
//!     counter2_key.kty(keys::EC2);
//!     counter2_key.alg(algs::ES256);
//!     counter2_key.crv(keys::P_256);
//!     counter2_key.x(
//!         hex::decode("98f50a4ff6c05861c8860d13a638ea56c3f5ad7590bbfbf054e1c7b4d91d6280")
//!             .unwrap(),
//!     );
//!     counter2_key.y(
//!         hex::decode("f01400b089867804b8e9fc96c3932161f1934f4223069170d924b7e03bf822bb")
//!             .unwrap(),
//!     );
//!     counter2_key.d(
//!         hex::decode("02d1f7e6f26c43d4868d87ceb2353161740aacf1f7163647984b522a848df1c3")
//!             .unwrap(),
//!     );
//!     counter2_key.key_ops(vec![keys::KEY_OPS_SIGN]);
//!
//!     // Add counter 2 cose-key
//!     counter2.key(&counter2_key).unwrap();
//!
//!     // Counter sign with counter 2 the cose-sign1 signature
//!     sign1.counter_sig(None, &mut counter2).unwrap();
//!     // Add counter 2 to cose-sign1
//!     sign1.add_counter_sig(counter2).unwrap();
//!
//!     // Prepare counter 3 headers
//!     let mut counter3 = recipients::CoseRecipient::new_counter_sig();
//!     counter3.header.alg(algs::ES256, true, false);
//!     counter3.header.kid([3].to_vec(), true, false);
//!
//!     // Get sign_struct to counter sign from the cose-sign1
//!     let to_sign = sign1.get_to_sign(None, &mut counter3).unwrap();
//!
//!     // Key pair
//!     let counter3_pub_key =
//!         hex::decode("0398f50a4ff6c05861c8860d13a638ea56c3f5ad7590bbfbf054e1c7b4d91d6280")
//!         .unwrap();
//!     let counter3_priv_key =
//!         hex::decode("02d1f7e6f26c43d4868d87ceb2353161740aacf1f7163647984b522a848df1c3")
//!             .unwrap();
//!
//!     // Counter sign the content to sign
//!     let number = BigNum::from_slice(counter3_priv_key.as_slice()).unwrap();
//!     let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap();
//!     let ec_key = EcKey::from_private_components(&group, &number, &EcPoint::new(&group).unwrap()).unwrap();
//!     let final_key = PKey::from_ec_key(ec_key).unwrap();
//!     let mut signer = Signer::new(MessageDigest::sha256(), &final_key).unwrap();
//!     signer.update(to_sign.as_slice()).unwrap();
//!
//!     let signature = signer.sign_to_vec().unwrap();
//!
//!     // Add externally made counter signature to counter 3
//!     counter3.add_signature(signature).unwrap();
//!     // Add counter 3 to cose-sign1
//!     sign1.add_counter_sig(counter3).unwrap();
//!
//!     // Encode the cose-sign1 message
//!     sign1.encode(true).unwrap();
//!
//!     // Prepare verifier
//!     let mut verify = sign::CoseSign::new();
//!     verify.bytes = sign1.bytes.clone();
//!
//!     // Decode the cose-sign1 message
//!     verify.init_decoder(None).unwrap();
//!     verify.key(&key).unwrap();
//!     verify.decode(None, None).unwrap();
//!
//!     // Get all the counter signatures from cose-sign1
//!     let counters = verify.header.get_counters().unwrap();
//!
//!     // loop through all counter signatures and verify them
//!     for mut c in counters {
//!         // If it's counter 3, verify the counter signature externally
//!         if *c.header.kid.as_ref().unwrap() == vec![3] {
//!             let to_sign = sign1.get_to_sign(None, &mut c).unwrap();
//!
//!             let mut ctx = BigNumContext::new().unwrap();
//!             let group = EcGroup::from_curve_name(Nid::X9_62_PRIME256V1).unwrap();
//!             let point = EcPoint::from_bytes(&group, &counter3_pub_key, &mut ctx).unwrap();
//!             let ec_key = EcKey::from_public_key(&group, &point).unwrap();
//!             let final_key = PKey::from_ec_key(ec_key).unwrap();
//!             let mut verifier = Verifier::new(MessageDigest::sha256(), &final_key).unwrap();
//!             verifier.update(&to_sign).unwrap();
//!             assert!(verifier.verify(&c.payload).unwrap());
//!         } else {
//!             // For this example the same key was used for all counter signatures
//!             let mut c_key = keys::CoseKey::new();
//!             c_key.kty(keys::EC2);
//!             c_key.alg(algs::ES256);
//!             c_key.crv(keys::P_256);
//!             c_key.x(hex::decode(
//!                 "98f50a4ff6c05861c8860d13a638ea56c3f5ad7590bbfbf054e1c7b4d91d6280",
//!             )
//!             .unwrap());
//!             c_key.y(hex::decode(
//!                 "f01400b089867804b8e9fc96c3932161f1934f4223069170d924b7e03bf822bb",
//!             )
//!             .unwrap());
//!             c_key.d(hex::decode(
//!                 "02d1f7e6f26c43d4868d87ceb2353161740aacf1f7163647984b522a848df1c3",
//!             )
//!             .unwrap());
//!             c_key.key_ops(vec![keys::KEY_OPS_VERIFY]);
//!             c.key(&c_key).unwrap();
//!             
//!             verify.counters_verify(None, &c).unwrap();
//!         }
//!     }
//! }
//!
//! ```
use crate::algs;
use crate::enc_struct;
use crate::errors::{CoseError, CoseResult, CoseResultWithRet};
use crate::headers;
use crate::kdf_struct;
use crate::keys;
use crate::sig_struct;
use cbor::{Decoder, Encoder};
use std::io::Cursor;

/// COSE recipient/counter-signature structure.
#[derive(Clone)]
pub struct CoseRecipient {
    /// Header of the COSE recipient/counter-signature.
    pub header: headers::CoseHeader,
    /// Payload (signature, ciphertext or MAC) of the COSE recipient/counter-signature.
    pub payload: Vec<u8>,
    pub(in crate) ph_bstr: Vec<u8>,
    /// Public key.
    pub pub_key: Vec<u8>,
    /// Private/Symmetric key.
    pub s_key: Vec<u8>,
    pub(in crate) context: String,
    pub(in crate) crv: Option<i32>,
    pub(in crate) key_ops: Vec<i32>,
}

pub const COUNTER_CONTEXT: &str = "CounterSignature";
pub const SIZE: usize = 3;

impl CoseRecipient {
    /// Creates an empty CoseRecipient structure.
    pub fn new() -> CoseRecipient {
        CoseRecipient {
            header: headers::CoseHeader::new(),
            payload: Vec::new(),
            ph_bstr: Vec::new(),
            pub_key: Vec::new(),
            key_ops: Vec::new(),
            s_key: Vec::new(),
            crv: None,
            context: "".to_string(),
        }
    }

    /// Creates an empty CoseRecipient structure for counter signatures.
    pub fn new_counter_sig() -> CoseRecipient {
        CoseRecipient {
            header: headers::CoseHeader::new(),
            payload: Vec::new(),
            ph_bstr: Vec::new(),
            pub_key: Vec::new(),
            key_ops: Vec::new(),
            s_key: Vec::new(),
            crv: None,
            context: COUNTER_CONTEXT.to_string(),
        }
    }

    /// Adds an [header](../headers/struct.CoseHeader.html).
    pub fn add_header(&mut self, header: headers::CoseHeader) {
        self.header = header;
    }

    /// Adds a [cose-key](../keys/struct.CoseKey.html).
    pub fn key(&mut self, key: &keys::CoseKey) -> CoseResult {
        let alg = self
            .header
            .alg
            .as_ref()
            .ok_or(CoseError::MissingAlgorithm())?;
        if algs::SIGNING_ALGS.contains(alg) {
            if key.key_ops.contains(&keys::KEY_OPS_SIGN) {
                self.s_key = key.get_s_key()?;
            }
            if key.key_ops.contains(&keys::KEY_OPS_VERIFY) {
                self.pub_key = key.get_pub_key(*alg)?;
            }
        } else if algs::KEY_DISTRIBUTION_ALGS.contains(alg) || algs::ENCRYPT_ALGS.contains(alg) {
            if key.key_ops.contains(&keys::KEY_OPS_DERIVE_BITS)
                || key.key_ops.contains(&keys::KEY_OPS_DERIVE)
                || key.key_ops.contains(&keys::KEY_OPS_DECRYPT)
                || key.key_ops.contains(&keys::KEY_OPS_ENCRYPT)
                || key.key_ops.contains(&keys::KEY_OPS_WRAP)
                || key.key_ops.contains(&keys::KEY_OPS_UNWRAP)
            {
                self.s_key = key.get_s_key()?;
            }
            if algs::ECDH_ALGS.contains(alg) {
                if key.key_ops.len() == 0 {
                    self.pub_key = key.get_pub_key(alg.clone())?;
                }
            }
        }
        self.crv = key.crv;
        self.key_ops = key.key_ops.clone();
        Ok(())
    }

    pub(in crate) fn enc(
        &mut self,
        content: &Vec<u8>,
        external_aad: &Vec<u8>,
        body_protected: &Vec<u8>,
        alg: &i32,
        iv: &Vec<u8>,
    ) -> CoseResultWithRet<Vec<u8>> {
        if !self.key_ops.contains(&keys::KEY_OPS_ENCRYPT) {
            return Err(CoseError::KeyDoesntSupportSigning());
        }
        Ok(enc_struct::gen_cipher(
            &self.s_key,
            alg,
            iv,
            &external_aad,
            &self.context,
            &body_protected,
            &content,
        )?)
    }
    pub(in crate) fn dec(
        &self,
        content: &Vec<u8>,
        external_aad: &Vec<u8>,
        body_protected: &Vec<u8>,
        alg: &i32,
        iv: &Vec<u8>,
    ) -> CoseResultWithRet<Vec<u8>> {
        if !self.key_ops.contains(&keys::KEY_OPS_DECRYPT) {
            return Err(CoseError::KeyDoesntSupportVerification());
        }
        Ok(enc_struct::dec_cipher(
            &self.s_key,
            alg,
            iv,
            &external_aad,
            &self.context,
            &body_protected,
            &content,
        )?)
    }

    pub(in crate) fn sign(
        &mut self,
        content: &Vec<u8>,
        external_aad: &Vec<u8>,
        body_protected: &Vec<u8>,
    ) -> CoseResult {
        self.ph_bstr = self.header.get_protected_bstr()?;
        if !self.key_ops.contains(&keys::KEY_OPS_SIGN) {
            return Err(CoseError::KeyDoesntSupportSigning());
        }
        self.payload = sig_struct::gen_sig(
            &self.s_key,
            &self.header.alg.ok_or(CoseError::MissingAlgorithm())?,
            &external_aad,
            &self.context,
            &body_protected,
            &self.ph_bstr,
            &content,
        )?;
        Ok(())
    }
    pub(in crate) fn verify(
        &self,
        content: &Vec<u8>,
        external_aad: &Vec<u8>,
        body_protected: &Vec<u8>,
    ) -> CoseResult {
        if !self.key_ops.contains(&keys::KEY_OPS_VERIFY) {
            return Err(CoseError::KeyDoesntSupportVerification());
        }
        assert!(sig_struct::verify_sig(
            &self.pub_key,
            &self.header.alg.ok_or(CoseError::MissingAlgorithm())?,
            &external_aad,
            &self.context,
            &body_protected,
            &self.ph_bstr,
            &content,
            &self.payload,
        )?);
        Ok(())
    }

    /// Adds a signature to the counter signature.
    ///
    /// Function to use when signature was produce externally to the module.
    ///  
    /// This function is to use only in the context of counter signatures, not message recipients.
    pub fn add_signature(&mut self, signature: Vec<u8>) -> CoseResult {
        if self.context != COUNTER_CONTEXT {
            return Err(CoseError::FunctionOnlyAvailableForContext(
                COUNTER_CONTEXT.to_string(),
            ));
        }
        self.payload = signature;
        Ok(())
    }

    pub(in crate) fn get_to_sign(
        &mut self,
        content: &Vec<u8>,
        external_aad: &Vec<u8>,
        body_protected: &Vec<u8>,
    ) -> CoseResultWithRet<Vec<u8>> {
        if self.context != COUNTER_CONTEXT {
            return Err(CoseError::FunctionOnlyAvailableForContext(
                COUNTER_CONTEXT.to_string(),
            ));
        }
        self.ph_bstr = self.header.get_protected_bstr()?;
        sig_struct::get_to_sign(
            &external_aad,
            COUNTER_CONTEXT,
            &body_protected,
            &self.ph_bstr,
            &content,
        )
    }

    pub(in crate) fn derive_key(
        &mut self,
        cek: &Vec<u8>,
        size: usize,
        sender: bool,
    ) -> CoseResultWithRet<Vec<u8>> {
        if self.ph_bstr.len() <= 0 {
            self.ph_bstr = self.header.get_protected_bstr()?;
        }
        if [algs::A128KW, algs::A192KW, algs::A256KW].contains(
            self.header
                .alg
                .as_ref()
                .ok_or(CoseError::MissingAlgorithm())?,
        ) {
            if sender {
                self.payload = algs::aes_key_wrap(&self.s_key, size, &cek)?;
            } else {
                return Ok(algs::aes_key_unwrap(&self.s_key, size, &cek)?);
            }
            return Ok(cek.to_vec());
        } else if [algs::DIRECT_HKDF_AES_128, algs::DIRECT_HKDF_AES_256].contains(
            self.header
                .alg
                .as_ref()
                .ok_or(CoseError::MissingAlgorithm())?,
        ) {
            return Err(CoseError::NotImplemented(
                "DIRECT HKDF AES-128/AES-256".to_string(),
            ));
        } else if [algs::DIRECT_HKDF_SHA_256, algs::DIRECT_HKDF_SHA_512].contains(
            self.header
                .alg
                .as_ref()
                .ok_or(CoseError::MissingAlgorithm())?,
        ) {
            let salt;
            if self.header.party_u_nonce == None {
                salt = Some(
                    self.header
                        .salt
                        .as_ref()
                        .ok_or(CoseError::MissingAlgorithm())?,
                );
            } else {
                salt = Some(
                    self.header
                        .party_u_nonce
                        .as_ref()
                        .ok_or(CoseError::MissingAlgorithm())?,
                );
            }
            let mut kdf_context = kdf_struct::gen_kdf(
                self.header
                    .alg
                    .as_ref()
                    .ok_or(CoseError::MissingAlgorithm())?,
                self.header.party_u_identity.clone(),
                self.header.party_u_nonce.clone(),
                self.header.party_u_other.clone(),
                self.header.party_v_identity.clone(),
                self.header.party_v_nonce.clone(),
                self.header.party_v_other.clone(),
                size as u16 * 8,
                self.ph_bstr.clone(),
                None,
                None,
            )?;
            return Ok(algs::hkdf(
                size,
                &self.s_key,
                salt,
                &mut kdf_context,
                self.header.alg.unwrap(),
            )?);
        } else if [
            algs::ECDH_ES_HKDF_256,
            algs::ECDH_ES_HKDF_512,
            algs::ECDH_SS_HKDF_256,
            algs::ECDH_SS_HKDF_512,
        ]
        .contains(
            self.header
                .alg
                .as_ref()
                .ok_or(CoseError::MissingAlgorithm())?,
        ) {
            let (salt, receiver_key, sender_key, crv_rec, crv_send);
            if self.header.party_u_nonce == None {
                salt = Some(
                    self.header
                        .salt
                        .as_ref()
                        .ok_or(CoseError::MissingAlgorithm())?,
                );
            } else {
                salt = Some(
                    self.header
                        .party_u_nonce
                        .as_ref()
                        .ok_or(CoseError::MissingAlgorithm())?,
                );
            }

            if sender {
                receiver_key = self.pub_key.clone();
                sender_key = self.header.ecdh_key.get_s_key()?;
                crv_rec = self.crv.unwrap();
                crv_send = self.header.ecdh_key.crv.unwrap();
            } else {
                receiver_key = self.header.ecdh_key.get_pub_key(self.header.alg.unwrap())?;
                sender_key = self.s_key.clone();
                crv_send = self.crv.unwrap();
                crv_rec = self.header.ecdh_key.crv.unwrap();
            }
            let shared = algs::ecdh_derive_key(&crv_rec, &crv_send, &receiver_key, &sender_key)?;

            let mut kdf_context = kdf_struct::gen_kdf(
                self.header
                    .alg
                    .as_ref()
                    .ok_or(CoseError::MissingAlgorithm())?,
                self.header.party_u_identity.clone(),
                self.header.party_u_nonce.clone(),
                self.header.party_u_other.clone(),
                self.header.party_v_identity.clone(),
                self.header.party_v_nonce.clone(),
                self.header.party_v_other.clone(),
                size as u16 * 8,
                self.ph_bstr.clone(),
                None,
                None,
            )?;
            return Ok(algs::hkdf(
                size,
                &shared,
                salt,
                &mut kdf_context,
                self.header.alg.unwrap(),
            )?);
        } else if [
            algs::ECDH_ES_A128KW,
            algs::ECDH_ES_A192KW,
            algs::ECDH_ES_A256KW,
            algs::ECDH_SS_A128KW,
            algs::ECDH_SS_A192KW,
            algs::ECDH_SS_A256KW,
        ]
        .contains(
            self.header
                .alg
                .as_ref()
                .ok_or(CoseError::MissingAlgorithm())?,
        ) {
            let (salt, receiver_key, sender_key, crv_rec, crv_send);
            if self.header.party_u_nonce == None {
                salt = Some(
                    self.header
                        .salt
                        .as_ref()
                        .ok_or(CoseError::MissingAlgorithm())?,
                );
            } else {
                salt = Some(
                    self.header
                        .party_u_nonce
                        .as_ref()
                        .ok_or(CoseError::MissingAlgorithm())?,
                );
            }
            if sender {
                receiver_key = self.pub_key.clone();
                sender_key = self.header.ecdh_key.get_s_key()?;
                crv_rec = self.crv.unwrap();
                crv_send = self.header.ecdh_key.crv.unwrap();
            } else {
                receiver_key = self.header.ecdh_key.get_pub_key(self.header.alg.unwrap())?;
                sender_key = self.s_key.clone();
                crv_send = self.crv.unwrap();
                crv_rec = self.header.ecdh_key.crv.unwrap();
            }
            let shared = algs::ecdh_derive_key(&crv_rec, &crv_send, &receiver_key, &sender_key)?;

            let mut kdf_context = kdf_struct::gen_kdf(
                self.header
                    .alg
                    .as_ref()
                    .ok_or(CoseError::MissingAlgorithm())?,
                self.header.party_u_identity.clone(),
                self.header.party_u_nonce.clone(),
                self.header.party_u_other.clone(),
                self.header.party_v_identity.clone(),
                self.header.party_v_nonce.clone(),
                self.header.party_v_other.clone(),
                size as u16 * 8,
                self.ph_bstr.clone(),
                None,
                None,
            )?;
            let kek = algs::hkdf(
                size,
                &shared,
                salt,
                &mut kdf_context,
                self.header.alg.unwrap(),
            )?;
            if sender {
                self.payload = algs::aes_key_wrap(&kek, size, &cek)?;
            } else {
                return Ok(algs::aes_key_unwrap(&kek, size, &cek)?);
            }
            return Ok(cek.to_vec());
        } else {
            return Err(CoseError::InvalidCoseStructure());
        }
    }

    pub(in crate) fn decode(&mut self, d: &mut Decoder<Cursor<Vec<u8>>>) -> CoseResult {
        self.header.decode_protected_bstr(&self.ph_bstr)?;
        self.header.decode_unprotected(d, true)?;
        self.payload = d.bytes()?;
        Ok(())
    }

    pub(in crate) fn encode(&mut self, e: &mut Encoder<Vec<u8>>) -> CoseResult {
        e.array(SIZE)?;
        e.bytes(&self.ph_bstr)?;
        self.header.encode_unprotected(e)?;
        e.bytes(&self.payload)?;
        Ok(())
    }
}
