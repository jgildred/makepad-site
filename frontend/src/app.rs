use makepad_widgets::*;
   
live_design!(
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::ui::*;
    
    App = {{App}} {
        ui: <Ui> {}
    }
);

#[derive(Live, LiveHook)]
pub struct App {
    #[live] ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::ui::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, _cx:&mut Cx, actions:&Actions){
        let blogs = self.ui.portal_list_set(ids!(blog.list));
        for (item_id, item) in blogs.items_with_actions(&actions) {
            if item.button(id!(likes)).clicked(&actions) {
                log!("liked {}", item_id);
            }  
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);