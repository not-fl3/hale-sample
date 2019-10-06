use crate::def::player_seeking::*;
use hale::FamilyContainer;

impl PlayerSeekingSystem {
    pub fn update(
        &mut self,
        _: hale::Time,
        entity: MainFamily,
        players: &FamilyContainer<PlayersFamily>,
    ) {
        // Find the closest player
        let my_pos = entity.position.position;
        let best_target = players
            .iter()
            .min_by_key(|p| float_ord::FloatOrd((p.position.position - my_pos).squared_length()));

        if let Some(best_target) = best_target {
            let target = best_target.position.position - my_pos;
            entity.mob.move_dir = target.unit();
            entity.mob.face_dir = target.unit();
        } else {
            // No players to seek
            entity.mob.move_dir = hale::Vector2::new(0., 0.);
        }
    }
}
