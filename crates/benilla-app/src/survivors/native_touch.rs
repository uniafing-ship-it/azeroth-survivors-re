//! Experimental iOS-only on-screen controls for Warcraft Survivors.
//! Multitouch is tracked independently; this adapts raw touch coordinates into
//! the existing keyboard-driven player and menu, without a webview.
use bevy::input::touch::Touches;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::collections::HashSet;

#[path = "native_touch_geometry.rs"]
mod geometry;

#[derive(Resource, Default)]
pub(super) struct HeldTouchKeys(HashSet<KeyCode>);

const MANAGED: &[KeyCode] = &[
    KeyCode::KeyW,
    KeyCode::KeyA,
    KeyCode::KeyS,
    KeyCode::KeyD,
    KeyCode::Space,
    KeyCode::KeyF,
    KeyCode::Enter,
    KeyCode::Escape,
    KeyCode::Digit1,
    KeyCode::Digit2,
    KeyCode::Digit3,
];

/// Poll after the Bevy input update, so a touch is visible to game systems in
/// the same frame. Release every virtual key after the finger disappears.
pub(super) fn poll(
    touches: Res<Touches>,
    ui_scale: Res<UiScale>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut keys: ResMut<ButtonInput<KeyCode>>,
    mut held: ResMut<HeldTouchKeys>,
) {
    let Ok(window) = windows.single() else { return };
    let mut down = HashSet::new();
    for touch in touches.iter() {
        let pressed = geometry::touch_keys(
            [window.width(), window.height()],
            ui_scale.0,
            touch.start_position().to_array(),
            touch.position().to_array(),
        );
        for (&key, pressed) in MANAGED.iter().zip(pressed) {
            if pressed {
                down.insert(key);
            }
        }
    }

    for &key in MANAGED {
        if down.contains(&key) {
            keys.press(key);
            held.0.insert(key);
        } else if held.0.remove(&key) {
            keys.release(key);
        }
    }
}

fn draw_control(parent: &mut ChildSpawnerCommands, label: &str, node: Node) {
    parent
        .spawn((
            node,
            BackgroundColor(Color::srgba(0.08, 0.10, 0.16, 0.67)),
            BorderColor::all(Color::srgba(1.0, 0.77, 0.36, 0.8)),
            bevy::ui::FocusPolicy::Pass,
        ))
        .with_children(|slot| {
            slot.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.88, 0.67)),
                TextLayout::new_with_justify(Justify::Center),
                Node {
                    align_self: AlignSelf::Center,
                    margin: UiRect::horizontal(Val::Auto),
                    ..default()
                },
                bevy::ui::FocusPolicy::Pass,
            ));
        });
}

fn at_right(right: f32, bottom: Option<f32>, top: Option<f32>, width: f32, height: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        right: Val::Px(right),
        bottom: bottom.map_or(Val::Auto, Val::Px),
        top: top.map_or(Val::Auto, Val::Px),
        width: Val::Px(width),
        height: Val::Px(height),
        border: UiRect::all(Val::Px(1.0)),
        border_radius: BorderRadius::all(Val::Px(14.0)),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    }
}

/// Display a controls HUD without loading any WoW textures or network assets.
pub(super) fn spawn_controls(mut commands: Commands) {
    commands
        .spawn((
            Name::new("iOS Touch Controls"),
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            GlobalZIndex(850),
            bevy::ui::FocusPolicy::Pass,
        ))
        .with_children(|root| {
            draw_control(
                root,
                "ДВИЖЕНИЕ",
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Px(24.0),
                    bottom: Val::Px(24.0),
                    width: Val::Px(140.0),
                    height: Val::Px(140.0),
                    border: UiRect::all(Val::Px(2.0)),
                    border_radius: BorderRadius::all(Val::Px(70.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            );
            draw_control(root, "ПРЫЖОК", at_right(24.0, Some(24.0), None, 64.0, 64.0));
            draw_control(root, "РЫВОК", at_right(104.0, Some(60.0), None, 64.0, 64.0));
            draw_control(root, "ПАУЗА", at_right(24.0, None, Some(22.0), 64.0, 56.0));
            draw_control(root, "СТАРТ", at_right(104.0, None, Some(22.0), 64.0, 56.0));
            for (i, dx) in [(-78.0), (-22.0), 34.0].into_iter().enumerate() {
                draw_control(
                    root,
                    &format!("{}", i + 1),
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Percent(50.0),
                        bottom: Val::Px(8.0),
                        margin: UiRect::left(Val::Px(dx)),
                        width: Val::Px(44.0),
                        height: Val::Px(44.0),
                        border: UiRect::all(Val::Px(1.0)),
                        border_radius: BorderRadius::all(Val::Px(8.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                );
            }
        });
}
