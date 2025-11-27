#[macro_export]
macro_rules! pattern {
    // Einstiegspunkt: Alle weiteren Token werden als Stream (tt-Muncher) übergeben.
    ($tokens:expr, $($input:tt)*) => {{
        use crate::core::lexer::token_match::TokenMatchSingleReturn;
        let mut vec: Vec<crate::core::lexer::token_match::Match<_>> = vec![];
        #[allow(unused_assignments)]
        let mut _parser_index = 0;
        pattern!(@internal vec, _parser_index, $tokens, $($input)*);

        $tokens.matches(&vec)
    }};

    // Basisfall: Nichts mehr zu verarbeiten.
    (@internal $vec:ident, $parser_index:ident, $tokens:expr,) => {};
    (@internal $vec:ident, $parser_index:ident, $tokens:expr) => {};

    // Überspringe ein Komma (damit du weiterhin Kommata in der Liste verwenden kannst).
    (@internal $vec:ident, $parser_index:ident, $tokens:expr, , $($rest:tt)*) => {
        pattern!(@internal $vec, $parser_index, $tokens, $($rest)*);
    };

    // Fall: @parse gefolgt von einem Identifier (z. B. `@parse RValue`).
    (@internal $vec:ident, $parser_index:ident, $tokens:expr, @parse $parser:ty, $($rest:tt)*) => {{
        if $parser_index >= $tokens.len() {
            return Err($crate::core::lexer::error::Error::UnexpectedEOF);
        }
        
        if let Ok(parse_result) = <$parser>::parse(&$tokens[$parser_index..]) {
            $parser_index += parse_result.consumed;
            $vec.push(parse_result.into());
            pattern!(@internal $vec, $parser_index, $tokens, $($rest)*);
        }
    }};

    // Standardfall: Ein Identifier wird als Token interpretiert.
    (@internal $vec:ident, $parser_index:ident, $tokens:expr, $token:ident $($rest:tt)*) => {{
        $vec.push($crate::core::lexer::token::Token::$token.into());
        $parser_index += 1;

        pattern!(@internal $vec, $parser_index, $tokens, $($rest)*);
    }};
}