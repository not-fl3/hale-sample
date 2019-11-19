use hale::{FamilyContainer, Point2, Vector2};

use crate::player_animation::*;

impl PlayerAnimationSystem {
    pub fn update(&mut self, delta: hale::Time, 
        e: MainFamily, weapons: &FamilyContainer<WeaponsFamily>, cursor: &FamilyContainer<CursorFamily>) {
        let weapon = weapons.get_by_id(e.shooter.weapon);

        let cursor = cursor.iter().next().unwrap();

        let vel = (cursor.position.position - e.position.position).unit();

        let angle = vel.y.atan2(vel.x);
        let dir = (8.0 * angle / (2.0 * std::f32::consts::PI) + 8.5) as usize % 8;
        let player = &mut e.sprite_animation.player;
        if e.mob.move_dir.length() >= 0.1 {
            player.set_sequence("run");
        } else {
            player.set_sequence("idle");
        }
        
        player.set_direction(dir);

        let player = &mut weapon.sprite_animation.player;
        player.set_direction(dir);

        let offsets = [
            (Vector2::new(3., 5.), Point2::new(15.0, 5.0)),
            (Vector2::new(4., 8.), Point2::new(12.0, 12.0)),
            (Vector2::new(0., 8.), Point2::new(0.0, 16.0)),
            (Vector2::new(-4., 8.), Point2::new(-12.0, 12.0)),
            (Vector2::new(-4., 5.), Point2::new(-15.0, 5.0)),
            (Vector2::new(-3., 2.), Point2::new(-10.0, -5.0)),
            (Vector2::new(0., -3.), Point2::new(0.0, -12.0)),
            (Vector2::new(3., 2.), Point2::new(10.0, -5.0)),
        ];
        if dir < 5 {
            weapon.sprite.layer = e.sprite.layer + 1;
        } else {
            weapon.sprite.layer = e.sprite.layer - 1;
        }
        weapon.gun.muzzle = offsets[dir].1;
        weapon.position.position = e.position.position + offsets[dir].0; 
    }
}
