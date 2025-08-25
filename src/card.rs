#[derive(Component)]
struct Card {
    card_type: CardType,
}
enum CardType {
    Construct,
    Turret,
    Effect,
}

#[derive(Component)]
struct Deck {}

#[derive(Component)]
struct Hand {}
