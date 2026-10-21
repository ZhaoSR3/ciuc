use std::env;

use ciuc::{
    event::handle_event,
    image::load_rgba_image,
    json::load_json,
    models::{Model, RunningState, SpriteFrame, SpriteFrames},
    tui,
    ui::view,
    update::update,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    let args_len = args.len();
    const HELP: &str = "Please enter the parameters as follows:\n\n--Parameter 1: Image path\n\n--Parameter 2: Json file path\n\nNote: If you need to render an image, you do not need to specify the second parameter; if you need to render an animation, you must specify both the first and second parameters.\n\nTo exit the program, please press the \"ESC\" key.\n\nPress 'm' or 'f' to choose between maintaining the original aspect ratio ('m') or filling the entire area ('f'); please note that filling the entire area may cause the image to become distorted.";

    let mut model = Model::new();

    let image;

    if args_len == 1 {
        eprintln!(
            "Please specify the image path, or provide both the image path and the path of the JSON file for the animation.\n\nYou can get help by entering the \"--help\" command."
        );
        return Ok(());
    } else if args_len == 2 {
        if args[1] == "--help" {
            eprintln!("{HELP}");
            return Ok(());
        }

        image = load_rgba_image(&args[1])?;
        model.sprite_frame = Some(SpriteFrame::new(&image));
    } else if args_len == 3 {
        image = load_rgba_image(&args[1])?;
        let json = load_json(&args[2])?;
        model.sprite_frames = Some(SpriteFrames::new(&json));
    } else {
        eprintln!("{HELP}");
        return Ok(());
    }

    tui::install_panic_hook();
    let mut terminal = tui::init_terminal()?;

    let result = (|| -> Result<(), Box<dyn std::error::Error>> {
        while model.running_state != RunningState::Done {
            terminal.draw(|f| view(&mut model, f, &image))?;

            let mut current_msg = handle_event()?;
            while let Some(msg) = current_msg {
                current_msg = update(&mut model, msg);
            }
        }

        Ok(())
    })();

    // 无论主循环成功还是返回 Err，都尝试恢复终端
    let restore_result = tui::restore_terminal();

    result?;
    restore_result?;

    Ok(())
}
