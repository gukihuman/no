use crate::settings::GameSettings;
use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras)
            .add_systems(Update, fit_canvas);
    }
}
const BACKGROUND_WIDTH: f32 = 512.;
const BACKGRAUND_HEIGHT: f32 = 288.;
pub const CANVAS_LAYER: RenderLayers = RenderLayers::layer(0); // settings resolution
pub const OUTER_LAYER: RenderLayers = RenderLayers::layer(1); // actual screen resolution
#[derive(Resource)]
pub struct CanvasScaleFit(pub f32);
#[derive(Resource)]
pub struct CanvasScaleCover(pub f32);
#[derive(Resource)]
pub struct OuterScale(pub f32);
#[derive(Component)]
pub struct CanvasCamera;
#[derive(Component)]
pub struct OuterCamera;
#[derive(Component)]
struct Canvas;
pub fn setup_cameras(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    settings: Res<GameSettings>,
) {
    let canvas_size = Extent3d {
        width: settings.window.width,
        height: settings.window.height,
        ..default()
    };
    let mut canvas = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: canvas_size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    canvas.resize(canvas_size);
    let image_handle = images.add(canvas);
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            ..default()
        },
        CanvasCamera,
        CANVAS_LAYER,
    ));
    commands.spawn((
        SpriteBundle {
            texture: image_handle.clone(),
            ..default()
        },
        Canvas,
        OUTER_LAYER,
    ));
    commands.spawn((Camera2dBundle::default(), OuterCamera, OUTER_LAYER));
    let scale_x = canvas_size.width as f32 / BACKGROUND_WIDTH;
    let scale_y = canvas_size.height as f32 / BACKGRAUND_HEIGHT;
    commands.insert_resource(CanvasScaleFit(scale_x.min(scale_y)));
    commands.insert_resource(CanvasScaleCover(scale_x.max(scale_y)));
    commands.insert_resource(OuterScale(1.));
}
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut transforms: Query<&mut Transform, With<Canvas>>,
    mut outer_scale: ResMut<OuterScale>,
    settings: Res<GameSettings>,
) {
    for event in resize_events.read() {
        let scale_x = event.width / settings.window.width as f32;
        let scale_y = event.height / settings.window.height as f32;
        outer_scale.0 = scale_x.min(scale_y);
        if let Ok(mut transform) = transforms.get_single_mut() {
            transform.scale = Vec3::new(outer_scale.0, outer_scale.0, 1.);
        }
    }
}
