use bevy::prelude::*;
use std::num::NonZero;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, animation_system)
        .add_systems(Update, move_player)
        .add_systems(Update, movement_system)
        .add_systems(Update, set_player_animation)
        .run();
}

#[derive(Clone, Copy)]
pub enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub enum Actions {
    WALKING,
    IDLE,
}

#[derive(Component, Deref, DerefMut)]
pub struct Action(Actions);

#[derive(Component)]
struct Player {
    max_life: usize,
    life: usize,
    stats: Stats,
}

#[derive(Component)]
struct Stats {
    movement_speed: f32,
}

#[derive(Component)]
struct Movement {
    x: f32,
    y: f32,
    direction: Directions,
}
impl Movement {
    fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}

#[derive(Component)]
struct Animation {
    state: AnimationState,
    timer: AnimationTimer,
    slice: AnimationSlice,
    indices: AnimationIndices,
    frames: usize,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}
#[derive(Component)]
struct AnimationState {
    action: Action,
    direction: Directions,
}
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
#[derive(Component)]
struct AnimationSlice {
    first: usize,
    last: usize,
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        projection: OrthographicProjection {
            near: -100.0,
            far: 100.0,
            scale: 0.250,
            ..default()
        },
        ..default()
    });

    let layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
    let texture_atlas_layout = texture_atlas_layout.add(layout);

    commands.spawn((
        SpriteBundle {
            texture: assets.load("Characters/Basic Charakter Spritesheet.png"),
            transform: Transform::from_xyz(0.0, 0.0, -10.0),

            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            ..default()
        },
        Animation {
            timer: AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            indices: AnimationIndices { first: 1, last: 15 },
            slice: AnimationSlice { first: 0, last: 3 },
            state: AnimationState {
                direction: Directions::DOWN,
                action: Action(Actions::IDLE),
            },
            frames: 4,
        },
        Movement {
            x: 0.0,
            y: 0.0,
            direction: Directions::DOWN,
        },
        Player {
            max_life: 100,
            life: 100,
            stats: Stats {
                movement_speed: 50.0,
            },
        },
    ));
}

fn animation_system(time: Res<Time>, mut query: Query<(&mut Animation, &mut TextureAtlas)>) {
    for (mut animation, mut atlas) in &mut query {
        if !(animation.slice.first <= atlas.index && atlas.index < animation.slice.last) {
            atlas.index = animation.slice.first;
        } else {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                atlas.index += 1
            }
        }
    }
}

fn set_player_animation(mut query: Query<(&mut Animation, &Movement)>) {
    for (mut animation, movement) in &mut query {
        animation.state.direction = movement.direction;

        if (movement.x != 0.0) | (movement.y != 0.0) {
            animation.state.action.0 = Actions::WALKING;
        } else if (movement.x == 0.0) | (movement.y == 0.0) {
            animation.state.action.0 = Actions::IDLE;
        }

        match movement.direction {
            Directions::DOWN => {
                animation.slice.first = 0;
                animation.slice.last = 3;
            }
            Directions::UP => {
                animation.slice.first = 4;
                animation.slice.last = 7;
            }
            Directions::LEFT => {
                animation.slice.first = 8;
                animation.slice.last = 11;
            }

            Directions::RIGHT => {
                animation.slice.first = 12;
                animation.slice.last = 15;
            }
        }
    }
}

fn move_player(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<(&Player, &mut Movement)>) {
    for (player, mut movement) in &mut query {
        movement.clear();
        for key in keyboard.get_pressed() {
            match key {
                KeyCode::KeyW => {
                    movement.y += player.stats.movement_speed;
                    movement.direction = Directions::UP
                }
                KeyCode::KeyA => {
                    movement.x -= player.stats.movement_speed;
                    movement.direction = Directions::LEFT;
                }
                KeyCode::KeyS => {
                    movement.y -= player.stats.movement_speed;
                    movement.direction = Directions::DOWN;
                }
                KeyCode::KeyD => {
                    movement.x += player.stats.movement_speed;
                    movement.direction = Directions::RIGHT;
                }
                _ => (),
            }
        }
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &mut Movement)>) {
    for (mut transform, movement) in &mut query {
        transform.translation.x += movement.x * time.delta_seconds();
        transform.translation.y += movement.y * time.delta_seconds();
    }
}
