use std::ops::Range;

/// 두 범위가 겹치면 true를 반환한다.
///
///     assert_eq!(fern_sim::ranges::overlap(0..7, 3..10), true);
///     assert_eq!(fern_sim::ranges::overlap(1..5, 101..105), false);
///
/// 두 범위 중 하나라도 비어 있으면 겹치지 않는다고 판단한다.
///
///     assert_eq!(fern_sim::ranges::overlap(0..0, 0..10), false);
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}
