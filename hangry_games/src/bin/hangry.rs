use dioxus::prelude::*;
use manganis::*;
use hangry_games::animals::Animal;

fn main() {
    launch(app);
}

fn new_animal() -> String {
    Animal::random().to_string()
}

#[component]
fn Header() -> Element {
    rsx! {
        h1 { "Hangry Games" }
    }
}

#[component]
fn Animal() -> Element {
    let mut animal = use_signal(||new_animal());

    rsx! {
        div {
            h2 { {animal} }
            button {
                onclick: move |_| { animal.set(new_animal()) },
                "Random"
            }
        }
    }
}

fn app() -> Element {
    rsx! {
        div {
            Header {}
            Animal {}
            img { src: mg!(image("assets/hangry-games.png").preload()) }
        }
    }
}
pub const LOGO_IMG: ImageAsset = mg!(image("assets/hangry-games.png")
        // Manganis uses the builder pattern inside the macro. You can set the image size in pixels at compile time to send the smallest possible image to the client
        .size(52, 52)
        // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
        .format(ImageType::Avif)
        // You can even tell manganis to preload the image so it's ready to be displayed as soon as it's needed
        .preload());
