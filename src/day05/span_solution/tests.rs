use super::{
    horizontal_spans, intersect_orthogonal_spans, intersect_parallel_spans, vertical_spans,
};
use crate::day05::{parse, point::Point};

#[test]
fn part1() {
    let lines = parse(crate::day05::tests::INPUT).unwrap();

    let horizontal_spans = horizontal_spans(&lines);
    assert_eq!(4, horizontal_spans.len());

    let vertical_spans = vertical_spans(&lines);
    assert_eq!(2, vertical_spans.len());

    let (intersections, vertical_non_intersecting_spans) = intersect_parallel_spans(vertical_spans);
    assert_eq!(0, intersections.len());
    assert_eq!(2, vertical_non_intersecting_spans.len());

    let (intersections, horizontal_non_intersecting_spans) =
        intersect_parallel_spans(horizontal_spans);
    assert_eq!(2, intersections.len());
    assert_eq!(3, horizontal_non_intersecting_spans.len());

    let intersections = intersect_orthogonal_spans(
        &horizontal_non_intersecting_spans,
        &vertical_non_intersecting_spans,
    );
    assert_eq!(1, intersections.len());
    assert_eq!(Point::new(7, 4), intersections[0]);
}
