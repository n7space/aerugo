#[cfg(test)]
mod tests {
    use env_parser::read_env;

    #[test]
    fn test_integer_default() {
        #[read_env]
        const ENV_PARSER_TEST_INTEGER_DEFAULT: u8 = 10;

        assert_eq!(ENV_PARSER_TEST_INTEGER_DEFAULT, 10)
    }

    #[test]
    fn test_integer_override() {
        #[read_env]
        const ENV_PARSER_TEST_INTEGER_OVERRIDE: u8 = 10;

        assert_eq!(ENV_PARSER_TEST_INTEGER_OVERRIDE, 42)
    }

    #[test]
    fn test_integer_different_name() {
        #[read_env("ENV_PARSER_TEST_INTEGER_DIFFERENT_NAME")]
        const FUN_VARIABLE: u32 = 10;

        assert_eq!(FUN_VARIABLE, 314)
    }
}
