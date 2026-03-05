use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: yew::Children
}

#[component[TopBar]]
fn topbar() -> Html {
    use crate::routes::Route;

    html! {
        <nav class="w-full bg-dark block">
            <ul style="list-style: none; display: flex">
                <li>
                    <Link<Route> to={Route::Index}>
                        {"Index"}
                    </Link<Route>>
                </li>

                <li>
                    <Link<Route> to={Route::About}>
                        {"About"}
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}

#[component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    html! {
        <>
            <TopBar />

            <main class="container">
                { props.children.clone() }
            </main>
        </>
    }
}