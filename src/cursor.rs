use crate::cameras::{setup_cameras, CanvasScaleFit, OuterCamera, OuterScale, OUTER_LAYER};
use bevy::prelude::*;
pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CursorPosition(Vec2::ZERO))
            .add_systems(Startup, setup.after(setup_cameras))
            .add_systems(Update, update_cursor_position);
    }
}
const CURSOR_Z_INDEX: f32 = 100.;
#[derive(Resource)]
pub struct CursorPosition(pub Vec2);
#[derive(Component)]
pub struct Cursor;
fn setup(asset_server: Res<AssetServer>, mut commands: Commands, mut windows: Query<&mut Window>) {
    windows.single_mut().cursor.visible = false;
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("cursor.webp"),
            ..default()
        },
        OUTER_LAYER,
        Cursor,
    ));
}
pub fn update_cursor_position(
    mut query: Query<&mut Transform, With<Cursor>>,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform), With<OuterCamera>>,
    canvas_scale_fit: Res<CanvasScaleFit>,
    outer_scale: Res<OuterScale>,
    mut cursor_position: ResMut<CursorPosition>,
) {
    let (camera, camera_transform) = cameras.single();
    let Some(outer_cursor) = windows.single().cursor_position() else {
        return;
    };
    let Some(position) = camera.viewport_to_world_2d(camera_transform, outer_cursor) else {
        return;
    };
    let mut transform = query.single_mut();
    transform.translation = position.extend(CURSOR_Z_INDEX);
    transform.scale = Vec3::new(
        canvas_scale_fit.0 * outer_scale.0,
        canvas_scale_fit.0 * outer_scale.0,
        1.,
    );
    cursor_position.0 = position / outer_scale.0;
}
