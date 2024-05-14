#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    GameOver,
    Victory,
    NextLevel,
}
