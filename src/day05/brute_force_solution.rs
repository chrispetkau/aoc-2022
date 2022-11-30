use super::{line::Line, span::Span};

const SIZE: usize = 1000;

fn mark_orthogonal_line(line: &Line, cells: &mut [usize]) -> bool {
    if line.horizontal() {
        let span = Span::new(line.a().y(), line.a().x(), line.b().x());
        let row_start = span.ordinal() * SIZE;
        (span.start()..span.end()).for_each(|x| cells[row_start + x] += 1);
        true
    } else if line.vertical() {
        let span = Span::new(line.a().x(), line.a().y(), line.b().y());
        (span.start()..span.end()).for_each(|y| cells[y * SIZE + span.ordinal()] += 1);
        true
    } else {
        false
    }
}

fn part1(lines: &[Line]) -> usize {
    let mut cells = vec![0; SIZE * SIZE];
    lines.iter().for_each(|line| {
        mark_orthogonal_line(line, &mut cells);
    });
    cells.iter().filter(|&&cell| 1 < cell).count()
}

fn part2(lines: &[Line]) -> usize {
    let mut cells = vec![0; SIZE * SIZE];
    lines.iter().for_each(|line| {
        if !mark_orthogonal_line(line, &mut cells) {
            let x_span = Span::new(Default::default(), line.a().x(), line.b().x());
            // Order points so that lower x is first
            let (a, b) = if line.a().x() < line.b().x() {
                (line.a(), line.b())
            } else {
                (line.b(), line.a())
            };
            let slopes_up = a.y() < b.y();
            if slopes_up {
                (x_span.start()..x_span.end())
                    .zip(a.y()..=b.y())
                    .for_each(|(x, y)| cells[y * SIZE + x] += 1);
            } else {
                (x_span.start()..x_span.end())
                    .zip((b.y()..=a.y()).rev())
                    .for_each(|(x, y)| cells[y * SIZE + x] += 1);
            }
        }
    });
    cells.iter().filter(|&&cell| 1 < cell).count()
}

pub(super) fn solve_for(lines: &[Line]) -> (usize, usize) {
    (part1(lines), part2(lines))
}
