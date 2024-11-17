pub(crate) fn show(len: u64, message: &str) {
    let bar = indicatif::ProgressBar::new(len);
    bar.set_style(
        indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("#-"),
    );
    // bar.inc(1);
    bar.finish_with_message(message.to_string());
}
