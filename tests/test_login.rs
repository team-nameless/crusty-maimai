#[cfg(test)]
mod test_login {
    use dotenv::dotenv;
    use crusty_maimai::MaimaiSession;

    #[test]
    fn credential_works() {
        dotenv().ok();

        let session = MaimaiSession::default();
        let username = std::env::var("SEGA_ID_USERNAME").unwrap();
        let password = std::env::var("SEGA_ID_PASSWORD").unwrap();

        let html = session.login_with_credentials(username, password);
        assert_ne!(html.len(), 0);
    }

    #[test]
    fn ssid_works() {
        dotenv().ok();

        let session = MaimaiSession::default();
        let ssid = std::env::var("SEGA_SSID").unwrap();

        let html = session.login_with_ssid(ssid);
        assert_ne!(html.len(), 0);
    }

    #[test]
    #[should_panic]
    fn credential_fails() {
        let session = MaimaiSession::default();
        let html = session.login_with_credentials("123", "456");
        assert_ne!(html.len(), 0);
    }

    #[test]
    #[should_panic]
    fn ssid_fails() {
        let session = MaimaiSession::default();
        let html = session.login_with_ssid("abc");
        assert_ne!(html.len(), 0);
    }
}
