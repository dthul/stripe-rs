// ======================================
// This file was automatically generated.
// ======================================

use crate::config::{Client, Response};
use crate::ids::{CustomerId, SubscriptionScheduleId};
use crate::params::{Expand, Expandable, List, Metadata, Object, RangeQuery, Timestamp};
use crate::resources::{
    CollectionMethod, Coupon, Customer, PaymentMethod, Plan, Scheduled, Subscription,
    SubscriptionBillingThresholds, SubscriptionItemBillingThresholds, TaxRate,
};
use serde_derive::{Deserialize, Serialize};

/// The resource representing a Stripe "SubscriptionSchedule".
///
/// For more details see [https://stripe.com/docs/api/subscription_schedules/object](https://stripe.com/docs/api/subscription_schedules/object).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedule {
    /// Unique identifier for the object.
    pub id: SubscriptionScheduleId,

    /// Time at which the subscription schedule was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<Timestamp>,

    /// Time at which the subscription schedule was completed.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<Timestamp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: Timestamp,

    /// Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<SubscriptionScheduleCurrentPhase>,

    /// ID of the customer who owns the subscription schedule.
    pub customer: Expandable<Customer>,

    pub default_settings: SubscriptionSchedulesResourceDefaultSettings,

    /// Behavior of the subscription schedule and underlying subscription when it ends.
    pub end_behavior: SubscriptionScheduleEndBehavior,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(default)]
    pub metadata: Metadata,

    /// Configuration for the subscription schedule's phases.
    pub phases: Vec<SubscriptionSchedulePhaseConfiguration>,

    /// Time at which the subscription schedule was released.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<Timestamp>,

    /// ID of the subscription once managed by the subscription schedule (if it is released).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_subscription: Option<String>,

    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    pub status: SubscriptionScheduleStatus,

    /// ID of the subscription managed by the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Expandable<Subscription>>,
}

impl SubscriptionSchedule {
    /// Retrieves the list of your subscription schedules.
    pub fn list(
        client: &Client,
        params: ListSubscriptionSchedules<'_>,
    ) -> Response<List<SubscriptionSchedule>> {
        client.get_query("/subscription_schedules", &params)
    }

    /// Creates a new subscription schedule object.
    pub fn create(
        client: &Client,
        params: CreateSubscriptionSchedule<'_>,
    ) -> Response<SubscriptionSchedule> {
        client.post_form("/subscription_schedules", &params)
    }

    /// Retrieves the details of an existing subscription schedule.
    ///
    /// You only need to supply the unique subscription schedule identifier that was returned upon subscription schedule creation.
    pub fn retrieve(
        client: &Client,
        id: &SubscriptionScheduleId,
        expand: &[&str],
    ) -> Response<SubscriptionSchedule> {
        client.get_query(&format!("/subscription_schedules/{}", id), &Expand { expand })
    }

    /// Updates an existing subscription schedule.
    pub fn update(
        client: &Client,
        id: &SubscriptionScheduleId,
        params: UpdateSubscriptionSchedule<'_>,
    ) -> Response<SubscriptionSchedule> {
        client.post_form(&format!("/subscription_schedules/{}", id), &params)
    }
}

impl Object for SubscriptionSchedule {
    type Id = SubscriptionScheduleId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "subscription_schedule"
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleCurrentPhase {
    pub end_date: Timestamp,

    pub start_date: Timestamp,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedulePhaseConfiguration {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account during this phase of the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    /// ID of the coupon to use during this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<Expandable<Coupon>>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<TaxRate>>,

    /// The end of this phase of the subscription schedule.
    pub end_date: Timestamp,

    /// The subscription schedule's default invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    /// Plans to subscribe during this phase of the subscription schedule.
    pub plans: Vec<SubscriptionScheduleConfigurationItem>,

    /// The start of this phase of the subscription schedule.
    pub start_date: Timestamp,

    /// If provided, each invoice created during this phase of the subscription schedule will apply the tax rate, increasing the amount billed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    /// When the trial ends within the phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleConfigurationItem {
    /// Define thresholds at which an invoice will be sent, and the related subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    /// ID of the plan to which the customer should be subscribed.
    pub plan: Expandable<Plan>,

    /// Quantity of the plan to which the customer should be subscribed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to this `phase_item`.
    ///
    /// When set, the `default_tax_rates` on the phase do not apply to this `phase_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedulesResourceDefaultSettings {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<SubscriptionSchedulesResourceDefaultSettingsCollectionMethod>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Expandable<PaymentMethod>>,

    /// The subscription schedule's default invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,
}

/// The parameters for `SubscriptionSchedule::create`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct CreateSubscriptionSchedule<'a> {
    /// The identifier of the customer to create the subscription schedule for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<CreateSubscriptionScheduleDefaultSettings>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<SubscriptionScheduleEndBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Migrate an existing subscription to be managed by a subscription schedule.
    ///
    /// If this parameter is set, a subscription schedule will be created using the subscription's plan(s), set to auto-renew using the subscription's interval.
    /// Other parameters cannot be set since their values will be inferred from the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_subscription: Option<&'a str>,

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<CreateSubscriptionSchedulePhases>>,

    /// The date at which the subscription schedule starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Scheduled>,
}

impl<'a> CreateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        CreateSubscriptionSchedule {
            customer: Default::default(),
            default_settings: Default::default(),
            end_behavior: Default::default(),
            expand: Default::default(),
            from_subscription: Default::default(),
            metadata: Default::default(),
            phases: Default::default(),
            start_date: Default::default(),
        }
    }
}

/// The parameters for `SubscriptionSchedule::list`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct ListSubscriptionSchedules<'a> {
    /// Only return subscription schedules that were created canceled the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules that completed during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules that were created during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules for the given customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<CustomerId>,

    /// A cursor for use in pagination.
    ///
    /// `ending_before` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, starting with `obj_bar`, your subsequent call can include `ending_before=obj_bar` in order to fetch the previous page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<SubscriptionScheduleId>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// A limit on the number of objects to be returned.
    ///
    /// Limit can range between 1 and 100, and the default is 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,

    /// Only return subscription schedules that were released during the given date interval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<RangeQuery<Timestamp>>,

    /// Only return subscription schedules that have not started yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<bool>,

    /// A cursor for use in pagination.
    ///
    /// `starting_after` is an object ID that defines your place in the list.
    /// For instance, if you make a list request and receive 100 objects, ending with `obj_foo`, your subsequent call can include `starting_after=obj_foo` in order to fetch the next page of the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<SubscriptionScheduleId>,
}

impl<'a> ListSubscriptionSchedules<'a> {
    pub fn new() -> Self {
        ListSubscriptionSchedules {
            canceled_at: Default::default(),
            completed_at: Default::default(),
            created: Default::default(),
            customer: Default::default(),
            ending_before: Default::default(),
            expand: Default::default(),
            limit: Default::default(),
            released_at: Default::default(),
            scheduled: Default::default(),
            starting_after: Default::default(),
        }
    }
}

/// The parameters for `SubscriptionSchedule::update`.
#[derive(Clone, Debug, Serialize, Default)]
pub struct UpdateSubscriptionSchedule<'a> {
    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<UpdateSubscriptionScheduleDefaultSettings>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<SubscriptionScheduleEndBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Expand::is_empty")]
    pub expand: &'a [&'a str],

    /// Set of key-value pairs that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    /// Note that past phases can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<UpdateSubscriptionSchedulePhases>>,

    /// If the update changes the current phase, indicates if the changes should be prorated.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}

impl<'a> UpdateSubscriptionSchedule<'a> {
    pub fn new() -> Self {
        UpdateSubscriptionSchedule {
            default_settings: Default::default(),
            end_behavior: Default::default(),
            expand: Default::default(),
            metadata: Default::default(),
            phases: Default::default(),
            prorate: Default::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<CreateSubscriptionScheduleDefaultSettingsBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CreateSubscriptionScheduleDefaultSettingsCollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CreateSubscriptionScheduleDefaultSettingsInvoiceSettings>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    pub plans: Vec<SubscriptionSchedulePhasesPlansParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<UpdateSubscriptionScheduleDefaultSettingsBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<UpdateSubscriptionScheduleDefaultSettingsCollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionSchedulePhases {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionBillingThresholds>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<CollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<SubscriptionScheduleInvoiceSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    pub plans: Vec<SubscriptionSchedulePhasesPlansParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<Scheduled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<Scheduled>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsBillingThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionScheduleInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubscriptionSchedulePhasesPlansParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<SubscriptionItemBillingThresholds>,

    pub plan: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsBillingThresholds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateSubscriptionScheduleDefaultSettingsInvoiceSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

/// An enum representing the possible values of an `CreateSubscriptionScheduleDefaultSettings`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            CreateSubscriptionScheduleDefaultSettingsCollectionMethod::ChargeAutomatically => {
                "charge_automatically"
            }
            CreateSubscriptionScheduleDefaultSettingsCollectionMethod::SendInvoice => {
                "send_invoice"
            }
        }
    }
}

impl AsRef<str> for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SubscriptionSchedule`'s `end_behavior` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl SubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionScheduleEndBehavior::Cancel => "cancel",
            SubscriptionScheduleEndBehavior::None => "none",
            SubscriptionScheduleEndBehavior::Release => "release",
            SubscriptionScheduleEndBehavior::Renew => "renew",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SubscriptionSchedule`'s `status` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
}

impl SubscriptionScheduleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionScheduleStatus::Active => "active",
            SubscriptionScheduleStatus::Canceled => "canceled",
            SubscriptionScheduleStatus::Completed => "completed",
            SubscriptionScheduleStatus::NotStarted => "not_started",
            SubscriptionScheduleStatus::Released => "released",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `SubscriptionSchedulesResourceDefaultSettings`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            SubscriptionSchedulesResourceDefaultSettingsCollectionMethod::ChargeAutomatically => {
                "charge_automatically"
            }
            SubscriptionSchedulesResourceDefaultSettingsCollectionMethod::SendInvoice => {
                "send_invoice"
            }
        }
    }
}

impl AsRef<str> for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionSchedulesResourceDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

/// An enum representing the possible values of an `UpdateSubscriptionScheduleDefaultSettings`'s `collection_method` field.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            UpdateSubscriptionScheduleDefaultSettingsCollectionMethod::ChargeAutomatically => {
                "charge_automatically"
            }
            UpdateSubscriptionScheduleDefaultSettingsCollectionMethod::SendInvoice => {
                "send_invoice"
            }
        }
    }
}

impl AsRef<str> for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UpdateSubscriptionScheduleDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
