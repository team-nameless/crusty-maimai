#[cfg(test)]
mod test_user {
    use dotenv::dotenv;
    use crusty_maimai::{models::MaimaiUser, *};

    #[test]
    fn user_data_exists() {
        dotenv().ok();

        let session = MaimaiSession::default();
        let username = std::env::var("SEGA_ID_USERNAME").unwrap();
        let password = std::env::var("SEGA_ID_PASSWORD").unwrap();

        session.login_with_credentials(username, password);

        let user: MaimaiUser = MaimaiUser::populate(session);

        assert_ne!(user.name, "");
        assert_ne!(user.rating, 0);
    }
}
