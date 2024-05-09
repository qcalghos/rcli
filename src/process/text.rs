use crate::get_reader;
use crate::TextSigFormat;
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::Signature;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use std::fs;
use std::io::Read;
trait TextSign {
    //返回签名结果
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}
trait TextVerify {
    //验证签名9
    //等价于 fn verify<R:Read>(&self,reader:R,sig:&[u8])
    fn verify(&self, reader: impl Read, sig: &[u8]) -> Result<bool>;
}
struct Blake3 {
    key: [u8; 32],
}
struct Ed25519Signer {
    key: SigningKey,
}
struct Ed25519Verifier {
    key: VerifyingKey,
}
pub fn process_text_sign(input: &str, key: &str, format: TextSigFormat) -> Result<()> {
    let mut reader = get_reader(input)?;
    let key = fs::read(key)?;
    let key = &key[..32];
    let key = key.try_into().unwrap();
    let signed = match format {
        TextSigFormat::Blake3 => {
            let signer = Blake3 { key };
            signer.sign(&mut reader)?
        }
        TextSigFormat::Ed25519 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{:?}", signed);
    Ok(())
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
        let binding = blake3::hash(&buf);
        let hash = binding.as_bytes();
        Ok(hash == sig)
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
impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify_strict(&buf, &sig).is_ok();
        Ok(ret)
    }
}
