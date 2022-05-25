use cursive::event::Event;
use cursive::view::{Identifiable, Scrollable, View};
use cursive::views::{LinearLayout, Panel, SelectView, TextView};

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::controllers::{event_to_string, Controllers};
use crate::tab_view::TabView;

pub struct ControllerHelper {
    event: Event,
    description: &'static str,
    cmd: &'static str,
    cmd_short: &'static str,
    args: &'static str,
}

impl std::fmt::Display for ControllerHelper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:<18} {:<11} {:<24} {:<10} {}",
            self.cmd,
            if self.cmd_short.is_empty() {
                "-"
            } else {
                self.cmd_short
            },
            &event_to_string(&self.event),
            self.args,
            self.description
        )
    }
}

fn get_args(controller: &Controllers) -> &'static str {
    match controller {
        Controllers::SortCol => "SortKey",
        Controllers::Filter => "Name",
        Controllers::JForward => "Time",
        Controllers::JBackward => "Time",
        _ => "-",
    }
}

fn get_title() -> Vec<String> {
    vec![
        format!("{:<18}", "Command"),
        format!("{:<11}", "Short Cmd"),
        format!("{:<24}", "Hot Key"),
        format!("{:<10}", "Args"),
        "Description".into(),
    ]
}

// Grab the user customized keymaps and generate helper message
fn fill_controllers(
    v: &mut SelectView<String>,
    event_controllers: Rc<RefCell<HashMap<Event, Controllers>>>,
) {
    // event_controllers can generate helper messages in completely random order base on
    // user's customization. Instead of using it directly, we will generate a cmd-msg map
    // to ensure the order.
    let cmd_map: HashMap<Controllers, ControllerHelper> = event_controllers
        .borrow()
        .iter()
        .map(|(event, controller)| {
            (
                controller.clone(),
                ControllerHelper {
                    event: event.clone(),
                    cmd: controller.command(),
                    cmd_short: controller.cmd_shortcut(),
                    description: "perf_events",
                    args: get_args(controller),
                },
            )
        })
        .collect();
}

fn fill_reserved(v: &mut LinearLayout) {
    let lines = vec![
        " <DOWN>         - scroll down primary display, next command if command palette activated\n",
        " <UP>           - scroll up primary display, last command if command palette activated\n",
        " <PgDn>         - scroll down 15 lines primary display\n",
        " <PgUp>         - scroll up 15 lines primary display\n",
        " <Home>         - scroll to top of primary display\n",
        " <End>          - scroll to end of primary display\n",
        " <Enter>        - collapse/expand cgroup tree, submit command if command palette activated\n",
        " <Ctrl>-r       - refresh the screen",
        " 'P'            - sort by pid (process view only)\n",
        " 'N'            - sort by name (process view only)\n",
        " 'C'            - sort by cpu (cgroup view and process view only)\n",
        " 'M'            - sort by memory (cgroup view and process view only)\n",
        " 'D'            - sort by total disk activity(cgroup view and process view only)\n",
    ];

    for line in lines {
        v.add_child(TextView::new(line));
    }
}

pub fn new(event_controllers: Rc<RefCell<HashMap<Event, Controllers>>>) -> impl View {
    let mut reserved = LinearLayout::vertical();
    fill_reserved(&mut reserved);
    let mut controllers = SelectView::<String>::new();
    fill_controllers(&mut controllers, event_controllers);
    LinearLayout::vertical()
        .child(Panel::new(reserved))
        .child(Panel::new(
            LinearLayout::vertical()
                .child(
                    TabView::new(get_title(), " ", 0 /* pinned titles */)
                        .expect("Failed to construct title tab in help menu"),
                )
                .child(controllers)
                .scrollable()
                .scroll_x(true),
        ))
        .with_name("perf_event_menu")
}
