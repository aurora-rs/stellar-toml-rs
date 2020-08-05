//! # Stellar Toml
//!
//! The `stellar-toml` provides functions to access the `stellar.toml`
//! file. This file contains information about an organization's
//! Stellar integration. See
//! [SEP-0001](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0001.md)
//! for more information.
//!
//! ## Example Usage
//!
//! ```rust
//! use stellar_toml::resolve;
//!
//! # async fn run() -> std::result::Result<(), stellar_toml::Error> {
//! let stellar = resolve("www.stellar.org").await?;
//! for validator in stellar.validators {
//!     println!("Name: {:?}, Host: {:?}", validator.display_name, validator.host);
//! }
//! # Ok(())
//! # }
//! ```

#[macro_use]
extern crate serde_derive;
use http::uri::Uri;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::result::Result;
use stellar_base::PublicKey;

/// The stellar.toml file is used to provide a common place where the Internet can find information about your organization’s Stellar integration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StellarToml {
    /// The version of SEP-1 your stellar.toml adheres to. This helps parsers know which fields to expect.
    #[serde(alias = "VERSION")]
    pub version: Option<String>,

    /// The passphrase for the specific Stellar network this infrastructure operates on.
    #[serde(alias = "NETWORK_PASSPHRASE")]
    pub network_passphrase: Option<String>,

    /// The endpoint for clients to resolve stellar addresses for users on your domain via SEP-2 Federation Protocol.
    #[serde(alias = "FEDERATION_SERVER")]
    #[serde(default, with = "option_display_fromstr")]
    pub federation_server: Option<Uri>,

    /// The endpoint used for SEP-3 Compliance Protocol.
    #[serde(alias = "AUTH_SERVER")]
    #[serde(default, with = "option_display_fromstr")]
    pub auth_server: Option<Uri>,

    /// The server used for SEP-6 Anchor/Client interoperability.
    #[serde(alias = "TRANSFER_SERVER")]
    #[serde(default, with = "option_display_fromstr")]
    pub transfer_server: Option<Uri>,

    /// The server used for SEP-24 Anchor/Client interoperability.
    #[serde(alias = "TRANSFER_SERVER_SEP0024")]
    #[serde(default, with = "option_display_fromstr")]
    pub transfer_server_sep0024: Option<Uri>,

    /// The server used for SEP-12 Anchor/Client customer info transfer.
    #[serde(alias = "KYC_SERVER")]
    #[serde(default, with = "option_display_fromstr")]
    pub kyc_server: Option<Uri>,

    /// The endpoint used for SEP-10 Web Authentication.
    #[serde(alias = "WEB_AUTH_ENDPOINT")]
    #[serde(default, with = "option_display_fromstr")]
    pub web_auth_endpoint: Option<Uri>,

    /// The signing key is used for SEP-3 Compliance Protocol and SEP-10 Authentication Protocol.
    #[serde(alias = "SIGNING_KEY")]
    #[serde(default, with = "option_display_fromstr")]
    pub signing_key: Option<PublicKey>,

    /// Location of public-facing Horizon instance (if you offer one)
    #[serde(alias = "HORIZON_URL")]
    #[serde(default, with = "option_display_fromstr")]
    pub horizon_url: Option<String>,

    /// A list of Stellar accounts that are controlled by this domain.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "ACCOUNTS")]
    pub accounts: Vec<String>,

    /// The signing key is used for SEP-7 delegated signing.
    #[serde(alias = "URI_REQUEST_SIGNING_KEY")]
    pub uri_request_signing_key: Option<String>,

    /// Information about the organization.
    #[serde(alias = "DOCUMENTATION")]
    pub documentation: Option<Documentation>,

    /// Information about the organization principals.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "PRINCIPALS")]
    pub principals: Vec<PointOfContact>,

    /// Information about supported currencies.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "CURRENCIES")]
    pub currencies: Vec<Currency>,

    /// Information about the organization validators.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "VALIDATORS")]
    pub validators: Vec<Validator>,
}

/// This section contains information about an organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Documentation {
    /// Legal name of your organization.
    #[serde(alias = "ORG_NAME")]
    pub org_name: Option<String>,

    /// (may not apply) DBA of your organization.
    #[serde(alias = "ORG_DBA")]
    pub org_dba: Option<String>,

    /// Your organization's official URL. Your stellar.toml must be hosted on the same domain.
    #[serde(alias = "ORG_URL")]
    #[serde(default, with = "option_display_fromstr")]
    pub org_url: Option<Uri>,

    /// A PNG image of your organization's logo on a transparent background.
    #[serde(alias = "ORG_LOGO")]
    #[serde(default, with = "option_display_fromstr")]
    pub org_logo: Option<Uri>,

    /// Short description of your organization.
    #[serde(alias = "ORG_DESCRIPTION")]
    pub org_description: Option<String>,

    /// Physical address for your organization.
    #[serde(alias = "ORG_PHYSICAL_ADDRESS")]
    pub org_physical_address: Option<String>,

    /// URL on the same domain as your `org_url` that contains an
    /// image or pdf official document attesting to your physical
    /// address.
    ///
    /// It must list your `org_name` or `org_dba` as the party at the
    /// address. Only documents from an official third party are
    /// acceptable. E.g. a utility bill, mail from a financial
    /// institution, or business license.
    #[serde(alias = "ORG_PHYSICAL_ADDRESS_ATTESTATION")]
    #[serde(default, with = "option_display_fromstr")]
    pub org_physical_address_attestation: Option<Uri>,

    /// Your organization's phone number in E.164 format, e.g. +14155552671.
    #[serde(alias = "ORG_PHONE_NUMBER")]
    pub org_phone_number: Option<String>,

    /// URL on the same domain as your `org_url` that contains an
    /// image or pdf of a phone bill showing both the phone number and
    /// your organization's name.
    #[serde(alias = "ORG_PHONE_NUMBER_ATTESTIATION")]
    #[serde(default, with = "option_display_fromstr")]
    pub org_phone_number_attestation: Option<Uri>,

    /// A Keybase account name for your organization.
    ///
    /// Should contain proof of ownership of any public online
    /// accounts you list here, including your organization's domain.
    #[serde(alias = "ORG_KEYBASE")]
    pub org_keybase: Option<String>,

    /// Your organization's Twitter account.
    #[serde(alias = "ORG_TWITTER")]
    pub org_twitter: Option<String>,

    /// Your organization's Github account.
    #[serde(alias = "ORG_GITHUB")]
    pub org_github: Option<String>,

    /// An email where clients can contact your organization. Must be hosted at your `org_url` domain.
    #[serde(alias = "ORG_OFFICIAL_EMAIL")]
    pub org_official_email: Option<String>,

    /// Name of the authority or agency that licensed your organization, if applicable.
    #[serde(alias = "ORG_LICENSING_AUTHORITY")]
    pub org_licensing_authority: Option<String>,

    /// Type of financial or other license your organization holds, if applicable.
    #[serde(alias = "ORG_LICENSE_TYPE")]
    pub org_license_type: Option<String>,

    /// Official license number of your organization, if applicable.
    #[serde(alias = "ORG_LICENSE_NUMBER")]
    pub org_license_number: Option<String>,
}

/// Contains identifying information for the primary point of contact
/// or principal(s) of the organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointOfContact {
    /// Full legal name.
    #[serde(alias = "NAME")]
    pub name: Option<String>,

    /// Business email address for the principal.
    #[serde(alias = "EMAIL")]
    pub email: Option<String>,

    /// Personal Keybase account.
    ///
    /// Should include proof of ownership for other online accounts,
    /// as well as the organization's domain.
    #[serde(alias = "KEYBASE")]
    pub keybase: Option<String>,

    /// Personal Telegram account.
    #[serde(alias = "TELEGRAM")]
    pub telegram: Option<String>,

    /// Personal Twitter account.
    #[serde(alias = "TWITTER")]
    pub twitter: Option<String>,

    /// Personal Github account.
    #[serde(alias = "GITHUB")]
    pub github: Option<String>,

    /// SHA-256 hash of a photo of the principal's government-issued photo ID.
    #[serde(alias = "ID_PHOTO_HASH")]
    pub id_photo_hash: Option<String>,

    /// SHA-256 hash of a verification photo of principal.
    ///
    /// Should be well-lit and contain: principal holding ID card and
    /// signed, dated, hand-written message stating `I, $NAME, am a
    /// principal of $ORG_NAME, a Stellar token issuer with address
    /// $ISSUER_ADDRESS`.
    #[serde(alias = "VERIFICATION_PHOTO_HASH")]
    pub verification_photo_hash: Option<String>,
}

/// Status of a token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CurrencyStatus {
    /// Token is live.
    #[serde(alias = "live")]
    Live,
    /// Token is dead.
    #[serde(alias = "dead")]
    Dead,
    /// Token is for testing.
    #[serde(alias = "test")]
    Test,
    /// Token is for private use.
    #[serde(alias = "private")]
    Private,
}

/// Type of asset anchored.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnchoredCurrencyType {
    /// Fiat currency, e.g. Euro.
    #[serde(alias = "fiat")]
    Fiat,
    /// Cyrpto currency, e.g. Ethereum.
    #[serde(alias = "crypto")]
    Crypto,
    /// Stock.
    #[serde(alias = "stock")]
    Stock,
    /// Bond.
    #[serde(alias = "bond")]
    Bond,
    /// Commodity.
    #[serde(alias = "commodity")]
    Commodity,
    /// Real Estate.
    #[serde(alias = "realestate")]
    RealEstate,
    /// Other.
    #[serde(alias = "other")]
    Other,
}

/// Contains information about a currency supported by the organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    /// Token code.
    #[serde(alias = "CODE")]
    pub code: Option<String>,

    /// A pattern with `?` as a single character wildcard.
    ///
    /// Allows a `Currency` entry to apply to multiple assets that
    /// share the same info. An example is futures, where the only
    /// difference between issues is the date of the
    /// contract. E.g. `CORN????????` to match codes such as
    /// `CORN20180604`.
    #[serde(alias = "CODE_TEMPLATE")]
    pub code_template: Option<String>,

    /// Token issuer Stellar public key.
    #[serde(alias = "ISSUER")]
    #[serde(with = "option_display_fromstr")]
    pub issuer: Option<PublicKey>,

    ///Status of token.
    ///
    /// Allows issuer to mark whether token is dead/for testing/for
    /// private use or is live and should be listed in live exchanges.
    #[serde(alias = "STATUS")]
    pub status: Option<CurrencyStatus>,

    /// Preference for number of decimals to show when a client displays currency balance.
    #[serde(alias = "DISPLAY_DECIMALS")]
    pub display_decimals: Option<u8>,

    /// A short name for the token.
    #[serde(alias = "NAME")]
    pub name: Option<String>,

    /// Description of token and what it represents.
    #[serde(rename = "desc")]
    #[serde(alias = "DESC")]
    pub description: Option<String>,

    /// Conditions on token.
    #[serde(alias = "CONDITIONS")]
    pub conditions: Option<String>,

    /// URL to a PNG image on a transparent background representing token.
    #[serde(alias = "IMAGE")]
    pub image: Option<String>,

    /// Fixed number of tokens, if the number of tokens issued will never change.
    #[serde(alias = "FIXED_NUMBER")]
    pub fixed_number: Option<i64>,

    /// Max number of tokens, if there will never be more than `max_number` tokens.
    #[serde(alias = "MAX_NUMBER")]
    pub max_number: Option<i64>,

    /// The number of tokens is dilutable at the issuer's discretion.
    #[serde(alias = "IS_UNLIMITED")]
    pub is_unlimited: Option<bool>,

    /// `true` if token can be redeemed for underlying asset, otherwise `false`.
    #[serde(alias = "IS_ASSET_ANCHORED")]
    pub is_asset_anchored: Option<bool>,

    /// Type of asset anchored.
    #[serde(alias = "ANCHOR_ASSET_TYPE")]
    pub anchor_asset_type: Option<AnchoredCurrencyType>,

    /// If anchored token, code / symbol for asset that token is anchored to.
    ///
    /// For example, USD, BTC, SBUX, Address of real-estate investment property.
    #[serde(alias = "ANCHOR_ASSET")]
    pub anchor_asset: Option<String>,

    /// If anchored token, these are instructions to redeem the underlying asset from tokens.
    #[serde(alias = "REDEMPTION_INSTRUCTIONS")]
    pub redemption_instructions: Option<String>,

    ///  this is an anchored crypto token, list of one or more public
    ///  addresses that hold the assets for which you are issuing
    ///  tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "COLLATERAL_ADDRESSES")]
    pub collateral_addresses: Vec<String>,

    /// Messages stating that funds in the `collateral_addresses` list
    /// are reserved to back the issued asset.
    ///
    /// The message to put in this field for each entry in
    /// `collateral_addresses`: ``` The assets in the account
    /// $PUBLIC_KEY are reserved to back $CODE issued by
    /// $ISSUER_ADDRESS on Stellar. Valid from $START to $END.  ```
    /// Replace `$PUBLIC_KEY` with the account's public key, `$CODE`
    /// with your asset code, `$ISSUER_ADDRESS` with the issuing
    /// Stellar address and `$START`, `$END` with the date range in
    /// ISO 8601 for which the reserve is valid. `$END` cannot be more
    /// than a year in the future to ensure yearly renewals of the
    /// commitment. `collateral_addresses` can be used to externally
    /// validate that you hold a reserve for the crypto funds you are
    /// issuing on Stellar. Issuers that hold a provable 100% reserve
    /// are prioritized by wallets and clients. Issuers not meeting
    /// this standard may not be listed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "COLLATERAL_ADDRESS_MESSAGES")]
    pub collateral_address_messages: Vec<String>,

    /// These prove you control the `collateral_addresses`.
    ///
    /// For each address you list, sign the entry in
    /// `collateral_address_messages` with the address's private key
    /// and add the resulting string to this list as a base64-encoded
    /// raw signature.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[serde(alias = "COLLATERAL_ADDRESS_SIGNATURES")]
    pub collateral_address_signatures: Vec<String>,

    #[serde(alias = "REGULATED")]
    /// Indicates whether or not this is a SEP-0008 regulated asset. If missing, false is assumed.
    pub regulated: Option<bool>,

    /// Url of a SEP-0008 compliant approval service that signs validated transactions.
    #[serde(alias = "APPROVAL_SERVER")]
    #[serde(default, with = "option_display_fromstr")]
    pub approval_server: Option<Uri>,

    /// A human readable string that explains the issuer's requirements for approving transactions.
    #[serde(alias = "APPROVAL_CRITERIA")]
    pub approval_criteria: Option<String>,
}

/// Information about an organization validator node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator {
    /// A name for display in stellar-core configs that conforms to `^[a-z0-9-]{2,16}$`.
    #[serde(alias = "ALIAS")]
    pub alias: Option<String>,

    /// A human-readable name for display in quorum explorers and other interfaces.
    #[serde(alias = "DISPLAY_NAME")]
    pub display_name: Option<String>,

    /// The Stellar account associated with the node.
    #[serde(alias = "PUBLIC_KEY")]
    #[serde(default, with = "option_display_fromstr")]
    pub public_key: Option<PublicKey>,

    /// The IP:port or domain:port peers can use to connect to the node.
    #[serde(alias = "HOST")]
    pub host: Option<String>,

    /// The location of the history archive published by this validator.
    #[serde(alias = "HISTORY")]
    #[serde(default, with = "option_display_fromstr")]
    pub history: Option<Uri>,
}

/// `stellar.toml` path.
pub const STELLAR_TOML_PATH: &str = ".well-known/stellar.toml";

/// Returns a parsed `stellar.toml` file at `domain`.
///
/// This function will always fetch the `stellar.toml` file using https.
/// If you need to fetch the file using http, you should build the
/// uri using `stellar_toml_path_insecure` and then call `resolve_url`.
/// Note, however, that this is not recommended.
pub async fn resolve(domain: &str) -> Result<StellarToml, Error> {
    let url = stellar_toml_path(&domain)?;
    resolve_url(&url).await
}

/// Returns a parsed `stellar.toml` file at `domain`.
pub async fn resolve_url(url: &Uri) -> Result<StellarToml, Error> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri: hyper::Uri = url.to_string().parse()?;
    let response = client.get(uri).await?;

    if response.status().is_success() {
        let bytes = hyper::body::to_bytes(response).await?;
        let result: StellarToml = toml::from_slice(&bytes)?;
        Ok(result)
    } else if response.status().is_client_error() {
        Err(Error::ClientError(response))
    } else {
        Err(Error::ServerError(response))
    }
}

/// Returns the https uri to the `stellar.toml` file at `domain`.
pub fn stellar_toml_path(domain: &str) -> Result<Uri, Error> {
    let url = format!("https://{}/{}", domain, STELLAR_TOML_PATH);
    Ok(url.parse()?)
}

/// Returns the http uri to the `stellar.toml` file at `domain`.
///
/// Http is not secure as should not be used in production.
pub fn stellar_toml_path_insecure(domain: &str) -> Result<Uri, Error> {
    let url = format!("http://{}/{}", domain, STELLAR_TOML_PATH);
    Ok(url.parse()?)
}

/// Crate error type.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The client sent a bad request.
    #[error("client response error")]
    ClientError(hyper::Response<hyper::Body>),
    /// Server error response.
    #[error("server response error")]
    ServerError(hyper::Response<hyper::Body>),
    /// Toml was not a valid `stellar.toml` file.
    #[error("toml parse error")]
    TomlParseError(#[from] toml::de::Error),
    /// Http error.
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    /// Invalid url format.
    #[error("invalid uri")]
    InvalidUri(#[from] http::uri::InvalidUri),
}

mod option_display_fromstr {
    use serde::{
        de::{Deserialize, Deserializer},
        ser::Serializer,
    };
    use serde_with::rust::display_fromstr;
    use std::{fmt::Display, str::FromStr};

    #[derive(Deserialize)]
    struct DeWrapper<T>(#[serde(with = "display_fromstr")] T)
    where
        T: FromStr,
        T::Err: Display;

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
        T::Err: Display,
    {
        let v: Option<DeWrapper<T>> = Option::deserialize(deserializer)?;
        Ok(v.map(|DeWrapper(a)| a))
    }

    pub fn serialize<S, T>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        match value {
            None => serializer.serialize_unit(),
            Some(v) => serializer.serialize_str(&*v.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stellar_toml_path() {
        let url = stellar_toml_path("foo.bar.example.org").unwrap();
        assert_eq!(
            "https://foo.bar.example.org/.well-known/stellar.toml",
            url.to_string()
        );
    }

    #[test]
    fn test_stellar_toml_path_insecure() {
        let url = stellar_toml_path_insecure("foo.bar.example.org").unwrap();
        assert_eq!(
            "http://foo.bar.example.org/.well-known/stellar.toml",
            url.to_string()
        );
    }

    #[tokio::test]
    async fn test_resolve_known_stellar_tomls() {
        let stellar = resolve("www.stellar.org").await.unwrap();
        assert!(!stellar.validators.is_empty());
        let usd = resolve("stablecoin.anchorusd.com").await.unwrap();
        assert!(!usd.currencies.is_empty());
    }
}
