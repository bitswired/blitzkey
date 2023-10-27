use std::vec;

use ratatui::{
    layout,
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Spans, Text},
    widgets::{Block, Borders, Cell, Padding, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::{
    keyboard::{self, Keyboard},
    state::{self, ElementValue},
};
use state::{Action, State};

fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub trait Component<B: Backend> {
    fn render(&mut self, state: &State, f: &mut Frame<B>, rect: Rect);
}

pub struct TypeTestView<'a> {
    render_cache: Option<Vec<Line<'a>>>,
}

impl<'a> TypeTestView<'a> {
    pub fn new() -> TypeTestView<'a> {
        TypeTestView { render_cache: None }
    }

    fn get_keyboard_spans(&self, state: &State) -> Vec<Line<'_>> {
        let Keyboard {
            layout,
            touch_map,
            active_keys,
        } = &state.type_test.keyboard;

        let keys_to_paint = active_keys.keys().collect::<Vec<&char>>();

        let mut lines = Vec::new();
        let mut current_line: Vec<Span<'_>> = Vec::new();

        for (i, c) in layout.chars().enumerate() {
            if c == '\n' {
                lines.push(Line::from(current_line.clone()));
                current_line.clear();
            } else {
                let char_at_i = touch_map.get(&i).unwrap();
                let mut style = Style::default().fg(Color::White);

                if keys_to_paint.contains(&char_at_i) {
                    style = style.bg(Color::Blue);
                }
                let span = Span::styled(c.to_string(), style);
                current_line.push(span);
            }
        }

        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }

        lines
    }

    fn build_render_cache(&mut self, state: &State) {
        let x = state
            .type_test
            .lines
            .iter()
            .map(|l| {
                let padding = l.0;
                let line = &l.1;

                let spans = line.iter().map(|e| {
                    let mut style = Style::default().fg(Color::White);
                    let c = match e.target {
                        ElementValue::Char(c) => c,
                        ElementValue::Newline => '↵',
                    };
                    Span::styled(c.to_string(), style)
                });

                let padding =
                    (0..padding).map(|_| Span::styled("␣", Style::default().fg(Color::Black)));

                let spans = padding.chain(spans).collect::<Vec<Span<'_>>>();

                Line::from(spans)
            })
            .collect::<Vec<Line<'_>>>();

        self.render_cache = Some(x);
    }
}

impl<B: Backend> Component<B> for TypeTestView<'_> {
    fn render(&mut self, state: &State, frame: &mut Frame<B>, rect: Rect) {
        if self.render_cache.is_none() {
            self.build_render_cache(state);
        }

        if let Some((i, j)) = state.type_test.last_modified {
            let padding = state
                .type_test
                .lines
                .get(state.type_test.last_modified.unwrap().0)
                .unwrap()
                .0 as usize;

            let span = &mut self.render_cache.as_mut().unwrap()[i].spans[j + padding];
            span.style.bg = None;

            let elem = &state.type_test.lines[i].1[j];

            match &elem.value {
                Some(v) if *v == elem.target => {
                    // Handle the case where the value is equal to the target
                    span.style.fg = Some(Color::Green);
                }
                Some(v) if *v != elem.target => {
                    // Handle the case where the value is not equal to the target
                    span.style.fg = Some(Color::Red);
                }
                Some(_) => {}
                None => {
                    span.style.fg = Some(Color::White);
                }
            }
        }

        let padding = state
            .type_test
            .lines
            .get(state.type_test.cursor.0)
            .unwrap()
            .0 as usize;

        let (i, j) = state.type_test.cursor;
        self.render_cache.as_mut().unwrap()[i].spans[j + padding]
            .style
            .bg = Some(Color::Blue);

        let area = frame.size();

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
            .split(frame.size());

        // split main_layout[1] 30 70
        let second_column = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(main_layout[1]);

        let first_column_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(main_layout[0]);

        let l: &Vec<Line<'_>> = self.render_cache.as_ref().unwrap();

        let (row, _) = state.type_test.cursor;
        let n_lines = l.len() as i32;

        let half_context = 10;
        let mut i: i32 = 0;
        let mut j: i32 = 0;

        j = row as i32 + half_context;
        i = row as i32 - half_context;
        if i < 0 {
            j -= i;
            i = 0;
        }

        let lines = l[i as usize..std::cmp::min(j, n_lines) as usize].to_vec();

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

        // sorted keys

        let key_precision = stats.keys_precision.map(|keys_prec| {
            let x = keys_prec
                .iter()
                .map(|(k, v)| {
                    let mut style = Style::default().fg(Color::White);
                    if *v > 0.9 {
                        style = style.fg(Color::Green);
                    } else if *v > 0.8 {
                        style = style.fg(Color::Yellow);
                    } else {
                        style = style.fg(Color::Red);
                    }
                    // format v with 3 decimal places
                    let e = format!("{:.3}", *v);

                    let p = Span::styled(e, style);
                    let line = Line::from(vec![
                        Span::styled(k.to_string(), Style::default().fg(Color::White)),
                        Span::styled(": ", Style::default().fg(Color::White)),
                        p,
                    ]);
                    Cell::from(line)
                })
                .collect::<Vec<Cell<'_>>>();

            x.chunks(3)
                .map(|chunk| Row::new(chunk.to_vec()))
                .collect::<Vec<_>>()
        });

        let kp = key_precision.map(|x| {
            Table::new(x)
                .block(Block::default().title("Table"))
                .style(Style::default().fg(Color::White))
                .block(Block::default().title("Table"))
                // Columns widths are constrained in the same way as Layout...
                .widths(&[
                    Constraint::Length(10),
                    Constraint::Length(10),
                    Constraint::Length(10),
                ])
        });

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

        let lines = self.get_keyboard_spans(state);

        frame.render_widget(
            Paragraph::new("")
                .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: true }),
            first_column_layout[1],
        );

        frame.render_widget(
            Paragraph::new(lines)
                // .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: true }),
            centered_rect(first_column_layout[1], 50, 80), // frame.size(),
        );

        frame.render_widget(
            Paragraph::new(stats).block(Block::default().borders(Borders::ALL)),
            second_column[0], // frame.size(),
        );

        if let Some(kp) = kp {
            frame.render_widget(
                kp.style(Style::default().fg(Color::White))
                    .block(Block::default().borders(Borders::ALL)),
                second_column[1], // frame.size(),
            );
        }
    }
}
