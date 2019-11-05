use hale::FamilyContainer;

use crate::{shooter::*, *};

impl ShooterSystem {
    pub fn update(&mut self, p: hale::Time, e: MainFamily, guns: &FamilyContainer<WeaponsFamily>) {
        let gun = guns.get_by_id(e.shooter.weapon).gun;

        let cooldown = &mut e.shooter.cooldown;
        if *cooldown > 0. {
            *cooldown = (*cooldown - p).max(0.0);
        }

        if e.shooter.shooting && *cooldown < 0.01 {
            *cooldown = gun.cooldown;
            self.spawn_bullet(
                &gun.kind,
                e.position.position,
                e.shooter.shoot_dir,
                e.velocity.velocity,
            );
        }
    }

    fn spawn_bullet(
        &mut self,
        _kind: &str,
        pos: hale::Point2,
        dir: hale::Vector2,
        player_vel: hale::Vector2,
    ) {
        let vel = dir * 500. + player_vel * 0.5;
        let origin = pos + hale::Vector2::new(0., 5.) + vel * 0.016;
        let ttl = 0.4;
        let damage = 1;

        let api = self.get_api();
        self.get_world()
            .create_entity()
            .add_component(Position { position: origin })
            .add_component(Velocity {
                velocity: vel,
                target_position: hale::Point2::new(0., 0.),
            })
            .add_component(Sprite {
                sprite: hale::api::Sprite::new()
                    .with_spritesheet(api.resources(), 
                        "spritesheet.json", 
                        &format!("flamethrower_bullet_{}.png", hale::rand::gen_range(0, 1)))
                    .with_pivot(hale::Vector2::new(0.5, 0.5)),
                layer: 0,
            })
            // 	.setColour(Colour(1, 1, 0))
            // 	.setPivot(Vector2f(0.5f, 0.5f))
            // 	.setRotation(vel.angle()), 0))
            .add_component(Bullet { damage })
            .add_component(TTL { time_left: ttl })
            .add_component(Collider {
                rect: hale::Rect::new(-5., -5., 10., 10.),
                layer: 2,
                trigger: false,
                is_static: false,
            });

        //auto clip = getAPI().getResource<AudioClip>("sound/weak_shot.ogg");
        //getAPI().audio->play(clip, AudioSourcePosition::makePositional(origin));
    }
}
