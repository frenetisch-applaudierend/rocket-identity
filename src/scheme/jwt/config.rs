use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct JwtConfig {
    pub encoding_key: EncodingKey,
    pub deconding_key: DecodingKey,
}
