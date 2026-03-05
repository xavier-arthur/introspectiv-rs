use yew::prelude::*;
use yew_router::prelude::Link;
use lucide_yew::ChevronDown;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub children: yew::Children
}

#[component[TopBar]]
fn topbar() -> Html {
    use crate::routes::Route;

    html! {
        <nav class="block w-full bg-dark border-b h-5">
            <ul class="flex items-center gap-x-2 [&>li]:text-xs">
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

                <li class="relative group">
                    <Link<Route> to={Route::About} classes={"block space-x-0.5"}>
                        { "Projects" }
                        <ChevronDown class="inline size-4" />
                    </Link<Route>>

                    <div class="hidden group-hover:block absolute top-4 bg-dark-1 w-full">
                        <Link<Route> to={Route::Gol}>
                            { "GOL" }
                        </Link<Route>>
                    </div>
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

            <main class="container mx-auto p-8 h-[calc(100%-1.250rem)]">
                { props.children.clone() }
            </main>
        </>
    }
}