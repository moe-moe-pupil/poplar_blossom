pub struct BattlefieldPlugin;

impl Plugin for BattlefieldPlugin {
  fn build(&self, app: &mut App) {
    
  }
}

#[derive(Component)]
struct Battlefield {
  cards: vec<Entity>,
  projectiles: vec<Entity>
}

