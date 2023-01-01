use bevy::prelude::*;

#[cfg(feature = "editor")]
use bevy_editor_pls::prelude::*;

#[cfg(debug_assertions)]
use bevy::diagnostic::LogDiagnosticsPlugin;

/// Plugin with debugging utility intended for use during development only.
/// Will not do anything when used in a release build.
pub struct DevPlugin;

impl Plugin for DevPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "editor")]
        {
            app.add_plugin(EditorPlugin);
        }
        #[cfg(debug_assertions)]
        {
            app.add_plugin(LogDiagnosticsPlugin::default());
        }

        #[cfg(not(any(feature = "editor", debug_assertions)))]
        {
            // Suppress warning of unused app in release builds.
            let _ = app;
        }
    }
}
