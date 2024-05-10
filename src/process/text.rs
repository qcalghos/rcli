use crate::get_reader;
use crate::process_genpass;
use crate::TextSigFormat;
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::Signature;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use std::fs;
use std::io::Read;
use std::path::Path;
pub trait TextSign {
    //返回签名结果
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
pub trait TextVerify {
    //验证签名9
    //等价于 fn verify<R:Read>(&self,reader:R,sig:&[u8])
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}
pub trait KeyGenerator{
    fn generate()->Result<Vec<Vec<u8>>>;
}
pub trait KeyLoader
where
    Self: Sized,
{
    fn load(path: impl AsRef<Path>) -> Result<Self>;
}
pub struct Blake3 {
    pub key: [u8; 32],
}
pub struct Ed25519Signer {
    pub key: SigningKey,
}
pub struct Ed25519Verifier {
    pub key: VerifyingKey,
}
pub fn process_text_sign(input: &str, key: &str, format: TextSigFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSigFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSigFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}
pub fn process_text_verify(
    input: &str,
    key: &str,
    sig: &str,
    format: TextSigFormat,
) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    match format {
        TextSigFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(reader, &sig)
        }
        TextSigFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)
        }
    }
}
pub fn process_text_generate(format:TextSigFormat)->Result<Vec<Vec<u8>>>{
    match format{
        TextSigFormat::Blake3=>Blake3::generate(),
        TextSigFormat::Ed25519=>Ed25519Signer::generate()
    }

}
impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // 这个写法存在生命周期问题 let hash=blake3::hash(&buf).as_bytes();
        let binding = blake3::keyed_hash(&self.key,&buf);
        let hash = binding.as_bytes();
        Ok(hash == sig)
    }
}
impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}
impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify_strict(&buf, &sig).is_ok();
        Ok(ret)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}
impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
    //改用KeyLoader trait方法实现
    // pub fn load(path: impl AsRef<Path>) -> Result<Self> {
    //     let key = fs::read(path)?;
    //     Self::try_new(&key)
    // }
}
impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);

        let signer = Self::new(key);
        Ok(signer)
    }
    // pub fn load(path: impl AsRef<Path>) -> Result<Self> {
    //     let key = fs::read(path)?;
    //     Self::try_new(&key)
    // }
}
impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }
    // pub fn load(path: impl AsRef<Path>) -> Result<Self> {
    //     let key = fs::read(path)?;
    //     Self::try_new(&key)
    // }
}
impl KeyGenerator for Blake3{
    fn generate()->Result<Vec<Vec<u8>>> {
        let key=process_genpass(32, false, false, false, false)?;
        
        let key=key.as_bytes().to_vec();
        Ok(vec![key])
    }
}
impl KeyGenerator for Ed25519Signer{
    fn generate()->Result<Vec<Vec<u8>>> {
        let mut csprng=OsRng;
        let signing_key=SigningKey::generate(&mut csprng);
        let public_key=signing_key.verifying_key().to_bytes().to_vec();
        let signing_key=signing_key.to_bytes().to_vec();
        Ok(vec![signing_key,public_key])
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_blake3_sign_verify(){
        let key="./fixtures/blake3.txt";
        let blake3=Blake3::load(key).unwrap();
        let data=b"hello,world";
        let sig=blake3.sign(&mut &data[..]).unwrap();
        println!("sig:{}",URL_SAFE_NO_PAD.encode(&sig));
        let verified=blake3.verify(&mut &data[..], &sig).unwrap();
        assert!(verified);

    }
}