use bevy::{prelude::*, utils::Duration};

#[cfg(debug_assertions)]
use bevy::utils::Instant;

#[cfg(debug_assertions)]
use draw_debug_material::DrawDebugMaterial;
#[cfg(debug_assertions)]
use once_cell::sync::Lazy;
#[cfg(debug_assertions)]
use std::sync::RwLock;

mod draw_debug_material;

#[macro_export]
macro_rules! draw_debug {
    ($entity:ident) => {
        #[cfg(debug_assertions)]
        crate::_draw_debug_mesh($entity, DrawDebugOptions::default());
    };
    ($entity:ident, $options:ident) => {
        #[cfg(debug_assertions)]
        crate::_draw_debug_mesh($entity, $options);
    };
    ($entity:ident, $options:expr) => {
        #[cfg(debug_assertions)]
        crate::_draw_debug_mesh($entity, $options);
    };
}

#[cfg(debug_assertions)]
static DRAW_DEBUG_OLD: Lazy<RwLock<Vec<DrawDebugObject>>> = Lazy::new(|| RwLock::new(vec![]));

#[cfg(debug_assertions)]
static DRAW_DEBUG_NEW: Lazy<RwLock<Vec<DrawDebugObject>>> = Lazy::new(|| RwLock::new(vec![]));

#[doc(hidden)]
#[derive(Copy, Clone)]
#[cfg(debug_assertions)]
pub struct DrawDebugObject {
    entity: Entity,
    options: DrawDebugOptions,
    creation: Instant,
}

#[cfg(debug_assertions)]
impl DrawDebugObject {
    pub fn new(entity: Entity, options: DrawDebugOptions) -> Self {
        Self {
            entity,
            options,
            creation: Instant::now(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct DrawDebugOptions {
    pub duration: Option<Duration>,
    pub color: Option<Color>,
}

impl Default for DrawDebugOptions {
    fn default() -> Self {
        Self {
            duration: Some(Duration::new(10, 0)),
            color: Some(Color::WHITE),
        }
    }
}

pub struct DrawDebugPlugin;

impl Plugin for DrawDebugPlugin {
    #[allow(unused)]
    fn build(&self, app: &mut App) {
        #[cfg(debug_assertions)]
        app.add_plugin(MaterialPlugin::<DrawDebugMaterial>::default())
            .add_system_to_stage(CoreStage::PostUpdate, draw_debug_system.exclusive_system());
    }
}

#[derive(Component)]
struct DebugDrawTag;

#[cfg(debug_assertions)]
fn draw_debug_system(world: &mut World) {
    let mut new = DRAW_DEBUG_NEW
        .write()
        .expect("Failed to get debug objects to draw");
    while !new.is_empty() {
        let obj = new.pop().expect("Should not have tried to pop empty list");
        let (mesh_handle, transform) = {
            let entity = world.entity(obj.entity);
            let mesh_h = entity
                .get::<Handle<Mesh>>()
                .expect("Failed to get mesh handle from entity");
            let trans = entity
                .get::<Transform>()
                .expect("Failed to get transform from entity");
            (mesh_h.clone(), trans.clone())
        };
        let mat_handle = {
            let mut assets = world
                .get_resource_mut::<Assets<DrawDebugMaterial>>()
                .expect("There should be an asset store for DrawDebugMaterial");
            assets.add(DrawDebugMaterial {
                color: obj.options.color.unwrap_or(Color::WHITE),
            })
        };
        let mut entity = world.spawn();
        entity.insert_bundle(MaterialMeshBundle {
            mesh: mesh_handle.clone(),
            material: mat_handle,
            transform,
            ..default()
        });
        let mut obj = obj.clone();
        obj.entity = entity.id();
        DRAW_DEBUG_OLD
            .write()
            .expect("Failed to get debug objects to check")
            .push(obj);
    }

    DRAW_DEBUG_OLD
        .write()
        .expect("Failed to get debug objects to check")
        .retain(|obj| match obj.options.duration {
            None => true,
            Some(duration) => {
                if Instant::now().duration_since(obj.creation) > duration {
                    world.despawn(world.entity(obj.entity).id());
                    false
                } else {
                    true
                }
            }
        });
}

#[doc(hidden)]
#[cfg(debug_assertions)]
pub fn _draw_debug_mesh(entity: Entity, options: DrawDebugOptions) {
    DRAW_DEBUG_NEW
        .write()
        .expect("Failed to get debug objects to draw")
        .push(DrawDebugObject::new(entity, options));
}
