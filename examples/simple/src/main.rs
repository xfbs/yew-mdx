use yew_mdx::*;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let source = include_str!("test.md");
    html!{
        <Markdown source={source.to_string()} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
