use console::{style, Term};
use indicatif::{ProgressBar, ProgressStyle};

pub fn print_banner() {
    let _ = Term::stdout().clear_screen();
    println!(
        "{}",
        style("🩺  HealDep – Self‑Healing Package Manager")
            .bold()
            .green()
    );
    println!(
        "{}",
        style("Первый в мире менеджер, самостоятельно лечащий конфликты зависимостей")
            .dim()
            .italic()
    );
    println!();
}

pub fn spinner(msg: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.to_owned());
    pb.enable_steady_tick(std::time::Duration::from_millis(80));
    pb
}

pub fn finish_spinner(pb: &ProgressBar, success: bool, final_msg: &str) {
    pb.finish_and_clear();
    let icon = if success {
        style("✔").green().bold()
    } else {
        style("✖").red().bold()
    };
    println!("{} {}", icon, final_msg);
}

pub fn separator() {
    println!("{}", style("────────────────────────────────────").dim());
}
