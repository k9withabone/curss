use dioxus::prelude::*;

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div { class: "container mx-auto", text_align: "center",
            img { src: "/logo.svg", alt: "curss logo", class: "mx-auto" }
            h3 { "RSS Feed Aggregator" }
            p {
                "Curss is a minimal RSS feed aggregator with support for notifications."
            }
        }
    ))
}
