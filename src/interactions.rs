use crate::{
    actions::Actions,
    cameras::{CanvasScaleCover, CanvasScaleFit},
    cursor::{update_cursor_position, CursorPosition},
    elements::{ElementAction, ElementId, ScaleCover},
    settings::GameSettings,
    views::{ViewMap, ViewStack},
};
use bevy::prelude::*;
pub struct InteractionsPlugin;
impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_interactions.after(update_cursor_position));
    }
}
const HOVER_COLOR: Color = Color::srgba(1.5, 1.5, 1.5, 1.);
#[derive(Component)]
pub struct Interactive {
    pub is_hovered: bool,
    pub actions: Vec<ElementAction>,
    pub color: Color,
}
impl Default for Interactive {
    fn default() -> Self {
        Self {
            is_hovered: false,
            actions: Vec::new(),
            color: Color::WHITE,
        }
    }
}
fn handle_interactions(
    mut query: Query<(
        &mut Interactive,
        &mut Sprite,
        &mut ScaleCover,
        &GlobalTransform,
        &Handle<Image>,
        &ElementId,
    )>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    images: Res<Assets<Image>>,
    canvas_scale_fit: Res<CanvasScaleFit>,
    canvas_scale_cover: Res<CanvasScaleCover>,
    cursor_position: Res<CursorPosition>,
    settings: Res<GameSettings>,
    view_stack: Res<ViewStack>,
    view_map: Res<ViewMap>,
    mut actions: ResMut<Actions>,
) {
    for (mut interactive, mut sprite, scale_cover, transform, image_handle, element_id) in
        &mut query
    {
        if let Some(image) = images.get(image_handle) {
            let mut sprite_size = Vec2::new(image.size().x as f32, image.size().y as f32);
            match settings.window.background_image.as_str() {
                "fit" => sprite_size = sprite_size * canvas_scale_fit.0,
                "cover" => {
                    if scale_cover.0 {
                        sprite_size = sprite_size * canvas_scale_cover.0
                    } else {
                        sprite_size = sprite_size * canvas_scale_fit.0
                    }
                }
                _ => sprite_size = sprite_size * canvas_scale_fit.0,
            };
            let sprite_pos = transform.translation().truncate();
            let half_size = sprite_size / 2.0;
            let left = sprite_pos.x - half_size.x;
            let right = sprite_pos.x + half_size.x;
            let top = sprite_pos.y + half_size.y;
            let bottom = sprite_pos.y - half_size.y;
            let is_within_bounds = cursor_position.0.x >= left
                && cursor_position.0.x <= right
                && cursor_position.0.y >= bottom
                && cursor_position.0.y <= top;
            if is_within_bounds {
                let sprite_local_pos = Vec2::new(
                    ((cursor_position.0.x - left) / sprite_size.x).clamp(0.0, 1.0),
                    1.0 - ((cursor_position.0.y - bottom) / sprite_size.y).clamp(0.0, 1.0),
                );
                let pixel_x = (sprite_local_pos.x * image.size().x as f32) as u32;
                let pixel_y = (sprite_local_pos.y * image.size().y as f32) as u32;
                if let Some(alpha) = get_pixel_alpha(image, pixel_x, pixel_y) {
                    if alpha > 0.1 {
                        let should_handle_actions = if view_stack.0.is_empty() {
                            true
                        } else if let Some(current_view_id) = view_stack.0.last() {
                            view_map
                                .0
                                .get(current_view_id)
                                .map(|view| view.0.contains(&element_id.0))
                                .unwrap_or(false)
                        } else {
                            false
                        };
                        if should_handle_actions && mouse_button.just_pressed(MouseButton::Left) {
                            actions.0.extend(interactive.actions.clone());
                        }
                        if should_handle_actions {
                            interactive.is_hovered = true;
                            sprite.color = HOVER_COLOR;
                            continue;
                        }
                    }
                }
            }
            if interactive.is_hovered {
                interactive.is_hovered = false;
                sprite.color = interactive.color;
            }
        }
    }
}
fn get_pixel_alpha(image: &Image, x: u32, y: u32) -> Option<f32> {
    if x >= image.size().x || y >= image.size().y {
        return None;
    }
    let pixel_idx = ((y * image.size().x + x) * 4 + 3) as usize;
    if pixel_idx >= image.data.len() {
        return None;
    }
    Some(image.data[pixel_idx] as f32 / 255.0)
}
