use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub id: String,
    pub columns: Vec<String>,
    pub command: Option<String>,
    pub action: Option<String>,
    #[serde(default)]
    pub style: ItemStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ItemStyle {
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuConfig {
    pub prompt: String,
    pub columns: Vec<ColumnConfig>,
    pub items: Vec<MenuItem>,
    #[serde(default)]
    pub width: u16,
    #[serde(default)]
    pub height: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnConfig {
    pub title: String,
    pub width: u16,
    #[serde(default)]
    pub align: Alignment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuResponse {
    pub selected: Option<MenuItem>,
    pub action: String,
}
