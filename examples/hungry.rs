use yes_or_no::yes_or_no;

yes_or_no!(Hungry);

fn main() {
    assert_eq!(Hungry::from(true), Hungry::Yes);
    assert_eq!(Hungry::from(false), Hungry::No);
    assert!(Hungry::Yes.yes());
    assert!(Hungry::No.no());
}
