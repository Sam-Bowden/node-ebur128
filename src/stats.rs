use napi_derive::napi;

#[napi(object)]
pub struct Stats {
    pub m_lufs: f64,
    pub s_lufs: f64,
    pub i_lufs: f64,
    pub lra_lu: f64,
    pub peaks_dbfs: Vec<f64>,
    pub true_peaks_dbtp: Vec<f64>,
}
