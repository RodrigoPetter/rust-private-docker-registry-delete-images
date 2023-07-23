use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

pub struct ScanProgressBar {
    multi_progress_bar: MultiProgress,
    pub total_bar: ProgressBar,
    bars: Vec<ProgressBar>,
    default_style: ProgressStyle
}

impl ScanProgressBar {    
    pub fn new(total: u64) -> ScanProgressBar {
        let default_style= ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap();
        let multi_progress_bar = MultiProgress::new();
        let total_bar = multi_progress_bar.add(ProgressBar::new(total));
        total_bar.set_style(default_style.clone());
        
        return ScanProgressBar{
            multi_progress_bar,
            total_bar,
            bars: vec![],
            default_style
        };
    }
}
