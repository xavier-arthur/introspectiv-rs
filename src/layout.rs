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
        <nav class="block w-full bg-dark border-b">
            <ul class="flex gap-x-2 [&>li]:text-xs">
                <li class="">
                    <Link<Route> to={Route::Index}>
                        {"Home"}
                    </Link<Route>>
                </li>

                <li>
                    <Link<Route> to={Route::About}>
                        {"About"}
                    </Link<Route>>
                </li>

                <li>
                    <Link<Route> to={Route::About}>
                        { "Projects" }
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