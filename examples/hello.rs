use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};

fn main() {
    launch(app);
}

static TOAST_MANAGER: GlobalSignal<ToastManager> = Signal::global(|| ToastManager::default());

fn app() -> Element {
    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let mut toast = TOAST_MANAGER.signal();

    rsx! {
        dioxus_toast::ToastFrame { manager: toast }
        div {
            button {
                onclick: move |_| {
                    let _id = toast.write().popup(ToastInfo::simple("hello world"));
                    println!("New Toast ID: {}", _id);
                },
                "Normal Toast"
            }
            button {
                onclick: move |_| {
                    let _id = toast.write().popup(ToastInfo::success("Hello World!", "Success"));
                    println!("New Toast ID: {}", _id);
                },
                "Success Toast"
            }
            button {
                onclick: move |_| {
                    let _id = toast
                        .write()
                        .popup(ToastInfo {
                            heading: Some("top-right".into()),
                            context: "Top Right Toast".into(),
                            allow_toast_close: true,
                            position: dioxus_toast::Position::TopRight,
                            icon: None,
                            hide_after: Some(1),
                        });
                },
                "Top Right"
            }
        }
    }
}
