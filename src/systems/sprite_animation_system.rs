use crate::def::sprite_animation::*;

impl SpriteAnimationSystem {
    pub fn update(&mut self, delta: hale::Time, e: MainFamily) {
        let sprite = &mut e.sprite.sprite;

        let vel = e.mob.face_dir;
        #[rustfmt::skip]
        let dir = if vel.y.abs() > vel.x.abs() { if vel.y < 0.0 { 0 } else { 2 } } else { if vel.x < 0.0 { 3 } else { 1 }};

        let player = &mut e.sprite_animation.player;
        player.set_direction(dir);
        player.update(delta);
        player.update_sprite(sprite);
    }
}
