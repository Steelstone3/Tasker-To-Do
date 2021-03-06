use crate::application::task_state::AppMsg;
use crate::application::task_view::TaskWidgets;
use gtk::prelude::{BoxExt, CheckButtonExt};
use relm4::factory::{FactoryPrototype, FactoryVec};
use relm4::{gtk, send, Sender, WidgetPlus};

pub struct Task {
    pub name: String,
    pub completed: bool,
}

impl FactoryPrototype for Task {
    type Factory = FactoryVec<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::Box;
    type View = gtk::ListBox;
    type Msg = AppMsg;

    fn init_view(&self, key: &usize, sender: Sender<Self::Msg>) -> Self::Widgets {
        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .build();
        let checkbox = gtk::CheckButton::builder().active(false).build();
        let label = gtk::Label::new(Some(&self.name));

        assert!(!self.completed);

        checkbox.set_margin_all(12);
        label.set_margin_all(12);

        hbox.append(&checkbox);
        hbox.append(&label);

        let index = *key;
        checkbox.connect_toggled(move |checkbox| {
            send!(sender, AppMsg::SetCompleted((index, checkbox.is_active())));
        });

        TaskWidgets { label, hbox }
    }

    fn position(&self, _key: &usize) {}

    fn view(&self, _key: &usize, widgets: &Self::Widgets) {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
        widgets.label.set_attributes(Some(&attrs));
    }

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.hbox
    }
}
