use crate::args::Args;
use crate::widget::style::Color;
use tui::style::Color as TuiColor;

/// Application states (flags) for managing the launcher.
#[derive(Clone, Debug)]
pub struct State {
	/// Is app running?
	pub running: bool,
	/// Is app colored?
	pub colored: bool,
	/// Accent color of the app.
	pub color: TuiColor,
	/// Is the options menu (popup) showing?
	pub show_options: bool,
	/// Is the splash screen showing?
	pub show_splash: bool,
}

impl Default for State {
	fn default() -> Self {
		Self {
			running: true,
			colored: false,
			color: Color::default().get(),
			show_options: false,
			show_splash: false,
		}
	}
}

impl<'a> From<&'a Args> for State {
	fn from(args: &'a Args) -> Self {
		State {
			colored: args.style == *"colored",
			color: args.color.get(),
			show_splash: args.splash,
			..Self::default()
		}
	}
}

impl State {
	/// Reverts back the values to default.
	pub fn refresh(&mut self) {
		let colored = self.colored;
		*self = Self::default();
		self.colored = colored;
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use pretty_assertions::assert_eq;
	#[test]
	fn test_app_state() {
		let mut state = State::default();
		state.refresh();
		assert_eq!(true, state.running);
		assert_eq!(false, state.colored);
		assert_eq!(TuiColor::Gray, state.color);
		assert_eq!(false, state.show_options);
		assert_eq!(false, state.show_splash);
	}
}
