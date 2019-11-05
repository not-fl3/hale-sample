use hale::FamilyContainer;

use crate::{camera_follow::*, Damage};

impl CameraFollowSystem {
    pub fn update(&mut self, time: hale::Time, e: MainFamily, player: &FamilyContainer<PlayerFamily>) {
        assert!(player.len() == 1);

        let player = player.iter().next().unwrap();

        e.camera.camera.target = player.position.position;
    }
}
