#[derive(Debug, Clone, Copy)]
pub enum GameMode {
    New,
    Menu,
    Running,
    Load,
    Map,
    MapEditor,
    Quit
}