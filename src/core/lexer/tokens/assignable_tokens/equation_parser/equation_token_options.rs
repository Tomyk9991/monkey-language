pub trait EquationTokenOptions {
    fn additive() -> Option<char>;
    fn inverse_additive() -> Option<char>;

    fn multiplicative() -> Option<char>;
    fn inverse_multiplicative() -> Option<char>;

    fn negate() -> Option<char>;
}

#[derive(Clone)]
pub struct ArithmeticEquationOptions;
#[derive(Clone)]
pub struct BooleanEquationOptions;


impl EquationTokenOptions for BooleanEquationOptions {
    fn additive() -> Option<char> { Some('|') }

    fn inverse_additive() -> Option<char> { None }

    fn multiplicative() -> Option<char> { Some('&') }

    fn inverse_multiplicative() -> Option<char> { None }

    fn negate() -> Option<char> { Some('~') }
}

impl EquationTokenOptions for ArithmeticEquationOptions {
    fn additive() -> Option<char> { Some('+') }
    fn inverse_additive() -> Option<char> { Some('-') }
    fn multiplicative() -> Option<char> { Some('*') }
    fn inverse_multiplicative() -> Option<char> { Some('/') }
    fn negate() -> Option<char> { Some('-') }
}