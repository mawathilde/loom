use std::error::Error;

pub trait FileTransform {
    fn encode(&self, input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
    fn decode(&self, input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>>;
}

pub struct IdentityTransform;

impl FileTransform for IdentityTransform {
    fn encode(&self, input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(input.to_vec())
    }

    fn decode(&self, input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(input.to_vec())
    }
}
