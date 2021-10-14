macro_rules! outer {
    ($string:literal) => {
        pmacro::macro_impl!($string)
    };
}

pub fn foo() {
    outer!("foobar");
}
