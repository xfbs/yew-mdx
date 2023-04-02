use markdown::mdast::Node;
use markdown::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {
    pub source: String,
    pub render: Callback<String, Html>,
}

#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let ast = use_memo(
        |string| to_mdast(&string, &ParseOptions {
            constructs: Constructs {
                gfm_table: true,
                ..Constructs::gfm()
            },
            ..ParseOptions::gfm()
        }),
        props.source.clone(),
    );

    if let Ok(node) = &*ast {
        let node = node.clone();
        let render = props.render.clone();
        html! {
            <MarkdownNode {node} {render} />
        }
    } else {
        html! {{"Error"}}
    }
}

#[derive(Properties, PartialEq)]
pub struct MarkdownNodesProps {
    pub nodes: Vec<Node>,
    pub render: Callback<String, Html>,
}

#[function_component(MarkdownNodes)]
pub fn markdown_nodes(props: &MarkdownNodesProps) -> Html {
    props
        .nodes
        .iter()
        .cloned()
        .map(|node| html!{ <MarkdownNode {node} render={props.render.clone()} /> })
        .collect::<Html>()
}

#[derive(Properties, PartialEq)]
pub struct MarkdownNodeProps {
    pub node: Node,
    pub render: Callback<String, Html>,
}

#[function_component(MarkdownNode)]
pub fn markdown_node(props: &MarkdownNodeProps) -> Html {
    let render = props.render.clone();
    match &props.node {
        Node::Root(root) => html!(
            <MarkdownNodes nodes={root.children.clone()} {render} />
        ),
        Node::BlockQuote(quote) => {
            let nodes = quote.children.clone();
            html!(<blockquote><MarkdownNodes {nodes} {render} /></blockquote>)
        },
        Node::List(list) if !list.ordered => {
            let nodes = list.children.clone();
            html!(<ul><MarkdownNodes {nodes} {render} /></ul>)
        },
        Node::InlineCode(code) => html!(<code>{&code.value}</code>),
        Node::Delete(delete) => html!(<strike><MarkdownNodes nodes={delete.children.clone()} {render} /></strike>),
        Node::Heading(heading) => {
            let nodes = heading.children.clone();
            match heading.depth {
                1 => html!{<h1><MarkdownNodes {nodes} {render} /></h1>},
                2 => html!{<h2><MarkdownNodes {nodes} {render} /></h2>},
                3 => html!{<h3><MarkdownNodes {nodes} {render} /></h3>},
                4 => html!{<h4><MarkdownNodes {nodes} {render} /></h4>},
                5 => html!{<h5><MarkdownNodes {nodes} {render} /></h5>},
                6 => html!{<h6><MarkdownNodes {nodes} {render} /></h6>},
                _ => html!({"Unsupported heading"}),
            }
        },
        Node::Text(text) => html!({&text.value}),
        Node::Paragraph(paragraph) => {
            let nodes = paragraph.children.clone();
            html!{<p><MarkdownNodes {nodes} {render} /></p>}
        },
        Node::ThematicBreak(_) => html!{<hr />},
        Node::Emphasis(emphasis) => {
            let nodes = emphasis.children.clone();
            html!{<em><MarkdownNodes {nodes} {render} /></em>}
        },
        Node::Strong(strong) => {
            let nodes = strong.children.clone();
            html!{<strong><MarkdownNodes {nodes} {render} /></strong>}
        },
        Node::Code(code) => html!(<pre><code>{&code.value}</code></pre>),
        Node::Link(link) => html!(<a href={link.url.clone()}><MarkdownNodes nodes={link.children.clone()} {render} /></a>),
        Node::Break(_) => html!(<br />),
        Node::List(list) if list.ordered => {
            let nodes = list.children.clone();
            html!(<ol><MarkdownNodes {nodes} {render} /></ol>)
        },
        Node::ListItem(item) => {
            let nodes = item.children.clone();
            html!(<li><MarkdownNodes {nodes} {render} /></li>)
        },
        Node::Html(html) => render.emit(html.value.clone()),
        node => html!{{format!("Error: {node:?}")}},
    }
}
