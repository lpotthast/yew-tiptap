use crate::{js_tiptap::State, ImageResource};
use tracing::error;
use wasm_bindgen::prelude::Closure;
use yew::{html::Scope, prelude::*};

use super::js_tiptap;

pub enum Msg {
    /// This is an "internal" event, meaning that it SHOULD NOT BE CREATED MANUALLY.
    /// It is automatically triggered from the JS tiptap instance whenever its selection changed.
    _SelectionChanged,

    /// This is an "internal" event, meaning that it SHOULD NOT BE CREATED MANUALLY.
    /// It is automatically triggered from the JS tiptap instance whenever its content changed.
    _ContentChanged { content: String },

    /// Toggles "H1" for the current selection.
    H1,

    /// Toggles "H2" for the current selection.
    H2,

    /// Toggle "H3" for the current selection.
    H3,

    /// Toggle "Paragraph" for the current selection.
    Paragraph,

    /// Toggle "Bold" for the current selection.
    Bold,

    /// Toggle "Italic" for the current selection.
    Italic,

    /// Toggle "Strike" for the current selection.
    Strike,

    /// Toggle "Blockquote" for the current selection.
    Blockquote,

    /// Toggle "Highlight" for the current selection.
    Highlight,

    /// Toggle "AlignLeft" for the current selection.
    AlignLeft,

    /// Toggle "AlignCenter" for the current selection.
    AlignCenter,

    /// Toggle "AlignRight" for the current selection.
    AlignRight,

    /// Toggle "AlignJustify" for the current selection.
    AlignJustify,

    /// Replace the current selection with an image.
    SetImage(ImageResource),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Selection {
    pub state: SelectionState,
}

pub type SelectionState = js_tiptap::State;
pub type HeadingLevel = js_tiptap::HeadingLevel;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Content {
    pub content: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,

    /// CSS class given to the tiptap instance.
    /// Defaults to "tiptap-instance" if not specified.
    pub class: Option<String>,

    /// Initial content of the editor.
    pub content: String,

    /// If set to true, the tiptap instance becomes un-editable.
    pub disabled: bool,

    pub on_link: Callback<Option<Scope<TiptapInstance>>>,
    pub on_selection_change: Option<Callback<Selection>>,
    pub on_content_change: Option<Callback<Content>>,
}

pub struct TiptapInstance {
    /// This closure is passed on to the JS tiptap instance.
    /// We expect this to be called whenever the INPUT in the editor changes.
    /// We have to own this closure until the end of this components lifetime.
    on_change: Closure<dyn Fn(String)>,

    /// This closure is passed on to the JS tiptap instance.
    /// We expect this to be called whenever the SELECTION in the editor changes.
    /// We have to own this closure until the end of this components lifetime.
    on_selection: Closure<dyn Fn()>,
}

fn fetch_selection_state(ctx: &Context<TiptapInstance>) -> State {
    match js_tiptap::get_state(ctx.props().id.clone()) {
        Ok(state) => state,
        Err(err) => {
            error!("Could not parse JsValue as TipTap state. Deserialization error: '{err}'. Falling back to default state.");
            Default::default()
        }
    }
}

impl Component for TiptapInstance {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        // Linking is deferred. See `rendered` function below.
        ctx.props().on_link.emit(None);

        let changed_callback = ctx
            .link()
            .callback(|content| Msg::_ContentChanged { content });
        let changed =
            Closure::wrap(Box::new(move |text| changed_callback.emit(text)) as Box<dyn Fn(String)>);

        let selection_changed_callback = ctx.link().callback(|_| Msg::_SelectionChanged);
        let selected =
            Closure::wrap(Box::new(move || selection_changed_callback.emit(())) as Box<dyn Fn()>);

        Self {
            on_change: changed,
            on_selection: selected,
        }
    }

    fn destroy(&mut self, ctx: &Context<Self>) {
        ctx.props().on_link.emit(None);
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::_SelectionChanged => {
                if let Some(on_selection_change) = &ctx.props().on_selection_change {
                    on_selection_change.emit(Selection {
                        state: fetch_selection_state(ctx),
                    });
                }
                false
            }
            Msg::_ContentChanged { content } => {
                if let Some(on_content_change) = &ctx.props().on_content_change {
                    on_content_change.emit(Content { content });
                }
                false
            }
            Msg::H1 => {
                js_tiptap::toggle_heading(ctx.props().id.clone(), js_tiptap::HeadingLevel::H1);
                true
            }
            Msg::H2 => {
                js_tiptap::toggle_heading(ctx.props().id.clone(), js_tiptap::HeadingLevel::H2);
                true
            }
            Msg::H3 => {
                js_tiptap::toggle_heading(ctx.props().id.clone(), js_tiptap::HeadingLevel::H3);
                true
            }
            Msg::Paragraph => {
                js_tiptap::set_paragraph(ctx.props().id.clone());
                true
            }
            Msg::Bold => {
                js_tiptap::toggle_bold(ctx.props().id.clone());
                true
            }
            Msg::Italic => {
                js_tiptap::toggle_italic(ctx.props().id.clone());
                true
            }
            Msg::Strike => {
                js_tiptap::toggle_strike(ctx.props().id.clone());
                true
            }
            Msg::Blockquote => {
                js_tiptap::toggle_blockquote(ctx.props().id.clone());
                true
            }
            Msg::Highlight => {
                js_tiptap::toggle_highlight(ctx.props().id.clone());
                true
            }
            Msg::AlignLeft => {
                js_tiptap::set_text_align_left(ctx.props().id.clone());
                true
            }
            Msg::AlignCenter => {
                js_tiptap::set_text_align_center(ctx.props().id.clone());
                true
            }
            Msg::AlignRight => {
                js_tiptap::set_text_align_right(ctx.props().id.clone());
                true
            }
            Msg::AlignJustify => {
                js_tiptap::set_text_align_justify(ctx.props().id.clone());
                true
            }
            Msg::SetImage(resource) => {
                js_tiptap::set_image(
                    ctx.props().id.clone(),
                    resource.url.clone(),
                    resource.alt.clone(),
                    resource.title.clone(),
                );
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id={ctx.props().id.clone()} class={ctx.props().class.as_ref().map(|it| it.clone()).unwrap_or("tiptap-instance".to_owned())}></div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            js_tiptap::create(
                ctx.props().id.clone(),
                ctx.props().content.clone(),
                !ctx.props().disabled,
                &self.on_change,
                &self.on_selection,
            );

            // NOTE: Linking is deferred until tiptap instance is known to be ready!
            // The user of this library would otherwise be able to send messages like `H1` before the instance was even created which would only lead to errors.
            ctx.props().on_link.emit(Some(ctx.link().clone()));
        }
    }
}
