use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

use crate::state;
use state::{Action, State};

pub trait Component<B: Backend> {
    fn render(&self, state: &State, f: &mut Frame<B>, rect: Rect);
}

pub struct TypeTestView;

impl TypeTestView {
    pub fn new() -> TypeTestView {
        TypeTestView {}
    }

    fn get_lines(&self, s: &State) -> Vec<Line<'_>> {
        let mut rres = Vec::new();
        let mut current = Vec::new();

        let state = &s.type_test;

        for i in 0..state.target.len() {
            let target_char = state.target.get(i);
            let player_char = state.player_moves.get(i);

            let c = target_char
                .map(|t| match t {
                    ' ' => String::from("␣"),
                    '\n' => String::from("↵"),
                    _ => t.to_string(),
                })
                .unwrap();

            match (target_char, player_char) {
                (Some(t), Some(p)) if t == p && *t != '↵' => {
                    current.push(Span::styled(c, Style::default().fg(Color::Green)));
                }
                (Some(t), Some(p)) if t != p && *t != '↵' => {
                    current.push(Span::styled(c, Style::default().fg(Color::Red)));
                }
                _ => {
                    current.push(Span::styled(c, Style::default().fg(Color::White)));
                }
            };
            if i == state.cursor {
                let j = current.len() - 1;
                current[j] = current[j].clone().bg(Color::Blue);
            }

            if (state.target.get(i).unwrap() == &'\n') {
                rres.push(Line::from(current.clone()));
                current.clear();
            }
        }
        rres
    }
}

impl<B: Backend> Component<B> for TypeTestView {
    fn render(&self, state: &State, frame: &mut Frame<B>, rect: Rect) {
        let area = frame.size();

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(frame.size());

        let first_column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(main_layout[0]);

        let lines = self.get_lines(&state);

        let aps = state
            .type_test
            .actions_per_seconds()
            .map_or(String::from("Undefined"), |aps| aps.to_string());

        let block = Block::default()
            .borders(Borders::ALL)
            .padding(Padding::new(2, 2, 2, 2));
        frame.render_widget(
            Paragraph::new(lines.clone())
                .block(block.clone().borders(Borders::ALL))
                .wrap(Wrap { trim: true }),
            first_column_layout[0], // frame.size(),
        );

        let stats = state.type_test.stats();
        let aps = stats
            .aps
            .map_or(String::from("Undefined"), |x| x.to_string());
        let words_per_minute = stats
            .words_per_minute
            .map_or(String::from("Undefined"), |x| x.to_string());
        let precision = stats
            .precision
            .map_or(String::from("Undefined"), |x| (x * 100.0).to_string());

        let stats = vec![
            Line::from(vec![
                Span::styled("APS: ", Style::default().fg(Color::White)),
                Span::styled(aps, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("WPM: ", Style::default().fg(Color::White)),
                Span::styled(words_per_minute, Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Precision: ", Style::default().fg(Color::White)),
                Span::styled(precision, Style::default().fg(Color::Green)),
            ]),
        ];

        // frame.render_widget(
        //     Paragraph::new(lines.clone())
        //         .block(Block::default().borders(Borders::ALL))
        //         .wrap(Wrap { trim: true }),
        //     first_column_layout[1], // frame.size(),
        // );

        frame.render_widget(
            Paragraph::new(stats).block(Block::default().borders(Borders::ALL)),
            main_layout[1], // frame.size(),
        );
    }
}
