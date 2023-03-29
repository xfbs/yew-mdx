use markdown::mdast::{Node, Root};
use markdown::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub source: String,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let ast = use_memo(
        |string| to_mdast(&string, &Default::default()),
        props.source.clone(),
    );

    if let Ok(Node::Root(root)) = &*ast {
        let root = root.clone();
        html! {
            <MarkdownRoot {root} />
        }
    } else {
        html! {{"Error"}}
    }
}

#[derive(Properties, PartialEq)]
pub struct MarkdownRootProps {
    pub root: Root,
}

#[function_component(MarkdownRoot)]
pub fn markdown_root(props: &MarkdownRootProps) -> Html {
    html! {
        <div class="markdown">
        {
            props
                .root
                .children
                .iter()
                .cloned()
                .map(|node| html!{ <MarkdownNode {node} /> })
                .collect::<Html>()
        }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct MarkdownNodeProps {
    pub node: Node,
}

#[function_component(MarkdownNode)]
pub fn markdown_node(props: &MarkdownNodeProps) -> Html {
    match &props.node {
        _ => html!{{"Error"}},
    }
}
