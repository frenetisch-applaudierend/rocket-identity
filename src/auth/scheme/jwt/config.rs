use jsonwebtoken::{DecodingKey, EncodingKey};

#[derive(Clone)]
pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub deconding_key: DecodingKey,
}

impl core::fmt::Debug for JwtConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtConfig")
            .field("encoding_key", &"hidden")
            .field("deconding_key", &"hidden")
            .finish()
    }
}
