use reqwest::header::HeaderMap;
use std::collections::HashMap;

/// A maimai session.
pub struct MaimaiSession {
    /// The internal request client.
    request_client: reqwest::blocking::Client,
}

impl MaimaiSession {
    /// Get authorization URL.
    fn get_auth_url() -> &'static str {
        "https://lng-tgk-aime-gw.am-all.net/common_auth/login"
    }

    /// Get index page URL.
    fn get_home_url() -> &'static str {
        "https://maimaidx-eng.com/maimai-mobile"
    }

    /// Login into a session with authentication credentials/SEGA ID.
    pub fn login_with_credentials(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> String {
        let username_converted = username.into();
        let password_converted = password.into();

        // auth page hit once to get required cookies.
        // actually we only need the session ID ("JSESSIONID=") one.
        let mut auth_page_hit_params = HashMap::new();
        auth_page_hit_params.insert("site_id", "maimaidxex");
        auth_page_hit_params.insert("redirect_url", Self::get_home_url());
        auth_page_hit_params.insert("back_url", "https://maimai.sega.com/");

        let auth_url = Self::get_auth_url();
        self.request_client
            .get(auth_url)
            .query(&auth_page_hit_params)
            .send()
            .expect("Invalid auth URL.");

        // doing the actual auth now.
        let mut login_page_form = HashMap::new();
        login_page_form.insert("retention", "1");
        login_page_form.insert("sid", username_converted.as_str());
        login_page_form.insert("password", password_converted.as_str());

        let sid_url = String::from(Self::get_auth_url()) + "/sid";
        let response = self
            .request_client
            .post(sid_url)
            .form(&login_page_form)
            .send()
            .expect("Invalid credentials provided.");

        if response.url().host().unwrap().to_string() == "lng-tgk-aime-gw.am-all.net" {
            panic!("You logged in with wrong credentials!")
        }

        // SEGA did a sneaky move.
        // we no longer need to parse SSID since the redirection already happened at the lib level.
        // let returned_url = response.url().as_str();
        // but we maybe need the headers for the future?
        response.text().expect("Wrong place to crawl.")
    }

    /// Login into a session with session ID.
    ///
    /// <div class="warning">The SSID login method can fail at anytime!!!</div>
    pub fn login_with_ssid(&self, ssid: impl Into<String>) -> String {
        let ssid_converted = ssid.into();

        let mut home_url_params = HashMap::new();
        home_url_params.insert("ssid", ssid_converted.as_str());

        let home_url = Self::get_home_url();
        let response = self
            .request_client
            .get(home_url)
            .query(&home_url_params)
            .send()
            .expect("Invalid home URL.");

        if response.url().to_string() == "https://maimaidx-eng.com/maimai-mobile/error/" {
            panic!("You logged in with wrong SSID!")
        }

        // SEGA did a sneaky move.
        // we no longer need to parse SSID since the redirection already happened at the lib level.
        // let returned_url = response.url().as_str();
        // but we maybe need the headers for the future?
        response.text().expect("Wrong place to crawl.")
    }
}

impl Default for MaimaiSession {
    fn default() -> Self {
        // supply default headers.
        let mut headers: HeaderMap = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );
        headers.insert("Connection", "keep-alive".parse().unwrap());

        Self {
            request_client: reqwest::blocking::Client::builder()
                .referer(true)
                .default_headers(headers)
                .cookie_store(true)
                .build()
                .unwrap(),
        }
    }
}
