// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::PersonId;
use crate::params::{Expandable, Metadata, Object, Timestamp};
use crate::resources::{Address, Dob, File};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "Person".
///
/// For more details see [https://stripe.com/docs/api/persons/object](https://stripe.com/docs/api/persons/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Person {
    /// Unique identifier for the object.
    pub id: PersonId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kana: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_kanji: Option<Address>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<Timestamp>,

    // Always true for a deleted object
    #[serde(default)]
    pub deleted: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<Dob>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kana: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_kanji: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maiden_name: Option<String>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship: Option<PersonRelationship>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<PersonRequirements>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn_last_4_provided: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<PersonVerification>,
}

impl Object for Person {
    type Id = PersonId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "person"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerification {
    /// A document showing address, either a passport, local ID card, or utility bill from a well-known utility company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_document: Option<PersonVerificationDocument>,

    /// A user-displayable string describing the verification state for the person.
    ///
    /// For example, this may say "Provided identity information could not be verified".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// One of `document_address_mismatch`, `document_dob_mismatch`, `document_duplicate_type`, `document_id_number_mismatch`, `document_name_mismatch`, `document_nationality_mismatch`, `failed_keyed_identity`, or `failed_other`.
    ///
    /// A machine-readable code specifying the verification state for the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PersonVerificationDocument>,

    /// The state of verification for the person.
    ///
    /// Possible values are `unverified`, `pending`, or `verified`.
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonVerificationDocument {
    /// The back of an ID returned by a [file upload](#create_file) with a `purpose` value of `identity_document`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub back: Option<Expandable<File>>,

    /// A user-displayable string describing the verification state of this document.
    ///
    /// For example, if a document is uploaded and the picture is too fuzzy, this may say "Identity document is too unclear to read".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    /// One of `document_corrupt`, `document_country_not_supported`, `document_expired`, `document_failed_copy`, `document_failed_other`, `document_failed_test_mode`, `document_fraudulent`, `document_failed_greyscale`, `document_incomplete`, `document_invalid`, `document_manipulated`, `document_missing_back`, `document_missing_front`, `document_not_readable`, `document_not_uploaded`, `document_photo_mismatch`, `document_too_large`, or `document_type_not_supported`.
    ///
    /// A machine-readable code specifying the verification state for this document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_code: Option<String>,

    /// The front of an ID returned by a [file upload](#create_file) with a `purpose` value of `identity_document`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front: Option<Expandable<File>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonRelationship {
    /// Whether the person is a director of the account's legal entity.
    ///
    /// Currently only required for accounts in the EU.
    /// Directors are typically members of the governing board of the company, or responsible for ensuring the company meets its regulatory obligations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub director: Option<bool>,

    /// Whether the person has significant responsibility to control, manage, or direct the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executive: Option<bool>,

    /// Whether the person is an owner of the account’s legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,

    /// The percent owned by the person of the account's legal entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_ownership: Option<f64>,

    /// Whether the person is authorized as the primary representative of the account.
    ///
    /// This is the person nominated by the business to provide information about themselves, and general information about the account.
    /// There can only be one representative at any given time.
    /// At the time the account is created, this person should be set to the person responsible for opening the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representative: Option<bool>,

    /// The person's title (e.g., CEO, Support Engineer).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PersonRequirements {
    /// Fields that need to be collected to keep the person's account enabled.
    ///
    /// If not collected by the account's `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    pub currently_due: Vec<String>,

    /// Fields that need to be collected assuming all volume thresholds are reached.
    ///
    /// As fields are needed, they are moved to `currently_due` and the account's `current_deadline` is set.
    pub eventually_due: Vec<String>,

    /// Fields that weren't collected by the account's `current_deadline`.
    ///
    /// These fields need to be collected to enable payouts for the person's account.
    pub past_due: Vec<String>,

    /// Fields that may become required depending on the results of verification or review.
    ///
    /// An empty array unless an asynchronous verification is pending.
    /// If verification fails, the fields in this array become required and move to `currently_due` or `past_due`.
    pub pending_verification: Vec<String>,
}
