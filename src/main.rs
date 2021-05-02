use druid::{Env, Window, WindowDesc, WindowHandle, WindowId, Widget, Data, Lens, WidgetExt, AppLauncher};
use druid::widget::{Label, Flex, Align, Button};
use druid::widget::Tabs;

const window_title : &str = "RS Combat: Cross Platform Parser for FFXIV!";
const NUM_TAB_ENTRIES : usize = 4;
const TAB_ENTRIES : [&str; NUM_TAB_ENTRIES] = ["Main", "Parser", "Plugins", "About"];

#[derive(Clone, Data, Lens)]
struct InitLayout {
    title: String,
}

fn main() {
   let primary_window = WindowDesc::new(startup)
       .title(window_title)
       .window_size((1600.0, 1000.0));

   let starting_state = InitLayout {
        title: "RS Combat".into()
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
        tab_system.add_tab(TAB_ENTRIES[i], Label::new(TAB_ENTRIES[i]));
    }
    tab_system
}
