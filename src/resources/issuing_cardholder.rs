// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingCardholderId;
use crate::params::{Metadata, Object, Timestamp};
use crate::resources::{Address, Currency, MerchantCategory, SpendingLimit};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingCardholder".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholder {
    /// Unique identifier for the object.
    pub id: IssuingCardholderId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_controls: Option<IssuingCardholderAuthorizationControls>,

    pub billing: IssuingCardholderAddress,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// The cardholder's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Whether or not this cardholder is the default cardholder.
    pub is_default: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Metadata,

    /// The cardholder's name.
    ///
    /// This will be printed on cards issued to them.
    pub name: String,

    /// The cardholder's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    pub requirements: IssuingCardholderRequirements,

    /// One of `active`, `inactive`, or `blocked`.
    pub status: IssuingCardholderStatus,

    /// One of `individual` or `business_entity`.
    #[serde(rename = "type")]
    pub type_: IssuingCardholderType,
}

impl Object for IssuingCardholder {
    type Id = IssuingCardholderId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.cardholder"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderAddress {
    pub address: Address,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderAuthorizationControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations permitted on this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<MerchantCategory>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to always decline on this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<MerchantCategory>>,

    /// Limit the spending with rules based on time intervals and categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<SpendingLimit>>,

    /// Currency for the amounts within spending_limits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits_currency: Option<Currency>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingCardholderRequirements {
    /// If the cardholder is disabled, this string describes why.
    ///
    /// Can be one of `listed`, `rejected.listed`, or `under_review`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<IssuingCardholderRequirementsDisabledReason>,

    /// If not empty, this field contains the list of fields that need to be collected in order to verify and re-enabled the cardholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<IssuingCardholderRequirementsPastDue>>,
}

/// An enum representing the possible values of an `IssuingCardholderRequirements`'s `disabled_reason` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    #[serde(rename = "rejected.listed")]
    RejectedListed,
    UnderReview,
}

impl IssuingCardholderRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderRequirementsDisabledReason::Listed => "listed",
            IssuingCardholderRequirementsDisabledReason::RejectedListed => "rejected.listed",
            IssuingCardholderRequirementsDisabledReason::UnderReview => "under_review",
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardholderRequirements`'s `past_due` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsPastDue {
    #[serde(rename = "individual.dob.day")]
    IndividualDobDay,
    #[serde(rename = "individual.dob.month")]
    IndividualDobMonth,
    #[serde(rename = "individual.dob.year")]
    IndividualDobYear,
    #[serde(rename = "individual.first_name")]
    IndividualFirstName,
    #[serde(rename = "individual.last_name")]
    IndividualLastName,
    #[serde(rename = "individual.verification.document")]
    IndividualVerificationDocument,
}

impl IssuingCardholderRequirementsPastDue {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderRequirementsPastDue::IndividualDobDay => "individual.dob.day",
            IssuingCardholderRequirementsPastDue::IndividualDobMonth => "individual.dob.month",
            IssuingCardholderRequirementsPastDue::IndividualDobYear => "individual.dob.year",
            IssuingCardholderRequirementsPastDue::IndividualFirstName => "individual.first_name",
            IssuingCardholderRequirementsPastDue::IndividualLastName => "individual.last_name",
            IssuingCardholderRequirementsPastDue::IndividualVerificationDocument => {
                "individual.verification.document"
            }
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsPastDue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardholder`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderStatus {
    Active,
    Blocked,
    Inactive,
    Pending,
}

impl IssuingCardholderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderStatus::Active => "active",
            IssuingCardholderStatus::Blocked => "blocked",
            IssuingCardholderStatus::Inactive => "inactive",
            IssuingCardholderStatus::Pending => "pending",
        }
    }
}

impl AsRef<str> for IssuingCardholderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `IssuingCardholder`'s `type` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderType {
    BusinessEntity,
    Individual,
}

impl IssuingCardholderType {
    pub fn as_str(self) -> &'static str {
        match self {
            IssuingCardholderType::BusinessEntity => "business_entity",
            IssuingCardholderType::Individual => "individual",
        }
    }
}

impl AsRef<str> for IssuingCardholderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
