use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct KtpExtractRequest {
    // base64 image
    pub image: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KtpExtractResponse {
    pub nik: String,
    pub city: String,
    pub rt_rw: String,
    pub state: String,
    pub gender: String,
    pub address: String,
    pub district: String,
    pub religion: String,
    pub full_name: String,
    pub blood_type: String,
    pub occupation: String,
    pub nationality: String,
    pub date_of_birth: String,
    pub marital_status: String,
    pub place_of_birth: String,
    pub administrative_village: String,
    pub reference_id: Option<String>,
    pub status_code: Option<String>,
    pub image_quality: KtpRecognitionImageQuality,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KtpRecognitionImageQuality {
    pub blur: bool,
    pub dark: bool,
    pub grayscale: bool,
    pub blur_score: f32,
    pub dark_score: f32,
    pub flashlight: bool,
}
