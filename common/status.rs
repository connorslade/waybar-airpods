#[derive(Default)]
pub struct Status {
    pub left: Option<u8>,
    pub right: Option<u8>,
    pub case: Option<u8>,
}

impl Status {
    pub fn is_valid(&self) -> bool {
        self.left.is_some() || self.right.is_some() || self.case.is_some()
    }

    pub fn min_pods(&self) -> u8 {
        let mut out = u8::MAX;

        if let Some(left) = self.left {
            out = out.min(left);
        }

        if let Some(right) = self.right {
            out = out.min(right);
        }

        if let Some(case) = self.case
            && case > 0
        {
            out = out.min(case);
        }

        out
    }
}
