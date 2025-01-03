use terminal_size::{terminal_size, Width};
use unicode_width::UnicodeWidthStr;

pub const TITLE_ART: &str = r#"
 ██████╗ █████╗ ██████╗ ██████╗  █████╗ ███╗   ██╗ ██████╗ ███╗   ███╗██╗   ██╗██╗      █████╗ 
██╔════╝██╔══██╗██╔══██╗██╔══██╗██╔══██╗████╗  ██║██╔═══██╗████╗ ████║██║   ██║██║     ██╔══██╗
██║     ███████║██████╔╝██║  ██║███████║██╔██╗ ██║██║   ██║██╔████╔██║██║   ██║██║     ███████║
██║     ██╔══██║██╔══██╗██║  ██║██╔══██║██║╚██╗██║██║   ██║██║╚██╔╝██║██║   ██║██║     ██╔══██║
╚██████╗██║  ██║██║  ██║██████╔╝██║  ██║██║ ╚████║╚██████╔╝██║ ╚═╝ ██║╚██████╔╝███████╗██║  ██║
 ╚═════╝╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝ ╚═╝     ╚═╝ ╚═════╝ ╚══════╝╚═╝  ╚═╝
"#;

pub const DONKEY_ART: &str = r#"
                                __...----..
                             .-'           `-.
                            /        .---.._  \
                            |        |   \  \ |
                             `.      |    | | |        _____
                               `     '    | | /    _.-`      `.
                                \    |  .'| //'''.'            \
                                 `---'_(`.||.`.`.'    _.`.'''-. \
                                    _(`'.    `.`.`'.-'  \\     \ \
                                   (' .'   `-._.- /      \\     \ |
                                  ('./   `-._   .-|       \\     ||
                                  ('.\ | | 0') ('0 __.--.  \`----'/
                             _.--('..|   `--    .'  .-.  `. `--..'
               _..--..._ _.-'    ('.:|      .  /   ` 0 `   \
            .'         .-'        `..'  |  / .^.           |
           /         .'                 \ '  .             `._
        .'|                              `.  \`...____.----._.'
      .'.'|         .                      \ |    |_||_||__|
     //   \         |                  _.-'| |_ `.   \
     ||   |         |                     /\ \_| _  _ |
     ||   |         /.     .              ' `.`.| || ||
     ||   /        ' '     |        .     |   `.`---'/
   .' `.  |       .' .'`.   \     .'     /      `...'
 .'     \  \    .'.'     `---\    '.-'   |
)/\ / /)/ .|    \             `.   `.\   \
 )/ \(   /  \   |               \   | `.  `-.
  )/     )   |  |             __ \   \.-`    \
         |  /|  )  .-.      //' `-|   \  _   /
        / _| |  `-'.-.\     ||    `.   )_.--'
        )  \ '-.  /  '|     ''.__.-`\  | 
       /  `-\  '._|--'               \  `.
       \    _\                       /    `---.
       /.--`  \                      \    .''''\
       `._..._|                       `-.'  .-. |
                                        '_.'-./.'
                  The Cardano Mule
"#;

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}


fn center_text(text: &str) -> String {
    let terminal_width = terminal_size().map(|(Width(w), _)| w).unwrap_or(80);
    text.lines()
        .map(|line| {
            let visible_length = UnicodeWidthStr::width(line);
            let padding = (terminal_width as usize).saturating_sub(visible_length) / 2;
            format!("{:>width$}{}", "", line, width = padding)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_landing_page() {
    clear_screen();
    println!("{}", center_text(TITLE_ART));
    println!("\n{}", center_text(DONKEY_ART));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
} 