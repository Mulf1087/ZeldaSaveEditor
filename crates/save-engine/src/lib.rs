pub mod binary;
pub mod botw;
pub mod completism;
pub mod error;
pub mod hashtable;
pub mod totk;

pub use error::SaveError;

pub enum Save {
    Botw(botw::BotwSave),
    Totk(totk::TotkSave),
}

impl Save {
    pub fn detect(bytes: Vec<u8>) -> Result<Save, SaveError> {
        if let Ok(save) = botw::BotwSave::load(bytes.clone()) {
            return Ok(Save::Botw(save));
        }
        totk::TotkSave::load(bytes).map(Save::Totk)
    }

    pub fn to_bytes(self) -> Vec<u8> {
        match self {
            Save::Botw(save) => save.to_bytes(),
            Save::Totk(save) => save.to_bytes(),
        }
    }
}
