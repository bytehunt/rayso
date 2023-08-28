const RAY_URL: &str = "https://ray.so/#code=";

// #[allow(non_camel_case_types)]
// struct URL_base {
//     theme: String,
//     background: bool,
//     padding: usize,
//     darkmode: bool,
// }

pub fn generate_url(
    theme: &str,
    background: bool,
    padding: usize,
    darkmode: bool,
    base64_encoded: &str,
    filename: &str,
) -> String {
    format!(
        "{}{}&darkMode={}&theme={}&title={}&background={}&padding={}",
        RAY_URL, base64_encoded, darkmode, theme, filename, background, padding
    )
}
