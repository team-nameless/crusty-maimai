#[cfg(test)]
mod test_login {
    use dotenv::dotenv;
    use crusty_maimai::*;

    #[test]
    fn login_credentials_works() {
        dotenv().ok();

        let session = MaimaiSession::default();
        let username = std::env::var("SEGA_ID_USERNAME").unwrap();
        let password = std::env::var("SEGA_ID_PASSWORD").unwrap();

        let html = session.login_with_credentials(username, password);
        assert_ne!(html.len(), 0);
    }

    #[test]
    #[should_panic]
    fn login_credentials_fails() {
        let session = MaimaiSession::default();
        let html = session.login_with_credentials("123", "456");
        assert_ne!(html.len(), 0);
    }

    #[test]
    fn logout_credentials_works() {
        dotenv().ok();

        let session = MaimaiSession::default();
        let username = std::env::var("SEGA_ID_USERNAME").unwrap();
        let password = std::env::var("SEGA_ID_PASSWORD").unwrap();

        session.login_with_credentials(username, password);
        session.logout();
    }

    #[test]
    fn logout_fails() {
        let session = MaimaiSession::default();
        session.logout();
    }

    #[test]
    fn jump_works() {
        dotenv().ok();

        let session = MaimaiSession::default();
        let username = std::env::var("SEGA_ID_USERNAME").unwrap();
        let password = std::env::var("SEGA_ID_PASSWORD").unwrap();

        session.login_with_credentials(username, password);
        let html = session.jump_to("https://maimaidx-eng.com/maimai-mobile/playerData/");
        assert_ne!(html.len(), 0);
    }
}
