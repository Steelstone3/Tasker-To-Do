use relm4::gtk;

pub struct Task {
    pub name: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct TaskWidgets {
    pub label: gtk::Label,
    pub hbox: gtk::Box,
}
