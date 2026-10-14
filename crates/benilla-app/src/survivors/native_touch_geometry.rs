//! Touch hit testing in the same unscaled UI units as the controls' Nodes.
//! Kept engine-free so device geometry regressions can run without WoW data.

/// Order matches native_touch::MANAGED: W, A, S, D, jump, dash, start,
/// pause, upgrade 1, upgrade 2, upgrade 3.
pub(super) fn touch_keys(
    window: [f32; 2],
    scale: f32,
    start: [f32; 2],
    current: [f32; 2],
) -> [bool; 11] {
    let mut down = [false; 11];
    if !scale.is_finite() || scale <= 0.0 {
        return down;
    }
    // Bevy touch positions and window dimensions are logical pixels already:
    // divide by UiScale only, never by the device's display scale factor.
    let [w, h] = window.map(|value| value / scale);
    let [sx, sy] = start.map(|value| value / scale);
    let [cx, cy] = current.map(|value| value / scale);
    let stick = [94.0, h - 94.0];
    if (sx - stick[0]).hypot(sy - stick[1]) <= 100.0 {
        down[0] = cy - stick[1] < -18.0;
        down[1] = cx - stick[0] < -18.0;
        down[2] = cy - stick[1] > 18.0;
        down[3] = cx - stick[0] > 18.0;
        return down;
    }
    // Lock each finger to its initial button, even when it drags away.
    let actions = [
        (4, [w - 56.0, h - 56.0], [66.0, 66.0]),
        (5, [w - 136.0, h - 92.0], [66.0, 66.0]),
        (7, [w - 56.0, 50.0], [66.0, 58.0]),
        (6, [w - 136.0, 50.0], [66.0, 58.0]),
        (8, [w * 0.5 - 56.0, h - 30.0], [44.0, 44.0]),
        (9, [w * 0.5, h - 30.0], [44.0, 44.0]),
        (10, [w * 0.5 + 56.0, h - 30.0], [44.0, 44.0]),
    ];
    for (index, center, size) in actions {
        if (sx - center[0]).abs() <= size[0] * 0.5 && (sy - center[1]).abs() <= size[1] * 0.5 {
            down[index] = true;
            break;
        }
    }
    down
}

#[cfg(test)]
mod tests {
    use super::touch_keys;

    const WINDOWS: [[f32; 2]; 3] = [[852.0, 393.0], [932.0, 430.0], [667.0, 375.0]];

    fn stick([_, h]: [f32; 2], scale: f32) -> [f32; 2] {
        // Center from the rendered Node: left/bottom 24, size 140.
        [
            (24.0 + 140.0 / 2.0) * scale,
            h - (24.0 + 140.0 / 2.0) * scale,
        ]
    }

    #[test]
    fn rendered_joystick_center_is_neutral_at_every_scale() {
        for window in WINDOWS {
            for scale in [window[1] / 768.0, 1.0, 1.25] {
                let center = stick(window, scale);
                assert_eq!(touch_keys(window, scale, center, center), [false; 11]);
            }
        }
    }

    #[test]
    fn rendered_button_centers_select_only_their_own_action() {
        for [w, h] in WINDOWS {
            for scale in [h / 768.0, 1.0, 1.25] {
                // Independently derive centers from the rendered Node margins/sizes.
                let centers = [
                    (4, [w - (24.0 + 32.0) * scale, h - (24.0 + 32.0) * scale]),
                    (5, [w - (104.0 + 32.0) * scale, h - (60.0 + 32.0) * scale]),
                    (7, [w - (24.0 + 32.0) * scale, (22.0 + 28.0) * scale]),
                    (6, [w - (104.0 + 32.0) * scale, (22.0 + 28.0) * scale]),
                    (
                        8,
                        [w / 2.0 + (-78.0 + 22.0) * scale, h - (8.0 + 22.0) * scale],
                    ),
                    (
                        9,
                        [w / 2.0 + (-22.0 + 22.0) * scale, h - (8.0 + 22.0) * scale],
                    ),
                    (
                        10,
                        [w / 2.0 + (34.0 + 22.0) * scale, h - (8.0 + 22.0) * scale],
                    ),
                ];
                for (index, center) in centers {
                    let mut expected = [false; 11];
                    expected[index] = true;
                    assert_eq!(
                        touch_keys([w, h], scale, center, center),
                        expected,
                        "window={w}x{h}, scale={scale}, action={index}"
                    );
                }
            }
        }
    }

    #[test]
    fn joystick_dead_zone_and_diagonals_scale_with_rendered_control() {
        let window = [852.0, 393.0];
        let scale = window[1] / 768.0;
        let center = stick(window, scale);
        let dead = [center[0] + 17.0 * scale, center[1] - 17.0 * scale];
        assert_eq!(touch_keys(window, scale, center, dead), [false; 11]);
        let drag = [center[0] + 40.0 * scale, center[1] - 40.0 * scale];
        let mut expected = [false; 11];
        expected[0] = true;
        expected[3] = true;
        assert_eq!(touch_keys(window, scale, center, drag), expected);
    }

    #[test]
    fn dragging_a_button_finger_does_not_turn_it_into_a_joystick() {
        let window = [852.0, 393.0];
        let scale = window[1] / 768.0;
        let dash = [window[0] - 136.0 * scale, window[1] - 92.0 * scale];
        let mut expected = [false; 11];
        expected[5] = true;
        assert_eq!(
            touch_keys(window, scale, dash, stick(window, scale)),
            expected
        );
    }

    #[test]
    fn invalid_scale_and_unclaimed_touches_do_not_press_keys() {
        for scale in [0.0, -1.0, f32::NAN, f32::INFINITY] {
            assert_eq!(
                touch_keys([852.0, 393.0], scale, [0.0; 2], [0.0; 2]),
                [false; 11]
            );
        }
        assert_eq!(
            touch_keys(
                [852.0, 393.0],
                393.0 / 768.0,
                [426.0, 196.5],
                [426.0, 196.5]
            ),
            [false; 11]
        );
    }
}
