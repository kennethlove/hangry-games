use dioxus::prelude::*;
use dioxus::desktop::{use_global_shortcut, Config, LogicalSize, WindowBuilder};
use dioxus_logger::tracing::Level;
use hangry_games::gui::components::*;
use hangry_games::gui::router::Routes;
use hangry_games::gui::states::HGState;
use hangry_games::gui::functions::list_of_games;

fn main() {
    dioxus_logger::init(Level::INFO).expect("logger failed to init");
    let mut head = r#"<script src="https://cdn.tailwindcss.com"></script>
        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Orbitron:wght@500&display=swap" rel="stylesheet">
        <link href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined" rel="stylesheet" />"#.to_string();

    if cfg!(target_family = "windows") {
        head.push_str(r#"<link rel="stylesheet" href="assets/hangry-games.css">"#);
    } else {
        head.push_str(r#"<link rel="stylesheet" href="hangry-games.css">"#);
    }

    let config = Config::new()
        .with_custom_head(head)
        .with_window(
            WindowBuilder::new()
                .with_resizable(true)
                .with_title("The Hangry Games")
                .with_min_inner_size(LogicalSize::new(800.0, 600.0))
        )
        .with_custom_index(
        r#"
<!DOCTYPE html>
<html>
  <head>
    <title>Dioxus app</title>
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
  </head>
  <body>
    <div id="main"></div>
  </body>
</html>
        "#
                .into());
    LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(app)
}

fn app() -> Element {
    use_context_provider(|| Signal::new(HGState { games: list_of_games() }));

    use_global_shortcut("CmdOrCtrl+Q", move || {
        std::process::exit(0);
    }).expect("Failed to register global quit shortcut");

    rsx! {
        div {
            class: "container mx-auto mt-6 p-4 bg-gray-200/50 dark:bg-gray-900/50 rounded-lg border border-1 border-orange-400 backdrop-blur-md",
            header::Header {}
            Router::<Routes> {}
        }
    }
}
