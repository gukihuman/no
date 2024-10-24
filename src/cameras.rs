use crate::*;
use bevy::{
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    window::WindowResized,
};
const CANVAS_INTERNAL_WIDTH: f32 = 512.;
const CANVAS_INTERNAL_HEIGHT: f32 = 288.;
pub const CANVAS_LAYER: RenderLayers = RenderLayers::layer(0); // settings resolution
pub const UI_LAYER: RenderLayers = RenderLayers::layer(1); // actual screen resolution
#[derive(Resource)]
pub struct CanvasResource {
    size: Extent3d,
    pub scale: f32,
}
#[derive(Component)]
struct Canvas;
pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cameras)
            .add_systems(Update, fit_canvas);
    }
}
pub fn setup_cameras(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    settings: Res<settings::GameSettings>,
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
        CANVAS_LAYER,
    ));
    commands.spawn((
        SpriteBundle {
            texture: image_handle.clone(),
            ..default()
        },
        Canvas,
        UI_LAYER,
    ));
    commands.spawn((Camera2dBundle::default(), UI_LAYER));
    let scale_x = canvas_size.width as f32 / CANVAS_INTERNAL_WIDTH;
    let scale_y = canvas_size.height as f32 / CANVAS_INTERNAL_HEIGHT;
    commands.insert_resource(CanvasResource {
        size: canvas_size,
        scale: match settings.window.background_image.as_str() {
            "fit" => scale_x.min(scale_y),
            "cover" => scale_x.max(scale_y),
            _ => scale_x.min(scale_y),
        },
    });
}
fn fit_canvas(
    mut resize_events: EventReader<WindowResized>,
    mut transforms: Query<&mut Transform, With<Canvas>>,
    canvas: Res<CanvasResource>,
) {
    for event in resize_events.read() {
        let scale_x = event.width / canvas.size.width as f32;
        let scale_y = event.height / canvas.size.height as f32;
        let scale = scale_x.min(scale_y);
        if let Ok(mut transform) = transforms.get_single_mut() {
            transform.scale = Vec3::new(scale, scale, 1.);
        }
    }
}
