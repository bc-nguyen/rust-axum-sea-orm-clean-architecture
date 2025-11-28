#[cfg(test)]
mod token_test_suite {
    use lib::infrastructure::helpers::token::JwtHelper;

    #[test]
    fn generate_token_success() {
        let jwt_helper = JwtHelper::new("secret".to_string());

        let result = jwt_helper.generate("test".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn validate_token_success() {
        let jwt_helper = JwtHelper::new("secret".to_string());

        let sub = "this is value";

        let token = jwt_helper.generate(sub.to_string()).unwrap();

        let valid = jwt_helper.validate(token.as_str());

        assert!(valid.is_ok());

        let value = valid.unwrap();

        assert!(value == sub)
    }

    #[test]
    fn validate_token_faild() {
        let jwt_helper = JwtHelper::new("secret".to_string());

        let valid = jwt_helper.validate("non-valid");

        assert!(valid.is_err());
    }
}
