pub enum AppMsg {
    SetCompleted((usize, bool)),
    AddEntry(String),
}