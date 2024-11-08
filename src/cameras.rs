use crate::{
    actions::handle_actions,
    elements::{spawn_element, ElementId, ElementMap},
    settings::GameSettings,
    steps::{CurrentStepID, StepMap},
    views::{ViewMap, ViewStack},
};
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
        app.insert_resource(OuterResolution {
            width: 0.,
            height: 0.,
        })
        .add_systems(Startup, setup_cameras)
        .add_systems(
            Update,
            (
                fit_canvas.after(reset_canvas),
                reset_canvas.after(handle_actions),
            ),
        );
    }
}
const BACKGROUND_WIDTH: f32 = 2560.;
const BACKGRAUND_HEIGHT: f32 = 1440.;
pub const CANVAS_LAYER: RenderLayers = RenderLayers::layer(0); // settings resolution
pub const OUTER_LAYER: RenderLayers = RenderLayers::layer(1); // actual screen resolution
#[derive(Resource)]
pub struct CanvasScaleFit(pub f32);
#[derive(Resource)]
pub struct CanvasScaleCover(pub f32);
#[derive(Resource)]
pub struct OuterScale(pub f32);
#[derive(Resource)]
pub struct OuterResolution {
    pub width: f32,
    pub height: f32,
}
#[derive(Component)]
pub struct CanvasCamera;
#[derive(Component)]
pub struct OuterCamera;
#[derive(Component)]
pub struct CanvasResetRequest;
#[derive(Component)]
pub struct Canvas;
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
    mut commands: Commands,
    mut resize_events: EventReader<WindowResized>,
    mut transforms: Query<&mut Transform, With<Canvas>>,
    mut outer_scale: ResMut<OuterScale>,
    settings: Res<GameSettings>,
    reset_requests: Query<Entity, With<CanvasResetRequest>>,
    mut outer_resolution: ResMut<OuterResolution>,
) {
    for event in resize_events.read() {
        outer_resolution.width = event.width;
        outer_resolution.height = event.height;
        let scale_x = event.width / settings.window.width as f32;
        let scale_y = event.height / settings.window.height as f32;
        outer_scale.0 = scale_x.min(scale_y);
        if let Ok(mut transform) = transforms.get_single_mut() {
            transform.scale = Vec3::new(outer_scale.0, outer_scale.0, 1.);
        }
    }
    for request_entity in reset_requests.iter() {
        let scale_x = outer_resolution.width / settings.window.width as f32;
        let scale_y = outer_resolution.height / settings.window.height as f32;
        outer_scale.0 = scale_x.min(scale_y);
        if let Ok(mut transform) = transforms.get_single_mut() {
            transform.scale = Vec3::new(outer_scale.0, outer_scale.0, 1.);
        }
        commands.entity(request_entity).despawn();
    }
}
pub fn reset_canvas(
    outer_scale: Res<OuterScale>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut canvas_query: Query<(&mut Handle<Image>, &mut Transform), With<Canvas>>,
    reset_requests: Query<Entity, With<CanvasResetRequest>>,
    mut canvas_camera: Query<&mut Camera, With<CanvasCamera>>,
    step_map: Res<StepMap>,
    elements_query: Query<(Entity, &ElementId)>,
    current_step: Res<CurrentStepID>,
    element_map: ResMut<ElementMap>,
    mut canvas_scale_fit: ResMut<CanvasScaleFit>,
    mut canvas_scale_cover: ResMut<CanvasScaleCover>,
    view_stack: ResMut<ViewStack>,
    view_map: Res<ViewMap>,
    asset_server: Res<AssetServer>,
    settings: ResMut<GameSettings>,
) {
    for _ in reset_requests.iter() {
        for (mut image_handle, mut transform) in canvas_query.iter_mut() {
            let canvas_size = Extent3d {
                width: settings.window.width,
                height: settings.window.height,
                ..default()
            };
            let mut new_canvas = Image {
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
            new_canvas.resize(canvas_size);
            let new_handle = images.add(new_canvas);
            *image_handle = new_handle.clone();
            let mut camera = canvas_camera.single_mut();
            camera.target = RenderTarget::Image(new_handle);
            let scale_x = canvas_size.width as f32 / BACKGROUND_WIDTH;
            let scale_y = canvas_size.height as f32 / BACKGRAUND_HEIGHT;
            canvas_scale_fit.0 = scale_x.min(scale_y);
            canvas_scale_cover.0 = scale_x.max(scale_y);
            transform.scale = Vec3::new(outer_scale.0, outer_scale.0, 1.);
        }
        for (entity, _) in elements_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        if let Some(step) = step_map.0.get(&current_step.0) {
            for element_id in step.0.iter() {
                if let Some(element) = element_map.0.get(element_id) {
                    spawn_element(
                        &mut commands,
                        &asset_server,
                        canvas_scale_fit.0,
                        canvas_scale_cover.0,
                        element_id.clone(),
                        element,
                        &settings,
                    );
                }
            }
        }
        if let Some(view_id) = view_stack.0.last() {
            if let Some(view) = view_map.0.get(view_id) {
                for element_id in view.0.iter() {
                    if let Some(element) = element_map.0.get(element_id) {
                        spawn_element(
                            &mut commands,
                            &asset_server,
                            canvas_scale_fit.0,
                            canvas_scale_cover.0,
                            element_id.clone(),
                            element,
                            &settings,
                        );
                    }
                }
            }
        }
    }
}
