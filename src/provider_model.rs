use serde::{Deserialize,Serialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize,Serialize)]
pub struct NYProviderFeed {
    #[serde(rename(deserialize = "MEDICAID PROVIDER ID"))]
    pub medicaid_provider_id: Option<String>,

    #[serde(rename(deserialize = "NPI"))]
    pub npi: Option<String>,

    #[serde(rename(deserialize = "PROVIDER OR FACILITY NAME"))]
    pub provider_or_facility_name: Option<String>,

    #[serde(rename(deserialize = "MEDICAID TYPE"))]
    pub medicaid_type: Option<String>,

    #[serde(rename(deserialize = "PROFESSION OR SERVICE"))]
    pub profession_or_service: Option<String>,

    #[serde(rename(deserialize = "PROVIDER SPECIALTY"))]
    pub provider_specialty: Option<String>,

    #[serde(rename(deserialize = "SERVICE ADDRESS"))]
    pub service_address: Option<String>,

    #[serde(rename(deserialize = "CITY"))]
    pub city: Option<String>,

    #[serde(rename(deserialize = "STATE"))]
    pub state: Option<String>,

    #[serde(rename(deserialize = "ZIP CODE"))]
    pub zip_code: Option<String>,

    #[serde(rename(deserialize = "COUNTY"))]
    pub county: Option<String>,
    #[serde(rename(deserialize = "TELEPHONE"))]
    pub telephone: Option<String>,
    #[serde(rename(deserialize = "LATITUDE"))]
    pub latitude: Option<String>,

    #[serde(rename(deserialize = "LONGITUDE"))]
    pub longitude: Option<String>,

    #[serde(rename(deserialize = "ENROLLMENT BEGIN DATE"))]
    pub enrollment_begin_date: Option<String>,

    #[serde(rename(deserialize = "NEXT ANTICIPATED REVALIDATION DATE"))]
    pub next_anticipated_revalidation_date: Option<String>,

    #[serde(rename(deserialize = "FILE DATE"))]
    pub file_date: Option<String>,

    #[serde(rename(deserialize = "MEDICALLY FRAGILE CHILDREN DIRECTORY IND"))]
    pub medically_fragile_children_directory_ind: Option<String>,

    #[serde(rename(deserialize = "PROVIDEREMAIL"))]
    pub provider_email: Option<String>,
}

