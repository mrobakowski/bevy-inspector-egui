use bevy::prelude::*;
use bevy_inspector_egui::widgets::InspectorQuery;

type RootUINode = InspectorQuery<Entity, (With<Node>, Without<Parent>)>;

/// This example illustrates how to modify UI using the inspector.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_inspector_egui::InspectorPlugin::<RootUINode>::new())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            background_color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                // left vertical fill (border)
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        // left vertical fill (content)
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn(TextBundle {
                                style: Style {
                                    margin: UiRect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::from_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                        font_size: 30.0,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..Default::default()
                            });
                        });
                });
            // right vertical fill
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                    ..Default::default()
                },
                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..Default::default()
            });
            // absolute positioning
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Px(200.0)),
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            left: Val::Px(210.0),
                            bottom: Val::Px(10.0),
                            ..Default::default()
                        },
                        border: UiRect::all(Val::Px(20.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.4, 0.4, 1.0).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            ..Default::default()
                        },
                        background_color: Color::rgb(0.8, 0.8, 1.0).into(),
                        ..Default::default()
                    });
                });
            // render order test: reddest in the back, whitest in the front (flex center)
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    background_color: (Color::NONE).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                ..Default::default()
                            },
                            background_color: Color::rgb(1.0, 0.0, 0.0).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(20.0),
                                        bottom: Val::Px(20.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                background_color: Color::rgb(1.0, 0.3, 0.3).into(),
                                ..Default::default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(40.0),
                                        bottom: Val::Px(40.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                background_color: Color::rgb(1.0, 0.5, 0.5).into(),
                                ..Default::default()
                            });
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(60.0),
                                        bottom: Val::Px(60.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                background_color: Color::rgb(1.0, 0.7, 0.7).into(),
                                ..Default::default()
                            });
                            // alpha test
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                                    position_type: PositionType::Absolute,
                                    position: UiRect {
                                        left: Val::Px(80.0),
                                        bottom: Val::Px(80.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                background_color: Color::rgba(1.0, 0.9, 0.9, 0.4).into(),
                                ..Default::default()
                            });
                        });
                });
            // bevy logo (flex center)
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                background_color: (Color::NONE).into(),
                ..Default::default()
            });
        });
}
