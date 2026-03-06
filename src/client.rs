use crate::types::*;

const DEFAULT_BASE_URL: &str = "https://smartcardfyi.com/api";

/// Async client for the SmartCardFYI API.
pub struct Client {
    base_url: String,
    http: reqwest::Client,
}

impl Client {
    /// Creates a new client with the default base URL.
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            http: reqwest::Client::new(),
        }
    }

    /// Creates a new client with a custom base URL.
    pub fn with_base_url(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            http: reqwest::Client::new(),
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T, SmartCardFyiError> {
        let url = format!("{}{}", self.base_url, path);
        let resp = self.http.get(&url).send().await?;
        if !resp.status().is_success() {
            return Err(SmartCardFyiError::Api {
                status: resp.status().as_u16(),
                body: resp.text().await.unwrap_or_default(),
            });
        }
        Ok(resp.json().await?)
    }

    /// Search across smart cards, platforms, and glossary terms.
    pub async fn search(&self, query: &str) -> Result<SearchResult, SmartCardFyiError> {
        let encoded = urlencoding(query);
        self.get(&format!("/search/?q={}", encoded)).await
    }

    /// Get details for a smart card by slug.
    pub async fn card(&self, slug: &str) -> Result<CardDetail, SmartCardFyiError> {
        self.get(&format!("/card/{}/", slug)).await
    }

    /// Get details for a smart card platform by slug.
    pub async fn platform(&self, slug: &str) -> Result<PlatformDetail, SmartCardFyiError> {
        self.get(&format!("/platform/{}/", slug)).await
    }

    /// Get details for a smart card standard by slug.
    pub async fn standard(&self, slug: &str) -> Result<StandardDetail, SmartCardFyiError> {
        self.get(&format!("/standard/{}/", slug)).await
    }

    /// Get details for a manufacturer by slug.
    pub async fn manufacturer(&self, slug: &str) -> Result<ManufacturerDetail, SmartCardFyiError> {
        self.get(&format!("/manufacturer/{}/", slug)).await
    }

    /// Get details for a smart card application by slug.
    pub async fn application(&self, slug: &str) -> Result<ApplicationDetail, SmartCardFyiError> {
        self.get(&format!("/application/{}/", slug)).await
    }

    /// Get details for a form factor by slug.
    pub async fn form_factor(&self, slug: &str) -> Result<FormFactorDetail, SmartCardFyiError> {
        self.get(&format!("/form-factor/{}/", slug)).await
    }

    /// Get details for a certification by slug.
    pub async fn certification(&self, slug: &str) -> Result<CertificationDetail, SmartCardFyiError> {
        self.get(&format!("/certification/{}/", slug)).await
    }

    /// Get a glossary term by slug.
    pub async fn glossary_term(&self, slug: &str) -> Result<GlossaryTerm, SmartCardFyiError> {
        self.get(&format!("/glossary/{}/", slug)).await
    }

    /// Compare two smart cards.
    pub async fn compare(&self, slug_a: &str, slug_b: &str) -> Result<CompareResult, SmartCardFyiError> {
        self.get(&format!("/compare/?a={}&b={}", slug_a, slug_b)).await
    }

    /// Get a random smart card.
    pub async fn random(&self) -> Result<CardDetail, SmartCardFyiError> {
        self.get("/random/").await
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

fn urlencoding(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
            ' ' => "+".to_string(),
            _ => format!("%{:02X}", c as u32),
        })
        .collect()
}
