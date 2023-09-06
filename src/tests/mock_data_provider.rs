use crate::data_provider::DataProvider;

pub(crate) struct MockDataProvider {
    data_waiting: bool,
}

impl MockDataProvider {
    pub(crate) const fn new() -> Self {
        MockDataProvider {
            data_waiting: false,
        }
    }

    pub(crate) fn set_data_waiting(&mut self, data_waiting: bool) {
        self.data_waiting = data_waiting
    }
}

impl DataProvider<()> for MockDataProvider {
    fn data_waiting(&self) -> bool {
        self.data_waiting
    }

    fn get_data(&self) -> Option<()> {
        if self.data_waiting {
            Some(())
        } else {
            None
        }
    }
}
