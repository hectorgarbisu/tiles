use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorPos(pub Vec2);
impl Default for CursorPos {
    fn default() -> Self {
        Self(Vec2::new(-1000.0, -1000.0))
    }
}

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cursor_pos);
    }
}

pub fn update_cursor_pos(
    camera_q: Query<(&GlobalTransform, &Camera)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut cursor_pos: ResMut<CursorPos>,
) {
    for cursor_moved in cursor_moved_events.read() {
         for (cam_t, cam) in camera_q.iter() {
            if let Some(pos) = cam.viewport_to_world_2d(cam_t, cursor_moved.position) {
                *cursor_pos = CursorPos(pos);
            }
        }
    }
}
