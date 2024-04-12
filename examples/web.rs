use dioxus::prelude::*;
use dioxus_toast::{ToastInfo, ToastManager};
use fermi::{use_atom_ref, use_init_atom_root, AtomRef};

fn main() {
    dioxus_web::launch(app)
}

static TOAST_MANAGER: AtomRef<ToastManager> = fermi::AtomRef(|_| ToastManager::default());

fn app(cx: Scope) -> Element {
    use_init_atom_root(&cx);

    std::panic::set_hook(Box::new(|info| {
        println!("Panic: {}", info);
    }));

    let toast = use_atom_ref(&cx, &TOAST_MANAGER);

    cx.render(rsx! {
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
                            hide_after: None,
                        });
                },
                "Top Right"
            }
        }
    })
}
