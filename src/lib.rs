#![allow(non_snake_case)]

mod id;

use std::collections::BTreeMap;
use dioxus::prelude::*;
use id::ID;

#[derive(Debug, Clone)]
struct ToastManagerItem {
    info: ToastInfo,
    hide_after: Option<i64>,
}

#[derive(Debug)]
pub struct ToastManager {
    list: BTreeMap<usize, ToastManagerItem>,
    maximum_toast: u8,
    id_manager: ID,
}

impl ToastManager {
    pub fn new(maximum_toast: u8) -> Self {
        Self {
            list: BTreeMap::new(),
            maximum_toast,
            id_manager: ID::new(),
        }
    }

    pub fn popup(&mut self, info: ToastInfo) -> usize {
        let toast_id = self.id_manager.add();

        if self.list.len() >= self.maximum_toast.into() {
            if let Some((id, _)) = self.list.pop_first() {
                println!("Deleted Toast ID: {:?}", id);
            }
        }

        let hide_after = info
            .hide_after
            .map(|duration| chrono::Local::now().timestamp() + duration as i64);

        self.list.insert(toast_id, ToastManagerItem { info, hide_after });

        toast_id
    }

    pub fn remove(&mut self, id: usize) {
        self.list.remove(&id);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl Default for ToastManager {
    fn default() -> Self {
        Self::new(6)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Position {
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Icon {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone)]
pub struct ToastInfo {
    pub heading: Option<String>,
    pub context: String,
    pub allow_toast_close: bool,
    pub position: Position,
    pub icon: Option<Icon>,
    pub hide_after: Option<usize>,
}

impl ToastInfo {
    fn new(text: &str, heading: Option<&str>, position: Position, icon: Option<Icon>) -> Self {
        Self {
            heading: heading.map(String::from),
            context: text.to_string(),
            allow_toast_close: true,
            position,
            icon,
            hide_after: Some(6),
        }
    }

    pub fn simple(text: &str) -> Self {
        Self::new(text, None, Position::BottomLeft, None)
    }

    pub fn success(text: &str, heading: &str) -> Self {
        Self::new(text, Some(heading), Position::BottomLeft, Some(Icon::Success))
    }

    pub fn warning(text: &str, heading: &str) -> Self {
        Self::new(text, Some(heading), Position::BottomLeft, Some(Icon::Warning))
    }

    pub fn info(text: &str, heading: &str) -> Self {
        Self::new(text, Some(heading), Position::BottomLeft, Some(Icon::Info))
    }

    pub fn error(text: &str, heading: &str) -> Self {
        Self::new(text, Some(heading), Position::BottomLeft, Some(Icon::Error))
    }
}

#[derive(Props, Copy, Clone, PartialEq)]
pub struct ToastFrameProps {
    manager: Signal<ToastManager>,
}

pub fn ToastFrame(props: ToastFrameProps) -> Element {
    let mut manager = props.manager;
    
    use_future(move || {
        async move {
            loop {
                let now = chrono::Local::now().timestamp();
                manager.write().list.retain(|_, item| {
                    item.hide_after.map_or(true, |hide_after| now < hide_after)
                });
                time_sleep(100).await;
            }
        }
    });
    
    let mut bottom_left_ele: Vec<Option<VNode>> = vec![];
    let mut bottom_right_ele: Vec<Option<VNode>> = vec![];
    let mut top_left_ele: Vec<Option<VNode>> = vec![];
    let mut top_right_ele: Vec<Option<VNode>> = vec![];

    for (id, item) in manager.read().list.iter() {
        let current_id = *id;

        let icon_class = if let Some(icon) = &item.info.icon {
            let mut class = String::from("has-icon ");

            match icon {
                Icon::Success => class.push_str("icon-success"),
                Icon::Warning => class.push_str("icon-warning"),
                Icon::Error => class.push_str("icon-error"),
                Icon::Info => class.push_str("icon-info"),
            }

            class
        } else {
            String::new()
        };

        let element = rsx! {
            div { class: "toast-single {icon_class}", id: "{id}",
                if item.info.allow_toast_close {
                    div {
                        class: "close-toast-single",
                        onclick: move |_| {
                            manager.write().list.remove(&current_id);
                        },
                        "×"
                    }
                }
                if let Some(v) = &item.info.heading {
                    h2 { class: "toast-heading", "{v}" }
                }
                span { dangerous_inner_html: "{item.info.context}" }
            }
        };
        
        match item.info.position {
            Position::BottomLeft => bottom_left_ele.push(element),
            Position::BottomRight => bottom_right_ele.push(element),
            Position::TopLeft => top_left_ele.push(element),
            Position::TopRight => top_right_ele.push(element),
        }
    }

    rsx! {
        div { class: "toast-scope",
            style { {include_str!("./assets/toast.css")} }
            div { class: "toast-wrap bottom-left", id: "wrap-bottom-left",
                {bottom_left_ele.into_iter()}
            }
            div { class: "toast-wrap bottom-right", id: "wrap-bottom-right",
                {bottom_right_ele.into_iter()}
            }
            div { class: "toast-wrap top-left", id: "wrap-top-left", {top_left_ele.into_iter()} }
            div { class: "toast-wrap top-right", id: "wrap-top-right", {top_right_ele.into_iter()} }
        }
    }
}

#[cfg(feature = "web")]
async fn time_sleep(interval: usize) {
    gloo_timers::future::TimeoutFuture::new(interval as u32).await;
}

#[cfg(feature = "desktop")]
async fn time_sleep(interval: usize) {
    tokio::time::sleep(tokio::time::Duration::from_millis(interval as u64)).await;
}
