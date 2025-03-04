use serde::{Deserialize, Serialize};

/// This object represents an error in the Telegram Passport element which was
/// submitted that should be resolved by the user.
///
/// [The official docs](https://core.telegram.org/bots/api#passportelementerror).
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct PassportElementError {
    /// Error message.
    message: String,

    #[serde(flatten)]
    kind: PassportElementErrorKind,
}

// TODO: use different types?
#[serde(tag = "source")]
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum PassportElementErrorKind {
    /// Represents an issue in one of the data fields that was provided by the
    /// user. The error is considered resolved when the field's value changes.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrordatafield).
    #[serde(rename = "data")]
    DataField {
        /// The section of the user's Telegram Passport which has the error.
        r#type: PassportElementErrorDataFieldType,

        /// Name of the data field which has the error.
        field_name: String,

        /// Base64-encoded data hash.
        data_hash: String,
    },

    /// Represents an issue with the front side of a document. The error is
    /// considered resolved when the file with the front side of the document
    /// changes.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfrontside).
    #[serde(rename = "snake_case")]
    FrontSide {
        /// The section of the user's Telegram Passport which has the issue.
        r#type: PassportElementErrorFrontSideType,

        /// Base64-encoded hash of the file with the front side of the
        /// document.
        file_hash: String,
    },

    /// Represents an issue with the reverse side of a document. The error is
    /// considered resolved when the file with reverse side of the document
    /// changes.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrorreverseside).
    #[serde(rename = "snake_case")]
    ReverseSide {
        /// The section of the user's Telegram Passport which has the issue.
        r#type: PassportElementErrorReverseSideType,

        //// Base64-encoded hash of the file with the reverse side of the
        //// document.
        file_hash: String,
    },

    //// Represents an issue with the selfie with a document. The error is
    //// considered resolved when the file with the selfie changes.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrorselfie).
    #[serde(rename = "snake_case")]
    Selfie {
        /// The section of the user's Telegram Passport which has the issue.
        r#type: PassportElementErrorSelfieType,

        /// Base64-encoded hash of the file with the selfie.
        file_hash: String,
    },

    /// Represents an issue with a document scan. The error is considered
    /// resolved when the file with the document scan changes.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfile).
    #[serde(rename = "snake_case")]
    File {
        /// The section of the user's Telegram Passport which has the issue.
        r#type: PassportElementErrorFileType,

        /// Base64-encoded file hash.
        file_hash: String,
    },

    /// Represents an issue with a list of scans. The error is considered
    /// resolved when the list of files containing the scans changes.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrorfiles).
    #[serde(rename = "snake_case")]
    Files {
        /// The section of the user's Telegram Passport which has the issue.
        r#type: PassportElementErrorFilesType,

        /// List of base64-encoded file hashes.
        file_hashes: Vec<String>,
    },

    /// Represents an issue with one of the files that constitute the
    /// translation of a document. The error is considered resolved when the
    /// file changes.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfile).
    #[serde(rename = "snake_case")]
    TranslationFile {
        /// Type of element of the user's Telegram Passport which has the
        /// issue.
        r#type: PassportElementErrorTranslationFileType,

        /// Base64-encoded file hash.
        file_hash: String,
    },

    /// Represents an issue with the translated version of a document. The
    /// error is considered resolved when a file with the document translation
    /// change.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrortranslationfiles).
    #[serde(rename = "snake_case")]
    TranslationFiles {
        /// Type of element of the user's Telegram Passport which has the issue
        r#type: PassportElementErrorTranslationFilesType,

        /// List of base64-encoded file hashes
        file_hashes: Vec<String>,
    },

    /// Represents an issue in an unspecified place. The error is considered
    /// resolved when new data is added.
    ///
    /// [The official docs](https://core.telegram.org/bots/api#passportelementerrorunspecified).
    #[serde(rename = "snake_case")]
    Unspecified {
        /// Type of element of the user's Telegram Passport which has the
        /// issue.
        r#type: PassportElementErrorUnspecifiedType,

        /// Base64-encoded element hash.
        element_hash: String,
    },
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorDataFieldType {
    PersonalDetails,
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    Address,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFrontSideType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorReverseSideType {
    DriverLicense,
    IdentityCard,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorSelfieType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFileType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorFilesType {
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorTranslationFileType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorTranslationFilesType {
    Passport,
    DriverLicense,
    IdentityCard,
    InternalPassport,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PassportElementErrorUnspecifiedType {
    DataField,
    FrontSide,
    ReverseSide,
    Selfie,
    File,
    Files,
    TranslationFile,
    TranslationFiles,
    Unspecified,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_data_field() {
        let data = PassportElementError {
            message: "This is an error message!".to_owned(),
            kind: PassportElementErrorKind::DataField {
                r#type: PassportElementErrorDataFieldType::InternalPassport,
                field_name: "The field name".to_owned(),
                data_hash: "This is a data hash".to_owned(),
            },
        };

        assert_eq!(
            serde_json::to_string(&data).unwrap(),
            r#"{"message":"This is an error message!","source":"data","type":"internal_passport","field_name":"The field name","data_hash":"This is a data hash"}"#
        );
    }
}
