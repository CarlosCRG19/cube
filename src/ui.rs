use crate::app::App;
use crate::math;

use std::time::Duration;
use ratatui::{
    prelude::*, widgets::{Block, List, ListItem, Paragraph}, Frame
};
use unicode_segmentation::UnicodeSegmentation;

pub fn draw(frame: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Percentage(50),
        ])
        .split(frame.size());

    let inner_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Min(0),
            Constraint::Length(36),
            Constraint::Length(36),
        ])
        .split(main_layout[1]);

    let scramble = match &app.current_scramble {
        Some(s) => s.clone(),
        None => "".to_string()
    };
    let scramble = Paragraph::new(scramble)
        .block(Block::bordered().title(" Scramble "));
    frame.render_widget(scramble, main_layout[0]);

    // timer
    let timer = Paragraph::new(format_time(app.timer.elapsed()))
        .block(Block::bordered());
    frame.render_widget(timer, inner_layout[0]);

    // times
    let times: Vec<_> = app.session.solves()
        .iter()
        .filter_map(|solve| {
            if let Some(time) = solve.time() {
                Some(*time)
            } else {
                None
            }
        })
        .collect();

    let formatted_times: Vec<_> = times.iter().map(|time| format_time(*time)).collect();
    let time_list: Vec<_> = segment_times_in_lines(&formatted_times, 35)
        .iter()
        .map(|time| ListItem::new(time.clone()))
        .collect();

    let list = List::new(time_list)
        .block(Block::bordered().title(" Times "));
    frame.render_widget(list, inner_layout[1]);
        
    // Stats
    let stats = format!(
        "avg: {} (σ = {})",
        if let Some(avg) = math::avg(&times) { format_time(avg) } else { "DNF".to_string() },
        if let Some(std) = math::std(&times) { format_time(std) } else { "-1".to_string() },
    );
    let stats = Paragraph::new(stats)
            .block(Block::bordered().title(" Stats "));
    frame.render_widget(stats, inner_layout[2]);
}

fn segment_times_in_lines(times: &[String], max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for (i, time) in times.iter().enumerate() {
        let separator = if i == times.len() - 1 { "" } else { ", " };
        let potential_addition = format!("{}{}", time, separator);
        
        if current_line.graphemes(true).count() + potential_addition.graphemes(true).count() <= max_width {
            current_line.push_str(&potential_addition);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line.trim_end().to_string());
            }
            current_line = potential_addition.to_string();
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line.trim_end().to_string());
    }

    lines
}

fn format_time(time: Duration) -> String {
    let total_seconds = time.as_secs();
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let millis = time.subsec_millis();

    match (hours, minutes, seconds) {
        (0, 0, s) => format!("{}.{:02}", s, millis / 10),
        (0, m, s) => format!("{}:{:02}.{:02}", m, s, millis / 10),
        (h, m, s) => format!("{}:{:02}:{:02}.{:02}", h, m, s, millis / 10),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_times_below_one_second() {
        assert_eq!(format_time(Duration::from_millis(0)), String::from("0.00"));
        assert_eq!(format_time(Duration::from_millis(500)), String::from("0.50"));
        assert_eq!(format_time(Duration::from_millis(999)), String::from("0.99"));
    }

    #[test]
    fn format_times_below_one_minute() {
        assert_eq!(format_time(Duration::from_millis(7480)), String::from("7.48"));
        assert_eq!(format_time(Duration::from_millis(45000)), String::from("45.00"));
        assert_ne!(format_time(Duration::from_millis(60000)), String::from("60.00"));
    }

    #[test]
    fn format_times_below_one_hour() {
        assert_eq!(format_time(Duration::from_secs(60)), String::from("1:00.00"));
        assert_eq!(format_time(Duration::from_secs(600)), String::from("10:00.00"));
        assert_ne!(format_time(Duration::from_secs(3600)), String::from("60:00.00"));
    }

    #[test]
    fn format_times_above_one_hour() {
        assert_eq!(format_time(Duration::from_secs(3600)), String::from("1:00:00.00"));
        assert_eq!(format_time(Duration::from_secs(86400)), String::from("24:00:00.00"));
    }

    #[test]
    fn times_segmentation() {
        let times = vec![
            "1.54".to_string(),
            "1:43.50".to_string(),
            "1:45:54.99".to_string(),
        ];

        assert_eq!(segment_times_in_lines(&times, 25),vec!["1.54, 1:43.50, 1:45:54.99"]);
        assert_eq!(segment_times_in_lines(&times, 15), vec!["1.54, 1:43.50,", "1:45:54.99"]);
    }
}
