use hale::{api, World};

hale::gen!("resources/hale-sample.yaml");

mod systems;

struct GameStage;

impl api::Stage for GameStage {}

fn create_player(world: &mut World, api: &mut api::Api, position: hale::Point2) {
    world
        .create_entity()
        .add_component(Position { position })
        .add_component(Velocity {
            velocity: hale::Vector2::new(0.0, 0.0),
            target_position: hale::Point2::new(0.0, 0.0),
        })
        .add_component(SpriteAnimation {
            player: hale::AnimationPlayer::new(
                api.get_resource::<hale::api::Animation>("Player"),
                "idle",
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
            accel: 50.,
            max_speed: 300.,
        })
        .add_component(PlayerInput {
            input: api.get_virtual_input(),
        })
        .add_component(Player {})
        .add_component(Shooter {
            shooting: false,
            shoot_dir: hale::Vector2::new(0., 0.),
            cooldown: 0.0,
        })
        .add_component(Gun {
            cooldown: 0.1,
            kind: "machinegun".to_string(),
        })
        .add_component(Collider {
            rect: hale::Rect::new(-13.0, -13.0, 26.0, 26.0),
            layer: 0,
            trigger: false,
            is_static: false,
        })
        .add_component(RepulseField { multiplier: 10.0 });
}

fn create_obstacle(world: &mut World, rect: hale::Rect) {
    let size = rect.get_size();

    world
        .create_entity()
        .add_component(Position {
            position: rect.get_center(),
        })
        .add_component(Collider {
            rect: hale::Rect::from_points((size / -2.).to_point(), (size / 2.).to_point()),
            layer: 1,
            trigger: false,
            is_static: true,
        });
}

fn create_room(world: &mut World, api: &mut api::Api, pos: hale::Point2, id: hale::EntityId) {
    // world
    //     .create_entity()
    //     .add_component(Position {
    //         position: pos + hale::Vector2::new(350.0, 350.0),
    //     })
    //     .add_component(Sprite {
    //         sprite: hale::api::Sprite::new()
    //             .with_spritesheet(
    //                 api.resources(),
    //                 "trapped_scenery.json",
    //                 &format!("BG_0{}.png", id + 1),
    //             )
    //             .with_pivot(hale::Vector2::new(0.5, 0.5)),
    //         layer: -20,
    //     });

    // Enemy spawners
    let x0 = 125.0;
    let x1 = 565.0;
    let y0 = 220.0;
    let y1 = 605.0;
    let spawners = vec![
        hale::Vector2::new(x1, y1),
        hale::Vector2::new(x0, y1),
        hale::Vector2::new(x1, y0),
        hale::Vector2::new(x0, y0),
    ];

    for spawner_pos in spawners {
        world
            .create_entity()
            .add_component(Position {
                position: pos + spawner_pos,
            })
            .add_component(EnemySpawner {
                cooldown: hale::rand::gen_range(2.0, 4.0),
                room_id: id as i32,
            });
    }
}

fn get_room_offset(i: i32) -> hale::Point2 {
    match i {
        0 => hale::Point2::new(0.0, 0.0),
        1 => hale::Point2::new(700.0, 0.0),
        2 => hale::Point2::new(0.0, 700.0),
        3 => hale::Point2::new(700.0, 700.0),
        _ => panic!(),
    }
}

fn create_walls(world: &mut World) {
    let rects = vec![
        hale::Rect::new(0.0, 0.0, 1400.0, 12.0),
        hale::Rect::new(0.0, 0.0, 50.0, 1400.0),
        hale::Rect::new(0.0, 1290.0, 1400.0, 50.0),
        hale::Rect::new(1350.0, 0.0, 50.0, 1400.0),
        hale::Rect::new(650.0, 0.0, 100.0, 270.0),
        hale::Rect::new(650.0, 345.0, 100.0, 645.0),
        hale::Rect::new(650.0, 1050.0, 100.0, 352.0),
        hale::Rect::new(0.0, 560.0, 272.0, 130.0),
        hale::Rect::new(460.0, 560.0, 500.0, 130.0),
        hale::Rect::new(1145.0, 560.0, 270.0, 130.0),
    ];

    for r in &rects {
        let r2 = hale::Rect::new(
            r.get_left(),
            1400.0 - r.get_top() - r.get_height(),
            r.get_width(),
            r.get_height(),
        );
        create_obstacle(world, r2);
    }
}

fn main() {
    env_logger::init();

    api::start(GameStage, |world, api| {
        register_systems(world, api);

        create_player(world, api, hale::Point2::new(350.0, 350.0));
        for i in 0..4 {
            create_room(world, api, get_room_offset(i), i as u32);
        }

        //create_walls(world);
    });
}
