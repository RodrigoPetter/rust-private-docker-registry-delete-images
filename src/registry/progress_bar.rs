use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

const STYLE: &str = "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}";

pub struct ScanProgressBar {
    pub total_bar: ProgressBarWrapper,
    multi_progress_bar: MultiProgress,
}

impl ScanProgressBar {
    pub fn new(total: u64) -> ScanProgressBar {
        let multi_progress_bar = MultiProgress::new();
        let total_bar = ProgressBarWrapper::new(None);
        let total_bar =
            ProgressBarWrapper::new(Some(multi_progress_bar.add(total_bar.progress_bar)));
        total_bar.set_length(total);

        return ScanProgressBar {
            multi_progress_bar,
            total_bar,
        };
    }

    pub fn add_bar(&self, index: usize) -> ProgressBarWrapper {
        let bar = ProgressBarWrapper::new(None);
        let pb = self
            .multi_progress_bar
            .insert_from_back(index+1, bar.progress_bar);
        return ProgressBarWrapper::new(Some(pb));
    }

    pub fn remove_bar(&self, bar: ProgressBarWrapper) {
        self.multi_progress_bar.remove(&bar.progress_bar)
    }
}

pub struct ProgressBarWrapper {
    progress_bar: ProgressBar,
}

impl ProgressBarWrapper {
    pub fn new(pb: Option<ProgressBar>) -> ProgressBarWrapper {
        return match pb {
            Some(progress_bar) => ProgressBarWrapper { progress_bar },
            None => {
                let progress_bar = ProgressBar::new(1);
                progress_bar.set_style(ProgressStyle::with_template(STYLE).unwrap());
                return ProgressBarWrapper { progress_bar };
            }
        };
    }

    pub fn inc(&self, delta: u64) {
        &self.progress_bar.inc(delta);
    }

    pub fn set_length(&self, len: u64) {
        &self.progress_bar.set_length(len);
    }

    pub fn set_message<S: Into<String>>(&self, msg: S) {
        &self.progress_bar.set_message(msg.into());
        #[cfg(debug_assertions)]
        {
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    }
}
