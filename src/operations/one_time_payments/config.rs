use serde::{Deserialize, Serialize};
use serde_valid::Validate;
use std::collections::HashMap;

use crate::operations::checkout_sessions::BillingAddress;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OneTimeProductCartItemReq {
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
#[serde(rename_all = "UPPERCASE")]
pub enum Currency {
    Aed, All, Amd, Ang, Aoa, Ars, Aud, Awg, Azn, Bam, Bbd, Bdt, Bgn, Bhd, Bif, Bmd, Bnd, Bob, Brl, Bsd, Bwp, Byn, Bzd, Cad, Chf, Clp, Cny, Cop, Crc, Cup, Cve, Czk, Djf, Dkk, Dop, Dzd, Egp, Etb, Eur, Fjd, Fkp, Gbp, Gel, Ghs, Gip, Gmd, Gnf, Gtq, Gyd, Hkd, Hnl, Hrk, Htg, Huf, Idr, Ils, Inr, Iqd, Jmd, Jod, Jpy, Kes, Kgs, Khr, Kmf, Krw, Kwd, Kyd, Kzt, Lak, Lbp, Lkr, Lrd, Lsl, Lyd, Mad, Mdl, Mga, Mkd, Mmk, Mnt, Mop, Mru, Mur, Mvr, Mwk, Mxn, Myr, Mzn, Nad, Ngn, Nio, Nok, Npr, Nzd, Omr, Pab, Pen, Pgk, Php, Pkr, Pln, Pyg, Qar, Ron, Rsd, Rub, Rwf, Sar, Sbd, Scr, Sek, Sgd, Shp, Sle, Sll, Sos, Srd, Ssp, Stn, Svc, Szl, Thb, Tnd, Top, Try, Ttd, Twd, Tzs, Uah, Ugx, Usd, Uyu, Uzs, Ves, Vnd, Vuv, Wst, Xaf, Xcd, Xof, Xpf, Yer, Zar, Zmw,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachExistingCustomer {
    pub customer_id: String,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewCustomer {
    pub name: String,
    pub email: String,
    pub phone_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum CustomerRequest {
    AttachExisting(AttachExistingCustomer),
    New(NewCustomer),
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
pub struct CreateOneTimePaymentRequest {
    #[validate(min_items = 1)]
    pub product_cart: Vec<OneTimeProductCartItemReq>,
    pub customer: CustomerRequest,
    pub billing: BillingAddress,
    pub allowed_payment_method_types: Option<Vec<PaymentMethodTypes>>,
    pub billing_currency: Option<Currency>,
    pub discount_code: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
    pub payment_link: Option<bool>,
    pub return_url: Option<String>,
    pub show_saved_payment_methods: Option<bool>,
    pub tax_id: Option<String>,
}
