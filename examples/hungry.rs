use yes_or_no::yes_or_no;

yes_or_no!(Hungry);
yes_or_no!(HungrySerde, true);

fn main() {
    assert_eq!(Hungry::from(true), Hungry::Yes);
    assert_eq!(Hungry::from(false), Hungry::No);
    assert!(Hungry::Yes.yes());
    assert!(Hungry::No.no());
}
