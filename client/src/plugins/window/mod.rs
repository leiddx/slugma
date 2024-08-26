use bevy::{
	app::{App, Startup, Update},
	ecs::{
		event::EventReader,
		schedule::common_conditions,
		system::{Query, ResMut},
	},
	input::{
		keyboard::{KeyCode, KeyboardInput},
		ButtonInput,
	},
	prelude::IntoSystemConfigs,
	window::{CursorGrabMode, EnabledButtons, Window, WindowFocused, WindowLevel, WindowMode},
};

pub fn setup(mut window: Query<&mut Window>) {
	#[cfg(target_os = "macos")]
	const GRAB_MODE: CursorGrabMode = CursorGrabMode::Locked;

	#[cfg(target_os = "windows")]
	const GRAB_MODE: CursorGrabMode = CursorGrabMode::Confined;

	#[cfg(not(any(target_os = "macos", target_os = "windows")))]
	const GRAB_MODE: CursorGrabMode = CursorGrabMode::None;

	for mut w in window.iter_mut() {
		w.focused = true;
		w.resizable = false;
		w.mode = WindowMode::Windowed;
		w.cursor.grab_mode = GRAB_MODE;

		w.enabled_buttons = EnabledButtons {
			minimize: true,
			maximize: false,
			close:    true,
		};
	}
}

pub fn focus(mut window: Query<&mut Window>, mut window_focused: EventReader<WindowFocused>) {
	for v in window_focused.read() {
		if let Ok(mut w) = window.get_mut(v.window) {
			if v.focused {
				if w.mode == WindowMode::Fullscreen {
					w.set_minimized(false);
					w.set_maximized(true);
				}
			}
			else {
				if w.mode == WindowMode::Fullscreen {
					w.set_minimized(true);
					w.set_maximized(false);
				}
			}
		}
	}
}

pub fn change_mode(mut key_code: ResMut<ButtonInput<KeyCode>>, mut window: Query<&mut Window>) {
	let mut window = window.single_mut();

	let alt = key_code.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]);
	let enter = key_code.any_just_pressed([KeyCode::Enter, KeyCode::NumpadEnter]);

	if alt && enter {
		window.mode = match window.mode {
			| WindowMode::Windowed => WindowMode::Fullscreen,
			| WindowMode::Fullscreen => WindowMode::Windowed,
			| _ => window.mode,
		};

		window.window_level = match window.mode {
			| WindowMode::BorderlessFullscreen => WindowLevel::AlwaysOnBottom,
			| WindowMode::Fullscreen => WindowLevel::AlwaysOnTop,
			| _ => window.window_level,
		};

		key_code.clear();
	}
}



pub struct Plugin;

impl bevy::app::Plugin for Plugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Startup, setup,
		);


		app.add_systems(
			Update,
			(
				focus.run_if(common_conditions::on_event::<WindowFocused>()),
				change_mode.run_if(common_conditions::on_event::<KeyboardInput>()),
			),
		);
	}
}
