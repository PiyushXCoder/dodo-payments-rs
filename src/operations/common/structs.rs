use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerInfo {
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillingAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub country: CountryCodeAlpha2,
    pub zipcode: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductItem {
    pub product_id: String,
    pub quantity: u32,
    pub amount: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodTypes {
    Credit,
    Debit,
    UpiCollect,
    UpiIntent,
    ApplePay,
    Cashapp,
    GooglePay,
    Multibanco,
    BancontactCard,
    Eps,
    Ideal,
    Przelewy24,
    Affirm,
    Klarna,
    Sepa,
    Ach,
    AmazonPay,
    AfterpayClearpay,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStage {
    PreDispute,
    Dispute,
    PreArbitration,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    DisputeOpened,
    DisputeExpired,
    DisputeAccepted,
    DisputeCancelled,
    DisputeChallenged,
    DisputeWon,
    DisputeLost,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IntentStatus {
    Succeeded,
    Failed,
    Cancelled,
    Processing,
    RequiresCustomerAction,
    RequiresMerchantAction,
    RequiresPaymentMethod,
    RequiresConfirmation,
    RequiresCapture,
    PartiallyCaptured,
    PartiallyCapturedAndCapturable,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    Succeeded,
    Failed,
    Pending,
    Review,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    Aed,
    All,
    Amd,
    Ang,
    Aoa,
    Ars,
    Aud,
    Awg,
    Azn,
    Bam,
    Bbd,
    Bdt,
    Bgn,
    Bhd,
    Bif,
    Bmd,
    Bnd,
    Bob,
    Brl,
    Bsd,
    Bwp,
    Byn,
    Bzd,
    Cad,
    Chf,
    Clp,
    Cny,
    Cop,
    Crc,
    Cup,
    Cve,
    Czk,
    Djf,
    Dkk,
    Dop,
    Dzd,
    Egp,
    Etb,
    Eur,
    Fjd,
    Fkp,
    Gbp,
    Gel,
    Ghs,
    Gip,
    Gmd,
    Gnf,
    Gtq,
    Gyd,
    Hkd,
    Hnl,
    Hrk,
    Htg,
    Huf,
    Idr,
    Ils,
    Inr,
    Iqd,
    Jmd,
    Jod,
    Jpy,
    Kes,
    Kgs,
    Khr,
    Kmf,
    Krw,
    Kwd,
    Kyd,
    Kzt,
    Lak,
    Lbp,
    Lkr,
    Lrd,
    Lsl,
    Lyd,
    Mad,
    Mdl,
    Mga,
    Mkd,
    Mmk,
    Mnt,
    Mop,
    Mru,
    Mur,
    Mvr,
    Mwk,
    Mxn,
    Myr,
    Mzn,
    Nad,
    Ngn,
    Nio,
    Nok,
    Npr,
    Nzd,
    Omr,
    Pab,
    Pen,
    Pgk,
    Php,
    Pkr,
    Pln,
    Pyg,
    Qar,
    Ron,
    Rsd,
    Rub,
    Rwf,
    Sar,
    Sbd,
    Scr,
    Sek,
    Sgd,
    Shp,
    Sle,
    Sll,
    Sos,
    Srd,
    Ssp,
    Stn,
    Svc,
    Szl,
    Thb,
    Tnd,
    Top,
    Try,
    Ttd,
    Twd,
    Tzs,
    Uah,
    Ugx,
    Usd,
    Uyu,
    Uzs,
    Ves,
    Vnd,
    Vuv,
    Wst,
    Xaf,
    Xcd,
    Xof,
    Xpf,
    Yer,
    Zar,
    Zmw,
}

#[derive(Debug, Serialize, Deserialize, Clone, Display)]
pub enum CountryCodeAlpha2 {
    AF,
    AX,
    AL,
    DZ,
    AS,
    AD,
    AO,
    AI,
    AQ,
    AG,
    AR,
    AM,
    AW,
    AU,
    AT,
    AZ,
    BS,
    BH,
    BD,
    BB,
    BY,
    BE,
    BZ,
    BJ,
    BM,
    BT,
    BO,
    BQ,
    BA,
    BW,
    BV,
    BR,
    IO,
    BN,
    BG,
    BF,
    BI,
    KH,
    CM,
    CA,
    CV,
    KY,
    CF,
    TD,
    CL,
    CN,
    CX,
    CC,
    CO,
    KM,
    CG,
    CD,
    CK,
    CR,
    CI,
    HR,
    CU,
    CW,
    CY,
    CZ,
    DK,
    DJ,
    DM,
    DO,
    EC,
    EG,
    SV,
    GQ,
    ER,
    EE,
    ET,
    FK,
    FO,
    FJ,
    FI,
    FR,
    GF,
    PF,
    TF,
    GA,
    GM,
    GE,
    DE,
    GH,
    GI,
    GR,
    GL,
    GD,
    GP,
    GU,
    GT,
    GG,
    GN,
    GW,
    GY,
    HT,
    HM,
    VA,
    HN,
    HK,
    HU,
    IS,
    IN,
    ID,
    IR,
    IQ,
    IE,
    IM,
    IL,
    IT,
    JM,
    JP,
    JE,
    JO,
    KZ,
    KE,
    KI,
    KP,
    KR,
    KW,
    KG,
    LA,
    LV,
    LB,
    LS,
    LR,
    LY,
    LI,
    LT,
    LU,
    MO,
    MK,
    MG,
    MW,
    MY,
    MV,
    ML,
    MT,
    MH,
    MQ,
    MR,
    MU,
    YT,
    MX,
    FM,
    MD,
    MC,
    MN,
    ME,
    MS,
    MA,
    MZ,
    MM,
    NA,
    NR,
    NP,
    NL,
    NC,
    NZ,
    NI,
    NE,
    NG,
    NU,
    NF,
    MP,
    NO,
    OM,
    PK,
    PW,
    PS,
    PA,
    PG,
    PY,
    PE,
    PH,
    PN,
    PL,
    PT,
    PR,
    QA,
    RE,
    RO,
    RU,
    RW,
    BL,
    SH,
    KN,
    LC,
    MF,
    PM,
    VC,
    WS,
    SM,
    ST,
    SA,
    SN,
    RS,
    SC,
    SL,
    SG,
    SX,
    SK,
    SI,
    SB,
    SO,
    ZA,
    GS,
    SS,
    ES,
    LK,
    SD,
    SR,
    SJ,
    SZ,
    SE,
    CH,
    SY,
    TW,
    TJ,
    TZ,
    TH,
    TL,
    TG,
    TK,
    TO,
    TT,
    TN,
    TR,
    TM,
    TC,
    TV,
    UG,
    UA,
    AE,
    GB,
    UM,
    US,
    UY,
    UZ,
    VU,
    VE,
    VN,
    VG,
    VI,
    WF,
    EH,
    YE,
    ZM,
    ZW,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonCartResponseItem {
    pub addon_id: String,
    pub quantity: i32,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerLimitedDetailsResponse {
    pub customer_id: String,
    pub email: String,
    pub name: String,
}

pub type Metadata = HashMap<String, String>;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MeterCartResponseItem {
    pub currency: Currency,
    pub description: Option<String>,
    pub free_threshold: i64,
    pub measurement_unit: String,
    pub meter_id: String,
    pub name: String,
    pub price_per_unit: String, // Using String as per OpenAPI example "10.50"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    Pending,
    Active,
    OnHold,
    Cancelled,
    Failed,
    Expired,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TimeInterval {
    Day,
    Week,
    Month,
    Year,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseKeyInstance {
    pub business_id: String,
    pub created_at: String,
    pub id: String,
    pub license_key_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    Percentage,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiscountResponse {
    pub amount: i32,
    pub business_id: String,
    pub code: String,
    pub created_at: String,
    pub discount_id: String,
    pub expires_at: Option<String>,
    pub name: Option<String>,
    pub restricted_to: Vec<String>,
    pub subscription_cycles: Option<i32>,
    pub times_used: i32,
    #[serde(rename = "type")]
    pub discount_type: DiscountType,
    pub usage_limit: Option<i32>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachExistingCustomer {
    pub customer_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CustomerRequest {
    AttachExisting(AttachExistingCustomer),
    New(CustomerInfo),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerInfoFull {
    pub business_id: String,
    pub created_at: String,
    pub customer_id: String,
    pub email: String,
    pub name: String,
    pub phone_number: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateDigitalProductDeliveryRequest {
    pub external_url: Option<String>,
    pub instructions: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PatchDigitalProductDeliveryRequest {
    pub external_url: Option<String>,
    pub files: Option<Vec<String>>,
    pub instructions: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LicenseKeyDuration {
    pub count: i32,
    pub interval: TimeInterval,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DigitalProductDeliveryFile {
    pub file_id: String,
    pub file_name: String,
    pub url: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DigitalProductDelivery {
    pub external_url: Option<String>,
    pub files: Option<Vec<DigitalProductDeliveryFile>>,
    pub instructions: Option<String>,
}


#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddMeterToPrice {
    pub meter_id: String,
    pub price_per_unit: String,
    pub description: Option<String>,
    pub free_threshold: Option<i64>,
    pub measurement_unit: Option<String>,
    pub name: Option<String>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OneTimePrice {
    pub price: i32,
    pub currency: Currency,
    pub discount: i64,
    pub purchasing_power_parity: bool,
    pub pay_what_you_want: Option<bool>,
    pub suggested_price: Option<i32>,
    pub tax_inclusive: Option<bool>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RecurringPrice {
    pub price: i32,
    pub currency: Currency,
    pub discount: i64,
    pub purchasing_power_parity: bool,
    pub payment_frequency_count: i32,
    pub payment_frequency_interval: TimeInterval,
    pub subscription_period_count: i32,
    pub subscription_period_interval: TimeInterval,
    pub tax_inclusive: Option<bool>,
    pub trial_period_days: Option<i32>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsageBasedPrice {
    pub fixed_price: i32,
    pub currency: Currency,
    pub discount: i64,
    pub purchasing_power_parity: bool,
    pub payment_frequency_count: i32,
    pub payment_frequency_interval: TimeInterval,
    pub subscription_period_count: i32,
    pub subscription_period_interval: TimeInterval,
    pub meters: Option<Vec<AddMeterToPrice>>,
    pub tax_inclusive: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Price {
    OneTime {
        #[serde(rename = "type")]
        type_field: String,
        #[serde(flatten)]
        one_time_price: OneTimePrice,
    },
    Recurring {
        #[serde(rename = "type")]
        type_field: String,
        #[serde(flatten)]
        recurring_price: RecurringPrice,
    },
    UsageBased {
        #[serde(rename = "type")]
        type_field: String,
        #[serde(flatten)]
        usage_based_price: UsageBasedPrice,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TaxCategory {
    DigitalProducts,
    Saas,
    EBook,
    Edtech,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AddonResponse {
    pub id: String,
    pub business_id: String,
    pub name: String,
    pub tax_category: TaxCategory,
    pub price: i32,
    pub currency: Currency,
    pub description: Option<String>,
    pub image: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

