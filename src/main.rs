mod item_identifier;

use item_identifier::ItemIdentifier;

#[derive(ItemIdentifier)]
struct Blah {}

fn main() {
    let b = Blah {};
    assert_eq!("Blah", b.identifier());
}
