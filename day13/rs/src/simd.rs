use core::simd::{prelude::*, LaneCount, SupportedLaneCount};

use crate::parse_machine;

const SIZE: usize = 1024;

#[repr(align(4))]
struct Data {
    ax: [i32; SIZE],
    ay: [i32; SIZE],
    bx: [i32; SIZE],
    by: [i32; SIZE],
    px: [i32; SIZE],
    py: [i32; SIZE],
    len: usize,
}

/// # Panics
#[allow(clippy::similar_names)]
pub fn solve_1<const N: usize>(input: &str) -> i32
where
    LaneCount<N>: SupportedLaneCount,
{
    let mut data = Data {
        ax: [0; SIZE],
        ay: [0; SIZE],
        bx: [0; SIZE],
        by: [0; SIZE],
        px: [0; SIZE],
        py: [0; SIZE],
        len: 0,
    };

    let Data {
        ax,
        ay,
        bx,
        by,
        px,
        py,
        len,
    } = &mut data;

    for line in input.split("\n\n") {
        let machine = parse_machine::<i32>(line);

        (
            (ax[*len], ay[*len]),
            (bx[*len], by[*len]),
            (px[*len], py[*len]),
        ) = machine;

        *len += 1;
    }

    let (pax, max, sax) = ax[..*len].as_simd::<N>();
    let (pay, may, say) = ay[..*len].as_simd::<N>();
    let (pbx, mbx, sbx) = bx[..*len].as_simd::<N>();
    let (pby, mby, sby) = by[..*len].as_simd::<N>();
    let (ppx, mpx, spx) = px[..*len].as_simd::<N>();
    let (ppy, mpy, spy) = py[..*len].as_simd::<N>();

    let n0 = Simd::<i32, N>::splat(0_i32);
    let n3 = Simd::<i32, N>::splat(3_i32);

    let mut s = n0;
    for i in 0..max.len() {
        let det = max[i] * mby[i] - may[i] * mbx[i];
        let num_a = mpx[i] * mby[i] - mpy[i] * mbx[i];
        let num_b = max[i] * mpy[i] - may[i] * mpx[i];

        let ma = (num_a % det).simd_eq(n0);
        let mb = (num_b % det).simd_eq(n0);
        let m = ma & mb;

        s += m.select((num_a / det) * n3 + num_b / det, n0);
    }

    let mut total = s.reduce_sum();

    let calc = |ax, ay, bx, by, px, py| {
        let det = ax * by - ay * bx;

        let a = px * by - py * bx;

        if a % det != 0 {
            return None;
        }

        let b = ax * py - ay * px;
        if b % det != 0 {
            return None;
        }

        Some(a / det * 3 + b / det)
    };

    for i in 0..pax.len() {
        let t = calc(pax[i], pay[i], pbx[i], pby[i], ppx[i], ppy[i]).unwrap_or_default();
        total += t;
    }
    for i in 0..sax.len() {
        let t = calc(sax[i], say[i], sbx[i], sby[i], spx[i], spy[i]).unwrap_or_default();
        total += t;
    }

    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn same_test_1() {
        assert_eq!(solve_1::<64>(&crate::tests::INPUT), 480);
    }

    #[test]
    fn same_results_1() {
        assert_eq!(solve_1::<64>(&crate::INPUT), 37686);
    }
}
