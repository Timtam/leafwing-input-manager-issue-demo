use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::{
        default, in_state, App, AppExtStates, Commands, DefaultPlugins, Entity, IntoSystemConfigs,
        IntoSystemSetConfigs, KeyCode, NextState, OnEnter, OnExit, Plugin, PluginGroup, Query,
        Reflect, ResMut, Startup, States, SystemSet, Update, Window, WindowPlugin, With,
    },
};
use leafwing_input_manager::prelude::{
    ActionState, Actionlike, InputControlKind, InputManagerBundle, InputManagerPlugin, InputMap,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum MenuSet {
    PreInput,
    Input,
    PostInput,
    PreAction,
    Action,
    PostAction,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, States)]
pub enum GameState {
    #[default]
    Uninitialized,
    Menu,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MenuAction>::default())
            .configure_sets(
                Update,
                (
                    MenuSet::PreInput,
                    MenuSet::Input,
                    MenuSet::PostInput,
                    MenuSet::PreAction,
                    MenuSet::Action,
                    MenuSet::PostAction,
                )
                    .chain(),
            )
            // only listen for input while a menu is active
            .add_systems(
                OnEnter(GameState::Menu),
                install_menu_keyhook.in_set(MenuSet::PreInput),
            )
            // handle keyboard input
            .add_systems(
                Update,
                process_menu_keyhook
                    .run_if(in_state(GameState::Menu))
                    .in_set(MenuSet::Input),
            )
            .add_systems(OnExit(GameState::Menu), uninstall_menu_keyhook);
    }
}

// possible actions in menus
#[derive(Copy, Clone, Debug, PartialEq, Eq, Reflect, Hash)]
enum MenuAction {
    Previous,
    Next,
    Click,
}

impl Actionlike for MenuAction {
    fn input_control_kind(&self) -> InputControlKind {
        InputControlKind::Button
    }
}

impl MenuAction {
    fn default_input_map() -> InputMap<Self> {
        let mut im = InputMap::default();
        im.insert(Self::Previous, KeyCode::ArrowUp);
        im.insert(Self::Next, KeyCode::ArrowDown);
        im.insert(Self::Click, KeyCode::Enter);
        im
    }
}

fn install_menu_keyhook(
    mut commands: Commands,
    query: Query<Entity, With<ActionState<MenuAction>>>,
) {
    if query.get_single().is_err() {
        commands.spawn(InputManagerBundle::<MenuAction>::with_map(
            MenuAction::default_input_map(),
        ));
    }
}

fn uninstall_menu_keyhook(
    mut commands: Commands,
    query: Query<Entity, With<ActionState<MenuAction>>>,
) {
    if let Ok(e) = query.get_single() {
        commands.entity(e).despawn();
    }
}

fn process_menu_keyhook(q_actions: Query<&ActionState<MenuAction>>) {
    if let Some(action) = q_actions.get_single().ok() {
        if action.just_pressed(&MenuAction::Previous) {
            println!("previous");
        } else if action.pressed(&MenuAction::Next) {
            println!("next");
        } else if action.just_pressed(&MenuAction::Click) {
            println!("clicked");
        }
    }
}

fn main() {
    let mut app = App::new();

    app.edit_schedule(Update, |schedule| {
        schedule.set_build_settings(ScheduleBuildSettings {
            ambiguity_detection: LogLevel::Warn,
            ..default()
        });
    })
    .add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test app".into(),
                ..default()
            }),
            ..default()
        }),
        MenuPlugin,
    ))
    .init_state::<GameState>()
    .add_systems(Startup, setup);

    app.run();
}

fn setup(mut ns: ResMut<NextState<GameState>>) {
    ns.set(GameState::Menu);
}
