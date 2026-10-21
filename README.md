# ciuc
A simple tool for creating images using characters.
## Architecture
Written in the Rust language.Built using [ratatui](https://github.com/ratatui/ratatui)
## Use
- You can clone the entire project and then run: "cargo build --release"
- Then you can execute the command "./target/release/ciuc" to run the program! (On Windows, use ".\target\release\ciuc.exe")
- After you run the program, it displays:
"Please specify the image path, or provide both the image path and the path of the JSON file for the animation.You can get help by entering the "--help" command."
This is normal, as you haven't specified the paths for the image and the JSON file (the JSON file is only required if you need to render sprites exported from [Aseprite](https://www.aseprite.org/)).
- Once the target image has been successfully rendered, you can press the 'f' key to fill the entire area, or the 'm' key to maintain the original image's aspect ratio; press the 'ESC' key to exit the program.
- You can get help by adding the "--help" command when running the program.
- For more details about Cargo and Rust, you can check: https://doc.rust-lang.org/book/
## Important Notes
- The version of [Aseprite](https://www.aseprite.org/) corresponding to the current project is v1.3.18.1.A version mismatch may prevent the JSON file from being read correctly.
- Currently, the project renders animations in a back-and-forth looping pattern; no other effects are supported at this time.
- If a program exits unexpectedly, it may cause the terminal to behave abnormally, requiring you to manually restore the terminal's original settings.
