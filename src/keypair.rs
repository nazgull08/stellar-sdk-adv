use anyhow::bail;
use nacl::sign::{generate_keypair, signature, verify};
use str_key::StrKey;

use crate::str_key;
use ed25519_dalek::{ExpandedSecretKey, SecretKey, Sha512};
use ed25519_dalek::Digest;


#[derive(Debug, Clone)]
pub struct Keypair {
    public_key: Vec<u8>,
    secret_key: Option<Vec<u8>>,
    secret_seed: Option<Vec<u8>>,
}

impl Keypair {
    fn new_from_secret_key(secret_seed: Vec<u8>) -> Result<Self, anyhow::Error> {
        if secret_seed.len() != 32 {
            bail!("secret_key length is invalid")
        }

        let mut cloned_secret_key = secret_seed.clone();
        let keypair = generate_keypair(&secret_seed);
        let mut pk = keypair.pkey.clone().to_vec();

        let mut secret_key = Vec::new();
        secret_key.append(&mut cloned_secret_key);
        secret_key.append(&mut pk);

        Ok(Self {
            secret_seed: Some(secret_seed),
            public_key: keypair.pkey.to_vec(),
            secret_key: Some(secret_key),
        })
    }

    fn new_from_secret_key_with_nonce(secret_seed: Vec<u8>, nonce: Vec<u8>) -> Result<Self, anyhow::Error> {
        if secret_seed.len() != 32 {
            bail!("secret_key length is invalid")
        }

        let mut nonced_secret_key = secret_seed.clone();
        println!("=======");
        println!("nonce: {:?}",nonce);
        println!("=======");
        for i in 0..nonce.len(){
            (nonced_secret_key[i],_) = nonced_secret_key[i].overflowing_add(nonce[i]);
        };
        let keypair = generate_keypair(&nonced_secret_key);
        let mut pk = keypair.pkey.clone().to_vec();

        let mut secret_key = Vec::new();
        secret_key.append(&mut nonced_secret_key);
        secret_key.append(&mut pk);

        Ok(Self {
            secret_seed: Some(nonced_secret_key),
            public_key: keypair.pkey.to_vec(),
            secret_key: Some(secret_key),
        })
    }

    fn new_from_public_key(public_key: Vec<u8>) -> Result<Self, anyhow::Error> {
        if public_key.len() != 32 {
            bail!("public_key length is invalid")
        }

        Ok(Self {
            public_key,
            secret_key: None,
            secret_seed: None,
        })
    }

    pub fn from_secret_key(secret: &str) -> Result<Self, anyhow::Error> {
        let raw_secret = StrKey::decode_ed25519_secret_seed(secret)?;

        Keypair::from_raw_ed25519_seed(&raw_secret)
    }

    pub fn from_secret_master_key(secret: &str, nonce:&str) -> Result<Self, anyhow::Error> {
        let raw_secret = StrKey::decode_ed25519_secret_seed(secret)?;
        let raw_nonce= nonce.as_bytes();
        let mut sha512 = Sha512::default();
        let mut cloned_sha512 = sha512.clone();
        let mut digest = sha512.chain(raw_secret.clone()).chain(nonce).finalize();
        let seed = &digest[..32];
        let secret_key = SecretKey::from_bytes(seed).unwrap(); 

        Keypair::from_raw_ed25519_seed(seed)
    }

    pub fn from_public_key(public_key: &str) -> Result<Self, anyhow::Error> {
        let decoded = StrKey::decode_ed25519_public_key(public_key)?;
        // let decoded = decode_check(&VersionBytes::Ed25519PublicKey, public_key);

        if decoded.len() != 32 {
            bail!("Invalid Stellar public key")
        }

        Ok(Self {
            public_key: decoded,
            secret_seed: None,
            secret_key: None,
        })
    }

    pub fn from_raw_ed25519_seed(seed: &[u8]) -> Result<Self, anyhow::Error> {
        Self::new_from_secret_key(seed.to_vec())
    }

    pub fn from_raw_ed25519_seed_with_nonce(seed: &[u8], nonce: &[u8]) -> Result<Self, anyhow::Error> {
        Self::new_from_secret_key_with_nonce(seed.to_vec(),nonce.to_vec())
    }


    pub fn raw_secret_key(&self) -> Option<Vec<u8>> {
        self.secret_seed.clone()
    }

    pub fn raw_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn secret_key(&mut self) -> Result<String, anyhow::Error> {
        match &mut self.secret_seed {
            None => bail!("no secret_key available"),
            Some(s) => Ok(StrKey::encode_ed25519_secret_seed(s)),
        }
    }

    pub fn public_key(&self) -> String {
        StrKey::encode_ed25519_public_key(&self.public_key)
    }

    pub fn can_sign(&self) -> bool {
        self.secret_key.is_some()
    }

    pub fn sign(&self, data: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
        if !self.can_sign() {
            bail!("cannot sign, no secret_key available")
        }

        if let Some(s) = &self.secret_key {
            match signature(data, s) {
                Err(_) => bail!("error while signing"),
                Ok(m) => return Ok(m),
            }
        }

        bail!("error while signing")
    }

    pub fn verify(&self, data: &[u8], signature: &[u8]) -> bool {
        verify(signature, data, &self.public_key).is_ok()
    }

    pub fn random() -> Result<Self, anyhow::Error> {
        Self::new_from_secret_key(rand::random::<[u8; 32]>().to_vec())
    }



    // fn master
    // fn xdr_account_id
    // fn xdr_public_key
    // fn xdr_muxed_account
    // fn signature_hint
    // fn sign_payload_decorated
    // fn sign_decorated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_master_secret_key() {
        println!("============TEST MASTER SECRET KEY=================");
        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let pk = String::from("GACAMF2WHKKQTYVHVA3CRMVUHN6GUBLTB7PBJQF73N7ATCIYAIFUCT6B");
        let pk_nonced1 = String::from("GBGCG3LQHGN4ROLA5GKMKQHGTBKZGA4OIE2AZYTC6LICGHWUXX67LNY3");
        let pk_nonced2 = String::from("GAP6JL574DJX3M36RK6SQSVKNCKOJXVUCFI2SENVC77F5VN22LDG5NHQ");
        let pk_nonced3 = String::from("GCPYIII5KJ56KTSLECFAV7OCG2HRERMZJXMONUSHAVBI57EDX74OQRFY");
        let seed_nonced1 = String::from("SBALFCEFFSFIVATKVUTODKNWNXDOU3ZDKZSV3RKWITOXHFNG43BJMSHS");
        let seed_nonced2 = String::from("SBREOODXPGF3PT64QEAMGMF3GIAIFKYRMJ3PBFX2R5U6J6RNKVKJMGZB");
        let seed_nonced3 = String::from("SDLZ2JSXKPODJMQMOSQRXPKVJZGGZDLQXL6OGEDLRFJ3JBKHJ46BBTDJ");
        let nonce1 = String::from("0");
        let nonce2 = String::from("1");
        let nonce3 = String::from("FWE4IF24WJ67IOQ8JWOI9EWQ3DAWD0WE");
        println!("seed {:?}",seed);
        println!("pk {:?}",pk);
        println!("nonce1 {:?}",nonce1);
        println!("nonce2 {:?}",nonce2);
        println!("nonce3 {:?}",nonce3);

        let mut keypair1 = Keypair::from_secret_master_key(&seed,&nonce1).unwrap();
        let mut keypair2 = Keypair::from_secret_master_key(&seed,&nonce2).unwrap();
        let mut keypair3 = Keypair::from_secret_master_key(&seed,&nonce3).unwrap();
        let seed_from_keypair1 = keypair1.secret_key().unwrap();
        let seed_from_keypair2 = keypair2.secret_key().unwrap();
        let seed_from_keypair3 = keypair3.secret_key().unwrap();

        println!("keypair1: {:?}",keypair1.clone());
        println!("keypair1 pk: {:?}",keypair1.public_key());
        println!("seed from keypair1: {:?}",seed_from_keypair1.clone());
        println!("keypair2: {:?}",keypair2.clone());
        println!("keypair2 pk: {:?}",keypair2.public_key());
        println!("seed from keypair2: {:?}",seed_from_keypair2.clone());
        println!("keypair3: {:?}",keypair3.clone());
        println!("keypair3 pk: {:?}",keypair3.public_key());
        println!("seed from keypair3: {:?}",seed_from_keypair3.clone());
        println!("============TEST MASTER SECRET KEY END=============");

        assert_eq!(pk_nonced1, keypair1.public_key());
        assert_eq!(pk_nonced2, keypair2.public_key());
        assert_eq!(pk_nonced3, keypair3.public_key());
        assert_eq!(seed_nonced1, seed_from_keypair1);
        assert_eq!(seed_nonced2, seed_from_keypair2);
        assert_eq!(seed_nonced3, seed_from_keypair3);
    }

    #[test]
    fn test_from_secret_key() {
        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let pk = String::from("GACAMF2WHKKQTYVHVA3CRMVUHN6GUBLTB7PBJQF73N7ATCIYAIFUCT6B");

        let mut keypair = Keypair::from_secret_key(&seed).unwrap();
        let seed_from_keypair = keypair.secret_key().unwrap();

        assert_eq!(pk, keypair.public_key());
        assert_eq!(seed, seed_from_keypair);
    }

    #[test]
    fn test_can_sign() {
        let pk = String::from("GACAMF2WHKKQTYVHVA3CRMVUHN6GUBLTB7PBJQF73N7ATCIYAIFUCT6B");
        let keypair = Keypair::from_public_key(&pk).unwrap();
        assert!(!keypair.can_sign());

        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let keypair = Keypair::from_secret_key(&seed).unwrap();
        assert!(keypair.can_sign());
    }

    #[test]
    fn test_from_raw_seed() {
        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let raw_seed = StrKey::decode_ed25519_secret_seed(&seed).unwrap();

        let keypair = Keypair::from_raw_ed25519_seed(&raw_seed).unwrap();

        if let Some(x) = keypair.raw_secret_key() {
            assert_eq!(raw_seed, x);
        }
    }

    #[test]
    fn test_sign_message() {
        let message = "Hello World".as_bytes().to_vec();

        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let keypair = Keypair::from_secret_key(&seed).unwrap();

        let signed_message = keypair.sign(&message).unwrap();

        let expected_signed_message = vec![
            249, 89, 99, 12, 220, 144, 11, 209, 11, 54, 119, 152, 58, 242, 131, 31, 212, 173, 213,
            95, 209, 35, 15, 223, 110, 215, 31, 220, 59, 125, 147, 141, 99, 116, 156, 12, 50, 28,
            137, 31, 0, 175, 86, 235, 92, 157, 151, 132, 88, 222, 147, 50, 248, 15, 191, 208, 153,
            16, 41, 169, 20, 202, 137, 15,
        ];

        assert_eq!(expected_signed_message, signed_message);
    }

    #[test]
    fn test_verify_signed_message() {
        let seed = String::from("SAZ443I6BNR2MD3G27C4EZIEEFMKOPT4SR6IHZDLXPODEHR2GRQVIC7R");
        let keypair = Keypair::from_secret_key(&seed).unwrap();

        let unsigned_message = "Hello World".as_bytes().to_vec();
        let signed_message = vec![
            249, 89, 99, 12, 220, 144, 11, 209, 11, 54, 119, 152, 58, 242, 131, 31, 212, 173, 213,
            95, 209, 35, 15, 223, 110, 215, 31, 220, 59, 125, 147, 141, 99, 116, 156, 12, 50, 28,
            137, 31, 0, 175, 86, 235, 92, 157, 151, 132, 88, 222, 147, 50, 248, 15, 191, 208, 153,
            16, 41, 169, 20, 202, 137, 15,
        ];

        assert!(keypair.verify(&signed_message, &unsigned_message))
    }

    #[test]
    fn test_random_keypair() {
        let keypair_1 = Keypair::random().unwrap();
        let keypair_2 = Keypair::random().unwrap();
        let keypair_3 = Keypair::random().unwrap();

        assert_ne!(keypair_1.raw_secret_key(), keypair_2.raw_secret_key());
        assert_ne!(keypair_2.raw_secret_key(), keypair_3.raw_secret_key());
    }
}
