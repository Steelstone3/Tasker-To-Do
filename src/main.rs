mod application;

use crate::application::task_state::AppMsg;
use crate::application::task_view::TaskWidgets;
use crate::application::task::Task;
use gtk::prelude::{
    BoxExt, EntryBufferExtManual, EntryExt, GtkWindowExt, OrientableExt, WidgetExt,
};
use relm4::factory::{FactoryVec};
use relm4::{gtk, send, AppUpdate, Model, RelmApp, Sender, WidgetPlus, Widgets};

struct AppModel {
    tasks: FactoryVec<Task>,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = ();
}

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &(), _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::SetCompleted((index, completed)) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.completed = completed;
                }
            }
            AppMsg::AddEntry(name) => {
                self.tasks.push(Task {
                    name,
                    completed: false,
                });
            }
        }
        true
    }
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = gtk::ApplicationWindow {
            set_width_request: 360,
            set_title: Some("Tasker To Do"),

            set_child = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,
                set_margin_all: 12,
                set_spacing: 6,

                append = &gtk::Entry {
                    connect_activate(sender) => move |entry| {
                        let buffer = entry.buffer();
                        send!(sender, AppMsg::AddEntry(buffer.text()));
                        buffer.delete_text(0, None);
                    }
                },

                append = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_min_content_height: 360,
                    set_vexpand: true,
                    set_child = Some(&gtk::ListBox) {
                        factory!(model.tasks),
                    }
                }
            }

        }
    }
}

fn main() {
    let model = AppModel {
        tasks: FactoryVec::new(),
    };
    let relm = RelmApp::new(model);
    relm.run();
}