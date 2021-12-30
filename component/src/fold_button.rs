#![allow(unused)]
use makepad_render::*;
use crate::button_logic::*;

live_register!{
    use makepad_render::shader::std::*;
    
    FoldButton: {{FoldButton}} {
        bg_quad: {
            instance opened: 0.0
            instance hover: 0.0
            
            fn pixel(self) -> vec4 {
                let sz = 3.;
                let c = vec2(5.0, 0.5 * self.rect_size.y);
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.clear(#2);
                // we have 3 points, and need to rotate around its center
                sdf.rotate(self.opened * 0.5 * PI + 0.5 * PI, c.x, c.y);
                sdf.move_to(c.x - sz, c.y + sz);
                sdf.line_to(c.x, c.y - sz);
                sdf.line_to(c.x + sz, c.y + sz);
                sdf.close_path();
                sdf.fill(mix(#a, #f, self.hover));
                return sdf.result;
            }
        }
        
        walk: Walk {
            width: Width::Fixed(15),
            height: Height::Fixed(15),
            margin: Margin {l: 1.0, r: 1.0, t: 1.0, b: 1.0},
        }
        
        default_state: {
            from: {all: Play::Forward {duration: 0.1}}
            apply: {
                bg_quad: {hover: 0.0}
            }
        }
        
        hover_state: {
            from: {all: Play::Forward {duration: 0.1}}
            apply: {
                bg_quad: {hover: [{time: 0.0, value: 1.0}],}
            }
        }
        
        closed_state: {
            track: open,
            from: {all: Play::Forward {duration: 0.2}}
            apply: {
                opened: [{value: 0.0, ease: Ease::OutExp}],
                bg_quad: {opened: (opened)}
            }
        }
        
        opened_state: {
            track: open,
            from: {all: Play::Forward {duration: 0.2}}
            apply: {
                opened: [{value: 1.0, ease: Ease::OutExp}],
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct FoldButton {
    #[rust] pub button_logic: ButtonLogic,
    #[default_state(default_state)] pub animator: Animator,
    
    default_state: Option<LivePtr>,
    hover_state: Option<LivePtr>,
    closed_state: Option<LivePtr>,
    opened_state: Option<LivePtr>,
    
    opened: f32,
    
    bg_quad: DrawQuad,
    walk: Walk,
}

pub enum FoldButtonAction {
    None,
    Opening,
    Closing
}

impl FoldButton {
    
    pub fn handle_event(
        &mut self,
        cx: &mut Cx,
        event: &mut Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, FoldButtonAction),
    ) {
        self.animator_handle_event(cx, event);
        let res = self.button_logic.handle_event(cx, event, self.bg_quad.draw_vars.area);
        
        match res.state {
            ButtonState::Pressed => {
                if self.opened > 0.2 {
                    self.animate_to(cx, self.closed_state);
                    dispatch_action(cx, FoldButtonAction::Closing)
                }
                else {
                    self.animate_to(cx, self.opened_state);
                    dispatch_action(cx, FoldButtonAction::Opening)
                }
            }
            ButtonState::Default => self.animate_to(cx, self.default_state),
            ButtonState::Hover => self.animate_to(cx, self.hover_state),
            _ => ()
        };
    }
    
    pub fn draw(&mut self, cx: &mut Cx, label: Option<&str>) {
        self.bg_quad.draw_walk(cx, self.walk);
    }
}


