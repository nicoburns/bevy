//! Demonstrates how the `AlignItems` and `JustifyContent` properties can be composed to layout text.
use bevy::prelude::*;

const ALIGN_ITEMS_COLOR: Color = Color::rgb(1., 0.066, 0.349);
const JUSTIFY_CONTENT_COLOR: Color = Color::rgb(0.102, 0.522, 1.);
const MARGIN: Val = Val::Px(5.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [1000., 700.].into(),
                title: "Bevy Flex Layout Example".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, spawn_layout)
        .run();
}

fn spawn_layout(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                size: Size::all(Val::Percent(100.)),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {

            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        size: Size::all(Val::Percent(100.0)),
                        grid_template_columns: RepeatedGridTrack::fr(5, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    // spawn one child node for each combination of `AlignItems` and `JustifyContent`
                    let justifications = [
                        (JustifyContent::Start, JustifyItems::Start),
                        (JustifyContent::Center, JustifyItems::Center),
                        (JustifyContent::End, JustifyItems::End),
                        (JustifyContent::SpaceEvenly, JustifyItems::Start),
                        (JustifyContent::SpaceAround, JustifyItems::Start),
                        (JustifyContent::SpaceBetween, JustifyItems::Start),
                    ];
                    let alignments = [
                        AlignItems::Baseline,
                        AlignItems::Start,
                        AlignItems::Center,
                        AlignItems::End,
                        AlignItems::Stretch,
                    ];
                    for justify_content in justifications {
                        for align_items in alignments {
                            spawn_demo_cell(
                                builder,
                                font.clone(),
                                Display::Flex,
                                FlexDirection::Column,
                                align_items,
                                justify_content,
                            );
                        }
                    }
                });

            // Sidebar
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::top(MARGIN),
                        padding: UiRect::all(Val::Px(10.)),
                        size: Size::width(Val::Px(300.)),
                        // justify_content: JustifyContent::Start,
                        align_items: AlignItems::Stretch,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    spawn_label(
                        builder,
                        font.clone(),
                        ALIGN_ITEMS_COLOR,
                        UiRect::bottom(MARGIN),
                        "AlignItems",
                    );
                    spawn_label(
                        builder,
                        font.clone(),
                        JUSTIFY_CONTENT_COLOR,
                        UiRect::default(),
                        "JustifyContent",
                    );
                });
        });
}

fn spawn_demo_cell(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    display: Display,
    flex_direction: FlexDirection,
    align_items: AlignItems,
    (justify_content, justify_items): (JustifyContent, JustifyItems),
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                display,
                flex_direction,
                align_items,
                justify_content,
                justify_items,
                // size: Size::all(Val::Px(100.)),
                margin: UiRect::all(MARGIN),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::DARK_GRAY),
            ..Default::default()
        })
        .with_children(|builder| {
            let labels = [
                (format!("{align_items:?}"), ALIGN_ITEMS_COLOR, 0.),
                match display {
                    Display::Flex => (format!("{justify_content:?}"), JUSTIFY_CONTENT_COLOR, 3.),
                    Display::Grid => (format!("{justify_items:?}"), JUSTIFY_CONTENT_COLOR, 3.),
                    Display::None => unreachable!(),
                }
            ];
            for (text, color, top_margin) in labels {
                spawn_label(
                    builder,
                    font.clone(),
                    color,
                    UiRect::top(Val::Px(top_margin)),
                    &text,
                );
            }
        });
}

fn spawn_label(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    background_color: Color,
    margin: UiRect,
    text: &str,
) {
    // We nest the text within a parent node because margins and padding can't be directly applied to text nodes currently.
    builder
        .spawn(NodeBundle {
            style: Style {
                margin,
                padding: UiRect {
                    top: Val::Px(1.),
                    left: Val::Px(5.),
                    right: Val::Px(5.),
                    bottom: Val::Px(1.),
                },
                grid_row: GridPlacement::start(1),
                grid_column: GridPlacement::start(1),
                ..Default::default()
            },
            background_color: BackgroundColor(background_color),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font,
                    font_size: 16.0,
                    color: Color::BLACK,
                },
            ));
        });
}
