use yew_mdx::*;
use yew::prelude::*;
use roxmltree::{Node, Document};

#[function_component(Component)]
fn component() -> Html {
    html!(<b>{"Component here"}</b>)
}

fn render(node: Node<'_, '_>) -> Html {
    match node.tag_name().name() {
        "Component" => html!(<Component />),
        _ => html!({format!("Error {node:?}")}),
    }
}

#[function_component(App)]
fn app() -> Html {
    let source = include_str!("../index.md");
    let render = |string: String| {
        let document = Document::parse(&string).unwrap();
        document.root().children().map(render).collect::<Html>()
    };
    html!{
        <Markdown source={source.to_string()} {render} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
