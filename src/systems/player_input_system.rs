use crate::player_input::*;

impl PlayerInputSystem {
    pub fn update(&mut self, _: hale::Time, entity: MainFamily) {
        let input = &entity.player_input.input;
        let move_dir: hale::Vector2 = hale::Vector2::new(input.get_axis(0), input.get_axis(1));
        let shoot_dir: hale::Vector2 = hale::Vector2::new(input.get_axis(2), input.get_axis(3));

        let shooting: bool = shoot_dir.length() > 0.1;
        let face_dir = if shooting { shoot_dir } else { move_dir };

        entity.mob.move_dir = move_dir;

        if face_dir.squared_length() > 0.1 {
            entity.mob.face_dir = face_dir.unit();
        }

        entity.shooter.shooting = shooting;
        entity.shooter.shoot_dir = face_dir.unit();

        if move_dir.length() > 0.1 {
            entity.sprite_animation.player.set_sequence("run");
        } else {
            entity.sprite_animation.player.set_sequence("idle");
        }
    }
}
