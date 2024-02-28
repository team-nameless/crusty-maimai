//! The maimai website session.

use std::collections::HashMap;
use reqwest::header::HeaderMap;

/// The maimai website session.
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
    ///
    /// # Arguments
    /// * `username` - SEGA ID username.
    /// * `password` - SEGA ID password.
    ///
    /// # Returns
    /// * Raw HTML of the index site.
    ///
    /// # Warning
    /// <div class="warning">
    /// Unless you are testing locally, <strong>DO NOT</strong> hardcode the values as parameters.
    /// </div>
    pub fn login_with_credentials(&self, username: impl Into<String>, password: impl Into<String>) -> String {
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
            .expect("Unable to reach authorization page.");

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
            .expect("Unable to reach authorization page.");

        if response.url().host().unwrap().to_string() == "lng-tgk-aime-gw.am-all.net" {
            panic!("You logged in with wrong credentials!")
        }

        // SEGA did a sneaky move.
        // we no longer need to parse SSID since the redirection already happened at the lib level.
        // let returned_url = response.url().as_str();
        // but we maybe need the headers for the future?
        response.text().expect("No data present.")
    }

    /// Login into a session with session ID.
    ///
    /// For your sanity, please use [MaimaiSession::login_with_credentials] instead.
    ///
    /// # Arguments
    /// * `ssid` - The session ID of the logged-in session. (value of `?ssid=` URL param)
    ///
    /// # Returns
    /// * Raw HTML of the index site.
    ///
    /// # Warning
    /// <div class="warning">
    /// The SSID login method can fail at anytime due to SEGA purging sessions. For this
    /// reason, we <strong>WILL NOT</strong> provide support for this login method.
    ///
    /// In other words: "you are on your own".
    /// </div>
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
            panic!("You logged in with bad SSID! Maybe SEGA revoked the session.");
        }

        response.text().expect("No data present.")
    }

    /// Logout from the session.
    ///
    /// # Warning
    /// <div class="warning">
    /// If you used the SSID method to login, this will also <strong>REVOKE</strong> the SSID
    /// (i.e. you can't use that same SSID again)
    /// </div>
    pub fn logout(&self) {
        // SEGA requires the logout to be made from the option page directly.
        // So will use this hack to get the `Referer` header value.
        let user_option_url = String::from(Self::get_home_url()) + "/home/userOption";
        let response = self.request_client
            .get(user_option_url)
            .send()
            .expect("Unable to reach option page.");

        if response.url().as_str() == "https://maimaidx-eng.com/maimai-mobile/error/" {
            panic!("Seems like the session is not logged in. Or SEGA did something...")
        }

        // Now we do the actual shit.
        let logout_url = String::from(Self::get_home_url()) + "/home/userOption/logout";

        self.request_client
            .get(logout_url)
            .send()
            .expect("Unable to reach logout page.");
    }

    /// Jumps to a place in maimai website.
    /// Requires logged in.
    ///
    /// # Arguments
    /// * `place` - Destination URL.
    ///
    /// # Returns
    /// * Raw HTML of the target site.
    pub fn jump_to(&self, place: impl Into<String>) -> String {
        let response = self.request_client
            .get(place.into())
            .send()
            .expect("Unable to access the site.");

        if response.url().host().unwrap().to_string() == "lng-tgk-aime-gw.am-all.net" {
            panic!("You are not logged in!")
        }

        response.text().expect("No data present.")
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
