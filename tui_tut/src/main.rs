use cursive::Cursive;

use cursive::views::Dialog;
use cursive::views::TextView;

fn yes(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(TextView::new("You finished working!"));
}

fn no(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(TextView::new("You didn't finish working!"));
}

fn main() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(
        Dialog::text("Finished Working?")
            .title("Question")
            .button("Yes", yes)
            .button("No", no),
    );

    siv.run();
}
