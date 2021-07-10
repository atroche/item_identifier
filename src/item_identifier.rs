pub use struct_name_derive::*;

pub trait ItemIdentifier {
    const IDENTIFIER: &'static str;

    fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}

mod test {
    use super::ItemIdentifier;

    struct Foobar {}
    impl ItemIdentifier for Foobar {
        const IDENTIFIER: &'static str = "Foobar";
    }

    #[test]
    fn some_test() {
        let f = Foobar {};
        dbg!(f.identifier());
    }
}
