pub struct ButtonGroup<'a> {
    labels: &'a [&'a str],
    index: usize
}

impl<'a> ButtonGroup<'a> {
    pub fn new(labels: &'a [&str]) -> Self {
        Self { labels, index: 0 }
    }

    pub fn current(&self) -> &'a str {
        self.labels[self.index]
    }

    pub fn down(&mut self) {
        self.index = (self.index + 1) % self.labels.len();
    }

    pub fn up(&mut self) {
        if self.index == 0 {
            self.index = self.labels.len() - 1;
        } else {
            self.index -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_does_not_crash() {
        ButtonGroup::new(&["Play", "Options", "Quit"]);
    }

    #[test]
    fn test_current_shows_first_by_default() {
        let group = ButtonGroup::new(&["Play", "Options", "Quit"]);
        assert_eq!(group.current(), "Play");
    }

    #[test]
    fn test_down_updates_index() {
        let mut group = ButtonGroup::new(&["Play", "Options", "Quit"]);
        assert_eq!(group.current(), "Play");

        group.down();
        assert_eq!(group.current(), "Options");

        group.down();
        assert_eq!(group.current(), "Quit");

        group.down();
        assert_eq!(group.current(), "Play");
    }

    #[test]
    fn test_up_updates_index() {
        let mut group = ButtonGroup::new(&["Play", "Options", "Quit"]);
        assert_eq!(group.current(), "Play");

        group.up();
        assert_eq!(group.current(), "Quit");

        group.up();
        assert_eq!(group.current(), "Options");

        group.up();
        assert_eq!(group.current(), "Play");
    }
}
