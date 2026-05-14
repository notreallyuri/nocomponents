use floating_ui_leptos::{Middleware, MiddlewareReturn, MiddlewareState, Placement};
use serde::{Deserialize, Serialize};
use web_sys::{Element, Window};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformOriginData {
    pub x: String,
    pub y: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransformOrigin;

impl Middleware<Element, Window> for TransformOrigin {
    fn name(&self) -> &'static str {
        "transformOrigin"
    }

    fn compute(&self, state: MiddlewareState<'_, Element, Window>) -> MiddlewareReturn {
        let placement = state.placement;

        let (placed_side, placed_align) = match placement {
            Placement::Top => ("top", "center"),
            Placement::TopStart => ("top", "start"),
            Placement::TopEnd => ("top", "end"),
            Placement::Bottom => ("bottom", "center"),
            Placement::BottomStart => ("bottom", "start"),
            Placement::BottomEnd => ("bottom", "end"),
            Placement::Left => ("left", "center"),
            Placement::LeftStart => ("left", "start"),
            Placement::LeftEnd => ("left", "end"),
            Placement::Right => ("right", "center"),
            Placement::RightStart => ("right", "start"),
            Placement::RightEnd => ("right", "end"),
        };

        let no_arrow_align = match placed_align {
            "start" => "0%",
            "end" => "100%",
            _ => "50%",
        };

        let float_h = state.rects.floating.height;
        let float_w = state.rects.floating.width;

        let (x, y) = match placed_side {
            "bottom" => (no_arrow_align.to_string(), "0px".to_string()),
            "top" => (no_arrow_align.to_string(), format!("{}px", float_h)),
            "right" => ("0px".to_string(), no_arrow_align.to_string()),
            "left" => (format!("{}px", float_w), no_arrow_align.to_string()),
            _ => ("50%".to_string(), "50%".to_string()),
        };

        MiddlewareReturn {
            x: None,
            y: None,
            reset: None,
            data: Some(serde_json::to_value(TransformOriginData { x, y }).unwrap()),
        }
    }
}
