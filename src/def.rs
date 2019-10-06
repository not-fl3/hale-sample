hale::gen!(
    "
---

component:
    name: Position
    members:
        - position: hale::Point2

---

component:
    name: Velocity
    members:
        - velocity: hale::Vector2
        - target_position: hale::Point2

---

component: 
  name: Collider
  members:
    - rect: hale::Rect
    - layer: i32
    - trigger: bool
    - is_static: bool

---

component: 
  name: Sprite
  members:
    - sprite: hale::Sprite
    - layer: i32

---

component: 
  name: SpriteAnimation
  members:
    - player: hale::AnimationPlayer

---

component:
    name: Score
    members:        
        - collides: i32

---

component: 
  name: PlayerInput
  members:
    - input: hale::InputVirtual

---

component: 
  name: Mob
  members:
    - move_dir: hale::Vector2
    - face_dir: hale::Vector2
    - accel: f32
    - max_speed: f32

---
component: 
  name: RepulseField
  members:
    - multiplier: f32

---

component: 
  name: Enemy

---

component: 
  name: Player

---

component: 
  name: Bullet
  members:
    - damage: i32

---

component: 
  name: TTL
  members:
    - time_left: f32

---

component: 
  name: Shooter
  members:
    - shooting: bool
    - shoot_dir: hale::Vector2
    - cooldown: f32

---

component: 
  name: Gun
  members:
    - cooldown: f32
    - kind: String

---

component: 
  name: Health
  members:
    - max: i32
    - current: i32

---

component: 
  name: EnemySpawner
  members:
    - cooldown: f32
    - room_id: i32

---

component: 
  name: Flashing
  members:
    - active: bool
    - total_time: f32
    - cur_time: f32
    - colour: hale::Color

---

system: 
  name: PlayerInput
  families:
    - main:
      - Mob: write
      - Shooter: write
      - PlayerInput: read

---

system: 
  name: Mob
  families:
    - main:
      - Mob: read
      - Velocity: write

---

system: 
  name: IntegrateVelocity
  families:
    - main:
      - Position: read
      - Velocity: write

---

system: 
  name: CheckStaticCollision
  families:
    - main:
      - Position: read
      - Velocity: write
      - Collider: read
    - obstacles:
      - Position: read
      - Collider: read
  messages:
    - Collision: send

---

system: 
  name: CheckDynamicCollision
  families:
    - main:
      - Position: read
      - Velocity: write
      - Collider: read
    - targets:
      - Position: read
      - Collider: read
  messages:
    - Collision: send

---

system: 
  name: Movement
  families:
    - main:
      - Position: write
      - Velocity: read

---

system: 
  name: Repulsion
  families:
    - main:
      - Position: read
      - Velocity: write
      - RepulseField: read
      - Collider: read
  messages:
    - Collision: receive
  access: ['world']

---

system:
  name: PlayerSeeking
  families:
    - main:
      - Position: read
      - Mob: write
      - Enemy: read
    - players:
      - Position: read
      - Player: read

---

system: 
  name: Shooter
  families:
    - main:
      - Shooter: write
      - Position: read
      - Velocity: read
      - Gun: read
  access: ['world', 'api']

---

system: 
  name: TTL
  families:
    - main:
      - TTL: write
  access: ['world']

---

system: 
  name: Damage
  families:
    - main:
      - Health: write
  messages:
    - Damage: receive
    - Death: send
  access: ['world']

---

system: 
  name: Bullet
  families:
    - main:
      - Bullet: read
      - Position: read
  messages:
    - Collision: receive
    - Damage: send
  access: ['world', 'api']

---

system: 
  name: EnemySpawning
  families:
    - main:
      - Position: read
      - EnemySpawner: write
    - enemies:
      - Enemy: read
  access: ['world', 'api']
  strategy: 'global'
---

system: 
  name: EnemyDeath
  families:
    - main:
      - Enemy: read
      - Position: read
  messages:
    - Death: receive
  access: ['api']

---

system: 
  name: Flashing
  families:
    - main:
      - Flashing: write
      - Sprite: write
  messages:
    - Damage: receive

---

system: 
  name: SpriteAnimation
  families:
    - main:
      - Sprite: write
      - SpriteAnimation: write
      - Mob: read

---

system: 
  name: SpriteRender
  families:
    - main:
      - Position: read
      - Sprite: write
  method: render
  strategy: global

---

message: 
  name: Collision
  members:
    - other: hale::EntityId
    - other_layer: i32
    - other_rect: hale::Rect

---

message: 
  name: Damage
  members:
    - amount: i32

---

message: 
  name: Death

...
"
);
