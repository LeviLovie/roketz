use bevy_ecs::prelude::*;
use nalgebra::Vector2;

use super::{RendererRes, Transform};
use deferred::{load_texture_from_disk, GpuTextureHandle, Object};

#[derive(Component)]
pub struct Texture {
    pub handle: Option<GpuTextureHandle>,
    pub path: String,
    pub width: u32,
    pub height: u32,
}

pub fn load_textures(mut query: Query<&mut Texture>, renderer: Res<RendererRes>) {
    let mut renderer_lock = renderer.0.lock_panic();

    for mut tex in &mut query {
        if tex.handle.is_none() {
            if let Some(&handle) = renderer_lock.texture_cache.lookup.get(&tex.path) {
                tex.handle = Some(renderer_lock.texture_cache.textures[handle as usize].clone());
            } else {
                let texture_handle =
                    load_texture_from_disk(&renderer_lock.device, &renderer_lock.queue, &tex.path);
                let id = renderer_lock.texture_cache.textures.len() as u32;
                renderer_lock
                    .texture_cache
                    .textures
                    .push(texture_handle.clone());
                renderer_lock
                    .texture_cache
                    .lookup
                    .insert(tex.path.clone(), id);

                tex.handle = Some(texture_handle);
                renderer_lock.textures_updated = true;
            }
        }
    }
}

pub fn clear_objects(renderer: ResMut<RendererRes>) {
    renderer.0.lock().unwrap().clear_layers();
}

pub fn transfer_objects(objects: Query<(&Transform, &Texture)>, renderer: ResMut<RendererRes>) {
    let mut renderer_lock = renderer.0.lock().unwrap();

    for (transform, texture) in &objects {
        if texture.handle.is_some() {
            if let Some(texture_id) = renderer_lock.texture_cache.get_texture_id(&texture.path) {
                if let Some((batch_id, id_in_batch)) =
                    renderer_lock.texture_cache.get_batch_info(texture_id)
                {
                    let mut object = Object::default();
                    object.set_pos(Vector2::from([transform.position.x, transform.position.y]));
                    object.set_size(Vector2::from([
                        texture.width as f32 * transform.scale.x,
                        texture.height as f32 * transform.scale.y,
                    ]));
                    object.set_rot(transform.rotation);
                    object.set_bid(batch_id);
                    object.set_tid(id_in_batch as usize);
                    renderer_lock.push_object(transform.layer, object);
                }
            }
        }
    }
}
