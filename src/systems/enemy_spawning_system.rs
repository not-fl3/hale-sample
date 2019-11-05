use crate::{enemy_spawning::*, *};
use hale::{
    rand::{IteratorRandom, Rng},
    FamilyContainer,
};

impl EnemySpawningSystem {
    pub fn update(
        &mut self,
        time: hale::Time,
        main_family: &FamilyContainer<MainFamily>,
        enemies_family: &FamilyContainer<EnemiesFamily>,
    ) {
        for e in main_family {
            e.enemy_spawner.cooldown = f32::max(e.enemy_spawner.cooldown - time, 0.0);
        }

        if enemies_family.len() < 20 {
            let mut eligible = main_family
                .iter()
                .filter(|e| e.enemy_spawner.room_id == 0 && e.enemy_spawner.cooldown == 0.0)
                .collect::<Vec<_>>();

            if eligible.len() > 0 {
                let mut rng = hale::rand::rng();

                let choice = eligible.iter_mut().choose(&mut rng).unwrap();

                self.create_enemy(choice.position.position);

                choice.enemy_spawner.cooldown = rng.gen_range(2.0, 5.0);
            }
        }
    }

    fn create_enemy(&mut self, position: hale::Point2) {
        let api = self.get_api();

        self.get_world()
            .create_entity()
            .add_component(Position { position })
            .add_component(Velocity {
                velocity: hale::Vector2::new(0.0, 0.0),
                target_position: hale::Point2::new(0.0, 0.0),
            })
            .add_component(SpriteAnimation {
                player: hale::AnimationPlayer::new(
                    api.get_resource::<hale::api::Animation>("Enemy"),
                    "default",
                    "default",
                ),
            })
            .add_component(Sprite {
                sprite: hale::api::Sprite::new(),
                layer: 0,
            })
            .add_component(Mob {
                move_dir: hale::Vector2::new(0.0, 0.0),
                face_dir: hale::Vector2::new(0.0, 0.0),
                accel: 30.,
                max_speed: 60.,
            })
            .add_component(Enemy {})
            .add_component(Collider {
                rect: hale::Rect::new(-18.0, -18.0, 36.0, 36.0),
                layer: 1,
                trigger: false,
                is_static: false,
            })
            .add_component(Health { max: 3, current: 3 })
            .add_component(RepulseField { multiplier: 5.0 })
            .add_component(Flashing {
                active: false,
                total_time: 0.0,
                cur_time: 0.0,
                colour: hale::Color::new(),
            });
    }
}
