extern crate web_view;
use web_view::*;

extern crate minesweeper_backend;
use minesweeper_backend::engine::minesweeper::*;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::time::Duration;
use std::thread::{spawn, sleep};
use std::sync::Mutex;

/* Going faster than ~100 micro seconds appears to cause crashes, 120fps sound be way more than enough */
const FPS_120: Duration = Duration::from_millis(8); /* 1/120 = 8.333... */

fn main() {
    let title = "Minesweeper";
    let content = Content::Html(create_html());
	let size = Some((800, 600));
	let resizable = true;
	let debug = false;
    let _initial_callback = "Need to define type of this if it's here";
    let _external_callback = "Need to define type of this if it's here";

    let state = Mutex::new(Minesweeper::new(8, 8, 10));
 
	run(title, content, size, resizable, debug, move |webview| {
		spawn(move || {
            webview.dispatch(|webview, state| {
                let game = state.lock().unwrap();

                update_ui(webview, &game);

                webview.eval(&format!("toFrontEnd({})", "'Test'"));
            });
		});
	}, move |webview, arg, state| {
        println!("Received: {}", arg);
        match serde_json::from_str(arg)
        {
            Ok(FromUiCommand::Reset) =>
            {
                // let mut counter = state.lock().unwrap();
                // counter.reset();
            },
            Ok(FromUiCommand::Start{ width, height, num_bombs }) => 
            {

            },
            Ok(FromUiCommand::Exit) => webview.terminate(),
            Err(error) => println!("{}", error),
        }
	}, state);
}

pub fn update_ui<'a, T>(webview: &mut WebView<'a, T>, minesweeper: &Minesweeper)
{
    match serde_json::to_string(&minesweeper.get_tiles())
    {
        Ok(json) => 
        {
            webview.eval(&format!("toFrontEnd({})", json));
        },
        Err(error) => {},
    };
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "_type")]
pub enum FromUiCommand {
    Reset,
    Start {width: u16, height: u16, num_bombs: u16},
    Exit,
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