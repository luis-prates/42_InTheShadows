use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (grab_mouse, cursor_recenter));
    }
}

// This system grabs the mouse when the left mouse button is pressed
// and releases it when the escape key is pressed
fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Right) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn cursor_recenter(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    if primary_window.cursor.grab_mode == CursorGrabMode::Locked {
        let center = Vec2::new(primary_window.width() / 2.0, primary_window.height() / 2.0);
        primary_window.set_cursor_position(Some(center));
    }
}
