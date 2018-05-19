#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum GameMode {
    New,
    Menu,
    Running,
    Load,
    Map,
    MapEditor,
    Quit
}