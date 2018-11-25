extern crate web_view;
use web_view::*;

extern crate minesweeper_backend;
use minesweeper_backend::engine::minesweeper::{Action, Minesweeper, Tile, State};
use minesweeper_backend::common::{Horizontal, Vertical};

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

#[macro_use]
extern crate log;
extern crate env_logger;
use env_logger::{Builder, Target};
 
use std::thread::spawn;
use std::sync::Mutex;
use std::env;

fn main() {
    configure_logger();

    let title = "Minesweeper";
    let content = Content::Html(create_html());
	let size = Some((800, 600));
	let is_resizable = true;
	let in_debug = false;

    let state = Mutex::new(Minesweeper::new(Horizontal(8), Vertical(8), 10).unwrap());
 
	run(title, content, size, is_resizable, in_debug, move |webview| {
		spawn(move || {
            webview.dispatch(|webview, state| {
                let game = state.lock().unwrap();

                send_to_ui(webview, &ToUiCommand::NewField {tiles: game.get_tiles()});
                send_to_ui(webview, &ToUiCommand::InProgress);

                /*
                    The examples typically have the initial callback having a loop but a loop isn't needed for Minesweeper.

                    Running the loop faster than once per ~100 microseconds appears to cause crashes though.
                */
            });
		});
	}, move |webview, arg, state| {
        trace!("Received from UI: {}", arg);

        let mut game = state.lock().unwrap();

        match serde_json::from_str(arg)
        {
            Ok(Action::Start{ width, height, num_bombs }) => 
            {
                match game.resize(Horizontal(width), Vertical(height), num_bombs)
                {
                    Ok(_) => {},
                    Err(error) => error!("failed to resize because {}", error),
                }
                send_to_ui(webview, &ToUiCommand::NewField {tiles: game.get_tiles()});
                send_to_ui(webview, &ToUiCommand::InProgress);
            },
            Ok(Action::Quit) => webview.terminate(),
            Ok(action) =>
            {
                match game.handle_action(action)
                {
                    Ok(_) => {},
                    Err(error) => error!("Action failed because: {}", error),
                }
                send_to_ui(webview, &ToUiCommand::NewField {tiles: game.get_tiles()});
                match game.get_state()
                {
                    State::Won => send_to_ui(webview, &ToUiCommand::Won),
                    State::Loss => send_to_ui(webview, &ToUiCommand::Loss),
                    _ => send_to_ui(webview, &ToUiCommand::InProgress),
                };
            }
            Err(error) => error!("Unable to parse [{}] because {}", arg, error),
        };
	}, state);
}

#[derive(Serialize, Debug)]
#[serde(tag = "_type")]
pub enum ToUiCommand<'a> {
    Won,
    Loss,
    InProgress,
    NewField { tiles: &'a Vec<Vec<Tile>> },
}

pub fn send_to_ui<'a, S, T>(webview: &mut WebView<'a, T>, data: &S)
    where S: serde::ser::Serialize
{
    trace!("Serializing to send to UI");
    match serde_json::to_string(data)
    {
        Ok(json) => 
        {
            webview.eval(&format!("toFrontEnd({})", json));
            trace!("Sent to UI")
        },
        Err(error) => error!("failed to send to ui because {}", error),
    };
}

fn configure_logger()
{
    let mut builder = Builder::new();
    builder.target(Target::Stdout);
    if let Ok(level) = env::var("RUST_LOG")
    {
        builder.parse(&level);
    }
    builder.init();
}

// TODO?: Handle this via build.rs 
fn create_html() -> String
{
    format!(r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width">
    </head>
    <body>
        <div id="view"></div>

        <script>
            {elmJs}
        
            {portsJs}
        </script> 
        
    </body>
    </html>
    "#,
        elmJs = ELM_JS,
        portsJs = PORTS_JS,
    )
}

const ELM_JS: &'static str = include_str!(concat!("../../", "minesweeper-ui/elm.js"));
const PORTS_JS: &'static str = r#"
        var app = Elm.Main.init({node: document.getElementById("view")});

        app.ports.toBackEnd.subscribe(function (str) {
            window.external.invoke(str);
        });

        function toFrontEnd(str) {
          app.ports.toFrontEnd.send(str);
        }
"#;