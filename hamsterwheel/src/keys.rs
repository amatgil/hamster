pub struct KeyDistribution<const MAXLEN: usize> {
    keys: [[Option<char>; MAXLEN]; 3],
}

impl<const MAXLEN: usize> KeyDistribution<MAXLEN> {
    pub const fn new<const A: usize, const B: usize, const C: usize>(
        first: [char; A],
        second: [char; B],
        third: [char; C],
    ) -> Self {
        assert!(A <= MAXLEN);
        assert!(B <= MAXLEN);
        assert!(C <= MAXLEN);
        let mut a = [None; MAXLEN];
        let mut b = [None; MAXLEN];
        let mut c = [None; MAXLEN];

        let mut ki = 0;
        while ki < A {
            a[ki] = Some(first[ki]);
            ki += 1;
        }

        let mut ki = 0;
        while ki < B {
            b[ki] = Some(second[ki]);
            ki += 1;
        }

        let mut ki = 0;
        while ki < C {
            c[ki] = Some(third[ki]);
            ki += 1;
        }

        Self { keys: [a, b, c] }
    }
    pub fn max_width(&self) -> usize {
        MAXLEN
    }
    pub const fn get(&self, row: i32, col: i32) -> Option<char> {
        assert!(row >= 0 && col >= 0);
        let (row, col) = (row as usize, col as usize);
        if row >= 3 || col >= MAXLEN {
            None
        } else {
            self.keys[row][col]
        }
    }
}
