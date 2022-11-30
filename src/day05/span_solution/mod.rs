use super::{line::Line, point::Point, span::Span};
use anyhow::anyhow;

#[cfg(test)]
mod tests;

fn horizontal_spans(lines: &[Line]) -> Vec<Span> {
    let mut horizontal_spans = lines
        .iter()
        .filter_map(|line| {
            if line.horizontal() {
                Some(Span::new(line.a().y(), line.a().x(), line.b().x()))
            } else {
                None
            }
        })
        .collect::<Vec<Span>>();
    horizontal_spans.sort_unstable();
    horizontal_spans
}

fn vertical_spans(lines: &[Line]) -> Vec<Span> {
    let mut vertical_spans = lines
        .iter()
        .filter_map(|line| {
            if line.vertical() {
                Some(Span::new(line.a().x(), line.a().y(), line.b().y()))
            } else {
                None
            }
        })
        .collect::<Vec<Span>>();
    vertical_spans.sort_unstable();
    vertical_spans
}

/// Intersect the spans with each other.
/// Return the intersections as a set of spans, and the non-intersecting spans as another set.
fn intersect_parallel_spans(mut spans: Vec<Span>) -> (Vec<Span>, Vec<Span>) {
    let mut intersections = Vec::with_capacity(spans.len());
    let mut non_intersections = Vec::with_capacity(spans.len());
    (0..spans.len()).for_each(|index| {
        // Check for intersections of this span with all spans at index+1 and later that have the same ordinal.
        let mut span = spans[index];
        let ordinal = span.ordinal();
        let mut matched_ordinal = false;
        let _expect_err_on_early_out = spans
            .iter_mut()
            .skip(index + 1)
            .take_while(|other_span| other_span.ordinal() == ordinal)
            .try_for_each(|other_span| {
                matched_ordinal = true;
                // If they intersect...
                if other_span.start() < span.end() {
                    if span.start() < other_span.start() {
                        non_intersections.push(Span::new(
                            ordinal,
                            span.start(),
                            other_span.start(),
                        ));
                    }
                    if span.end() < other_span.end() {
                        if other_span.start() <= span.last() {
                            intersections.push(Span::new(ordinal, other_span.start(), span.last()));
                        }
                        other_span.set_start(span.end()).unwrap();
                        return Err(anyhow!("Consumed end of initial span"));
                    } else {
                        intersections.push(*other_span);
                        other_span.set_start(other_span.end()).unwrap();
                        span.set_start(other_span.end()).unwrap();
                        // Consumed the next span, stay on the initial one.
                    }
                }
                // If not intersecting, then no other spans will intersect with this one because they are ordered.
                // Move to next span.
                else {
                    non_intersections.push(span);
                    return Err(anyhow!("No intersections with initial span"));
                }
                Ok(())
            });
        if !matched_ordinal {
            non_intersections.push(span);
        }
    });
    (intersections, non_intersections)
}

fn intersect_orthogonal_spans(horizontal_spans: &[Span], vertical_spans: &[Span]) -> Vec<Point> {
    if vertical_spans.is_empty() {
        return vec![];
    }
    // TODO this algorithm is wrong
    // the horizontal spans are ordered by row, and then by start, but any given vertical span may cross multiple rows
    // many of which may have spans
    // my silly algorithm assumes horizontal span only appear to the right of horizontal spans above
    let mut intersections = Vec::with_capacity(horizontal_spans.len() + vertical_spans.len());
    let mut vertical_span_index = 0;
    let _early_out_when_no_more_vertical_spans =
        horizontal_spans.iter().try_for_each(|horizontal_span| {
            let y = horizontal_span.ordinal();
            (horizontal_span.start()..horizontal_span.end()).try_for_each(|x| {
                while vertical_span_index != vertical_spans.len()
                    && vertical_spans[vertical_span_index].ordinal() < x
                {
                    vertical_span_index += 1;
                }
                if vertical_span_index == vertical_spans.len() {
                    Err(anyhow!("No more vertical spans"))
                } else {
                    let vertical_span = vertical_spans[vertical_span_index];
                    if vertical_span.ordinal() == x && vertical_span.contains(y) {
                        intersections.push(Point::new(x, y));
                    }
                    Ok(())
                }
            })
        });
    intersections
}

pub(super) fn solve_for(lines: &[Line]) -> (usize, usize) {
    let horizontal_spans = horizontal_spans(lines);
    let (horizontal_intersecting_spans, horizontal_non_intersecting_spans) =
        intersect_parallel_spans(horizontal_spans);
    let horizontal_span_aggregation = horizontal_intersecting_spans
        .iter()
        .fold(0, |current, span| current + span.len());

    let vertical_spans = vertical_spans(lines);
    let (vertical_intersecting_spans, vertical_non_intersecting_spans) =
        intersect_parallel_spans(vertical_spans);
    let vertical_span_aggregation = vertical_intersecting_spans
        .iter()
        .fold(0, |current, span| current + span.len());

    (
        horizontal_span_aggregation
            + vertical_span_aggregation
            + intersect_orthogonal_spans(
                &horizontal_non_intersecting_spans,
                &vertical_non_intersecting_spans,
            )
            .len(),
        0,
    )
}
