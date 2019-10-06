use crate::def::sprite_render::*;
use hale::FamilyContainer;

impl SpriteRenderSystem {
    pub fn update(&mut self, _: hale::Time, main_family: &FamilyContainer<MainFamily>) {
        let api = self.get_api();

        let mut sprites = main_family.iter().collect::<Vec<_>>();
        sprites.sort_by_key(|s| s.sprite.layer);

        for e in &sprites {
            match (e.sprite.sprite.image.as_ref(), e.sprite.sprite.frame) {
                (Some(ref image), Some(ref frame)) => {
                    api.textured_rect(
                        e.position.position,
                        &*image,
                        *frame,
                        e.sprite.sprite.color,
                        e.sprite.sprite.rotated,
                        e.sprite.sprite.flip,
                    );
                }
                _ => {
                    api.colored_dot(e.position.position, e.sprite.sprite.color);
                }
            }
        }
    }
}
