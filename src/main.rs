use druid::{Env, Window, WindowDesc, WindowHandle, WindowId, Widget, Data, Lens, WidgetExt, AppLauncher};
use druid::widget::{Label, Flex, Align, Button};
use druid::widget::Tabs;
use std::sync::RwLock;

mod packet_control;

pub use packet_control::PacketInterface;

const VERSION_NO : &str = "0.0.1";

const WINDOW_TITLE : &str = "RS Combat: Cross Platform Parser for FFXIV!";
const NUM_TAB_ENTRIES : usize = 4;
const TAB_ENTRIES : [&str; NUM_TAB_ENTRIES] = ["Main", "Parser", "Plugins", "About"];
const FFXIV_PROC_NAME : &str = "ffxiv";
const FFXIV_NAME : &str = "ffxiv_dx11.exe";


#[derive(Debug)]
enum AlertLevel {
    INFO, 
    DEBUG,
    WARNING,
    ERROR,
    
}


trait ProgramInterface {
    fn get_packet_interface(&self) -> PacketInterface {
        PacketInterface::new()
    }
}

#[derive(Clone, Data)]
struct InitLayout {
    title: String,
}

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref packet_if: PacketInterface = PacketInterface::new();
}

fn main() {
   let primary_window = WindowDesc::new(startup)
       .title(WINDOW_TITLE)
       .window_size((1600.0, 1000.0));

   let starting_state = InitLayout {
        title: "RS Combat".into(),
   };

   AppLauncher::with_window(primary_window)
       .launch(starting_state)
       .expect("Failed to start RS Combat!");
   
}

fn startup() -> impl Widget<InitLayout>{
    let nav_tabs = gen_tabs();
    let main_frame_layout = Flex::column()
        .with_child(nav_tabs);
    Align::centered(main_frame_layout)
}

fn gen_tabs() -> impl Widget<InitLayout>
{
    let mut tab_system = Tabs::new();
    for i in 0..NUM_TAB_ENTRIES {
        tab_system.add_tab(TAB_ENTRIES[i], gen_tab_layout(TAB_ENTRIES[i]));
    }
    tab_system
}

fn gen_tab_layout(tabname : &str) -> impl Widget<InitLayout> {
    let mut layout = Flex::row();
   match tabname {
       "Main" => { layout.add_child(gen_main_window()); },
       "Plugins" => { layout.add_child(gen_plugin_window()); },
       "About" => {
            let label = Label::new(|_d: &InitLayout, _e: &Env| format!("Version: {}\n Authored by Kenneth Hunter.", VERSION_NO));
            layout.add_child(label);
       },
        _ => { 
            let label = Label::new("Not Implemented!");
            layout.add_child(label);
        },
   }
   layout
}


fn gen_main_window() -> impl Widget<InitLayout> {
    let mut layout = Flex::column();
    let conn_btn = Button::new("Connect to game").on_click(|_ctx, _data, _env| -> () {
        packet_if.detect_process(FFXIV_PROC_NAME, FFXIV_NAME);
        println!("Current pid: {}", packet_if.get_process_id());
    });
    layout.add_child(conn_btn);
    layout
}

fn gen_plugin_window() -> impl Widget<InitLayout>{
    let mut layout = Flex::column();
    // begin layout for first column
    //
    // begin layout for plugin enable/disable buttons
    let en_btn = Button::new("->").on_click(|_ctx, _data, _env| match enable_plugin("FIXME") {
            true => { alert(AlertLevel::INFO, "FIXME was enabled."); },
            false => { alert(AlertLevel::ERROR, "Failed to enable FIXME"); },
    });
    let dis_btn = Button::new("<-").on_click(|_cts, _data, _env| match disable_plugin("FIXME") {
            true => { alert(AlertLevel::INFO, "FIXME was disable."); },
            false => { alert(AlertLevel::ERROR, "Failed to disable FIXME"); },
    });
    let btn_container = Flex::column()
        .with_child(en_btn)
        .with_child(dis_btn);
    layout.add_child(btn_container);
    layout
}

fn enable_plugin(plugin_url: &str) -> bool {
    return false
}

fn disable_plugin(plugin_url: &str) -> bool {
    return false
}

fn alert(level: AlertLevel, message: &str)
{
    println!("{:?}: {}", level, message);
}
