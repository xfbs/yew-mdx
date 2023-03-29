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
pub struct MarkdownNodesProps {
    pub nodes: Vec<Node>,
}

#[function_component(MarkdownNodes)]
pub fn markdown_nodes(props: &MarkdownNodesProps) -> Html {
    props
        .nodes
        .iter()
        .cloned()
        .map(|node| html!{ <MarkdownNode {node} /> })
        .collect::<Html>()
}

#[derive(Properties, PartialEq)]
pub struct MarkdownNodeProps {
    pub node: Node,
}

#[function_component(MarkdownNode)]
pub fn markdown_node(props: &MarkdownNodeProps) -> Html {
    match &props.node {
        Node::Heading(heading) => {
            let nodes = heading.children.clone();
            match heading.depth {
                1 => html!{<h1><MarkdownNodes {nodes} /></h1>},
                2 => html!{<h2><MarkdownNodes {nodes} /></h2>},
                3 => html!{<h3><MarkdownNodes {nodes} /></h3>},
                4 => html!{<h4><MarkdownNodes {nodes} /></h4>},
                5 => html!{<h5><MarkdownNodes {nodes} /></h5>},
                6 => html!{<h6><MarkdownNodes {nodes} /></h6>},
                _ => html!({"Unsupported heading"}),
            }
        },
        Node::Text(text) => html!({&text.value}),
        Node::Paragraph(paragraph) => {
            let nodes = paragraph.children.clone();
            html!{<p><MarkdownNodes {nodes} /></p>}
        },
        Node::ThematicBreak(_) => html!{<hr />},
        Node::Emphasis(emphasis) => {
            let nodes = emphasis.children.clone();
            html!{<em><MarkdownNodes {nodes} /></em>}
        },
        Node::Strong(strong) => {
            let nodes = strong.children.clone();
            html!{<strong><MarkdownNodes {nodes} /></strong>}
        },
        Node::InlineCode(code) => html!(<code>{&code.value}</code>),
        Node::Break(_) => html!(<br />),
        Node::BlockQuote(quote) => {
            let nodes = quote.children.clone();
            html!(<blockquote><MarkdownNodes {nodes} /></blockquote>)
        },
        Node::List(list) if list.ordered => {
            let nodes = list.children.clone();
            html!(<ol><MarkdownNodes {nodes} /></ol>)
        },
        Node::List(list) if !list.ordered => {
            let nodes = list.children.clone();
            html!(<ul><MarkdownNodes {nodes} /></ul>)
        },
        Node::ListItem(item) => {
            let nodes = item.children.clone();
            html!(<li><MarkdownNodes {nodes} /></li>)
        },
        _ => html!{{"Error"}},
    }
}
