use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

pub fn init() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(180, 180, 120, 255))
        .font_size(30)
        .build();

    let window_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/window_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
        .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/button_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
        .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
        .background_hovered(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/button_hovered_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/button_clicked_background.png"),
                None,
            )
            .unwrap(),
        )
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(40)
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0., 0., 0., 0.))
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .color_selected(Color::from_rgba(190, 190, 190, 255))
        .font_size(50)
        .build();

    Skin {
        editbox_style,
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    }
}

pub fn init_smaller() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .text_color(Color::from_rgba(180, 180, 120, 255))
        .font_size(40)
        .build();

    let window_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/window_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
        .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
        .build();

    let button_style = root_ui()
        .style_builder()
        .background(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/button_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
        .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
        .background_hovered(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/button_hovered_background.png"),
                None,
            )
            .unwrap(),
        )
        .background_clicked(
            Image::from_file_with_format(
                include_bytes!("../assets/ui/Classic/button_clicked_background.png"),
                None,
            )
            .unwrap(),
        )
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(25)
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0., 0., 0., 0.))
        .background(
            Image::from_file_with_format(include_bytes!("../assets/ui/Classic/blank.png"), None)
                .unwrap(),
        )
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .color_selected(Color::from_rgba(190, 190, 190, 255))
        .font_size(28)
        .build();

    Skin {
        editbox_style,
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    }
}
