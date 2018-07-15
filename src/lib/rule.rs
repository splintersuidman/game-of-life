// (bool, bool) = (survival, birth).
#[derive(Debug, Clone, PartialEq)]
pub struct Rule([(bool, bool); 8]);

impl Rule {
    #[inline]
    pub fn empty() -> Rule {
        Rule([(false, false); 8])
    }

    #[inline]
    pub fn normal() -> Rule {
        Rule::default()
    }

    #[inline]
    pub fn survival(&self, neighbours: usize) -> bool {
        self.0[neighbours].0
    }

    #[inline]
    pub fn birth(&self, neighbours: usize) -> bool {
        self.0[neighbours].1
    }

    #[inline]
    pub fn set_survival(&mut self, neighbours: usize, survival: bool) {
        self.0[neighbours].0 = survival;
    }

    #[inline]
    pub fn set_birth(&mut self, neighbours: usize, birth: bool) {
        self.0[neighbours].1 = birth;
    }

    pub fn display_birth(&self) -> String {
        let mut string = String::new();

        for i in 0..self.0.len() {
            if self.0[i].1 {
                string.push((b'0' + i as u8) as char);
            }
        }

        string
    }

    pub fn display_survival(&self) -> String {
        let mut string = String::new();

        for i in 0..self.0.len() {
            if self.0[i].0 {
                string.push((b'0' + i as u8) as char);
            }
        }

        string
    }
}

impl Default for Rule {
    #[inline]
    fn default() -> Rule {
        Rule([
            (false, false),
            (false, false),
            (true, false),
            (true, true),
            (false, false),
            (false, false),
            (false, false),
            (false, false),
        ])
    }
}
