use ratatui::layout::{Constraint, Layout};
use ratatui::widgets::Block;
// use ratatui::widgets::Paragraph;
use ratatui::{Frame};
use tui_tree_widget::{Tree, TreeItem, TreeState};

use crate::app::App;

pub fn render(frame: &mut Frame, app: &mut App) {
    // let greeting = Paragraph::new(format!("Spiders TUI for {}", app.title));
    // frame.render_widget(greeting, frame.area());
    let chunks = Layout::horizontal([Constraint::Percentage(20), Constraint::Percentage(80)]).split(frame.area());
    let mut state = TreeState::default();
    let items =  vec![
        TreeItem::new_leaf("a", "Alfa"),
        TreeItem::new(
            "b",
            "Bravo",
            vec![
                TreeItem::new_leaf("c", "Charlie"),
                TreeItem::new(
                    "d",
                    "Delta",
                    vec![
                        TreeItem::new_leaf("e", "Echo"),
                        TreeItem::new_leaf("f", "Foxtrot"),
                    ],
                )
                    .expect("all item identifiers are unique"),
                TreeItem::new_leaf("g", "Golf"),
            ],
        )
            .expect("all item identifiers are unique"),
        TreeItem::new_leaf("h", "Hotel"),
        TreeItem::new(
            "i",
            "India",
            vec![
                TreeItem::new_leaf("j", "Juliett"),
                TreeItem::new_leaf("k", "Kilo"),
                TreeItem::new_leaf("l", "Lima"),
                TreeItem::new_leaf("m", "Mike"),
                TreeItem::new_leaf("n", "November"),
            ],
        )
            .expect("all item identifiers are unique"),
        TreeItem::new_leaf("o", "Oscar"),
        TreeItem::new(
            "p",
            "Papa",
            vec![
                TreeItem::new_leaf("q", "Quebec"),
                TreeItem::new_leaf("r", "Romeo"),
                TreeItem::new_leaf("s", "Sierra"),
                TreeItem::new_leaf("t", "Tango"),
                TreeItem::new_leaf("u", "Uniform"),
                TreeItem::new(
                    "v",
                    "Victor",
                    vec![
                        TreeItem::new_leaf("w", "Whiskey"),
                        TreeItem::new_leaf("x", "Xray"),
                        TreeItem::new_leaf("y", "Yankee"),
                    ],
                )
                    .expect("all item identifiers are unique"),
            ],
        )
            .expect("all item identifiers are unique"),
        TreeItem::new_leaf("z", "Zulu"),
    ];

    let ltree = Tree::new(&items)
        .expect("all item identifiers are unique")
        .block(Block::bordered().title("Snooper"));

    frame.render_stateful_widget(ltree, chunks[0], &mut state);
}
