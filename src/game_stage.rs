use bevy_ecs::schedule::StageLabel;

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    PlayerCombat,
    MovePlayer,
    PlayerFov,
    GenerateMonsterMoves,
    MonsterCombat,
    MoveMonsters,
    MonsterFov,
}
