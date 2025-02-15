use leptos::*;
use leptos_router::*;

const WAIT_ONE_SECOND: u64 = 1;
const WAIT_TWO_SECONDS: u64 = 2;

#[server(FirstWaitFn "/api")]
async fn first_wait_fn(seconds: u64) -> Result<(), ServerFnError> {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;

    Ok(())
}

#[server(SecondWaitFn "/api")]
async fn second_wait_fn(seconds: u64) -> Result<(), ServerFnError> {
    tokio::time::sleep(tokio::time::Duration::from_secs(seconds)).await;

    Ok(())
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let style = r#"
        nav {
            display: flex;
            width: 100%;
            justify-content: space-around;
        }

        [aria-current] {
            font-weight: bold;
        }
    "#;
    view! {
        cx,
        <style>{style}</style>
        <Router>
            <nav>
                <A href="/out-of-order">"Out-of-Order"</A>
                <A href="/in-order">"In-Order"</A>
                <A href="/async">"Async"</A>
            </nav>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| view! { cx, <Redirect path="/out-of-order"/> }
                    />
                    // out-of-order
                    <Route
                        path="out-of-order"
                        view=|cx| view! { cx,
                            <SecondaryNav/>
                            <h1>"Out-of-Order"</h1>
                            <Outlet/>
                        }
                    >
                        <Route path="" view=Nested/>
                        <Route path="inside" view=NestedResourceInside/>
                        <Route path="single" view=Single/>
                        <Route path="parallel" view=Parallel/>
                        <Route path="inside-component" view=InsideComponent/>
                        <Route path="none" view=None/>
                    </Route>
                    // in-order
                    <Route
                        path="in-order"
                        ssr=SsrMode::InOrder
                        view=|cx| view! { cx,
                            <SecondaryNav/>
                            <h1>"In-Order"</h1>
                            <Outlet/>
                        }
                    >
                        <Route path="" view=Nested/>
                        <Route path="inside" view=NestedResourceInside/>
                        <Route path="single" view=Single/>
                        <Route path="parallel" view=Parallel/>
                        <Route path="inside-component" view=InsideComponent/>
                        <Route path="none" view=None/>
                    </Route>
                    // async
                    <Route
                        path="async"
                        ssr=SsrMode::Async
                        view=|cx| view! { cx,
                            <SecondaryNav/>
                            <h1>"Async"</h1>
                            <Outlet/>
                        }
                    >
                        <Route path="" view=Nested/>
                        <Route path="inside" view=NestedResourceInside/>
                        <Route path="single" view=Single/>
                        <Route path="parallel" view=Parallel/>
                        <Route path="inside-component" view=InsideComponent/>
                        <Route path="none" view=None/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn SecondaryNav(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <A href="" exact=true>"Nested"</A>
            <A href="inside" exact=true>"Nested (resource created inside)"</A>
            <A href="single">"Single"</A>
            <A href="parallel">"Parallel"</A>
            <A href="inside-component">"Inside Component"</A>
            <A href="none">"No Resources"</A>
        </nav>
    }
}

#[component]
fn Nested(cx: Scope) -> impl IntoView {
    let one_second = create_resource(cx, || WAIT_ONE_SECOND, first_wait_fn);
    let two_second = create_resource(cx, || WAIT_TWO_SECONDS, second_wait_fn);
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div>
            <Suspense fallback=|| "Loading 1...">
                {move || {
                    one_second.read(cx).map(|_| view! {cx,
                        <p id="loaded-1">"One Second: Loaded 1!"</p>
                    })
                }}
                <Suspense fallback=|| "Loading 2...">
                    {move || {
                        two_second.read(cx).map(|_| view! { cx,
                            <p id="loaded-2">"Two Second: Loaded 2!"</p>
                            <button on:click=move |_| set_count.update(|n| *n += 1)>
                                {count}
                            </button>
                        })
                    }}
                </Suspense>
            </Suspense>
        </div>
    }
}

#[component]
fn NestedResourceInside(cx: Scope) -> impl IntoView {
    let one_second = create_resource(cx, || WAIT_ONE_SECOND, first_wait_fn);
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div>
            <Suspense fallback=|| "Loading 1...">
                   {move || {
                    one_second.read(cx).map(|_| {
                        let two_second = create_resource(cx, || (), move |_| async move {
                            leptos::log!("creating two_second resource");
                            second_wait_fn(WAIT_TWO_SECONDS).await
                        });
                        view! { cx,
                            {move || one_second.read(cx).map(|_|
                                view! {cx,
                                    <p id="loaded-1">"One Second: Loaded 1!"</p>
                                }
                            )}
                            <Suspense fallback=|| "Loading 2...">
                                {move || {
                                    two_second.read(cx).map(|x| view! { cx,
                                        <span id="loaded-2">"Loaded 2 (created inside first suspense)!: " {format!("{x:?}")}</span>
                                        <button on:click=move |_| set_count.update(|n| *n += 1)>
                                            {count}
                                        </button>
                                    })
                                }}
                            </Suspense>
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn Parallel(cx: Scope) -> impl IntoView {
    let one_second = create_resource(cx, || WAIT_ONE_SECOND, first_wait_fn);
    let two_second = create_resource(cx, || WAIT_TWO_SECONDS, second_wait_fn);
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div>
            <Suspense fallback=|| "Loading 1...">
                {move || {
                    one_second.read(cx).map(move |_| view! { cx,
                        <p id="loaded-1">"One Second: Loaded 1!"</p>
                        <button on:click=move |_| set_count.update(|n| *n += 1)>
                            {count}
                        </button>
                    })
                }}
            </Suspense>
            <Suspense fallback=|| "Loading 2...">
                {move || {
                    two_second.read(cx).map(move |_| view! { cx,
                        <p id="loaded-2">"Two Second: Loaded 2!"</p>
                        <button id="second-count" on:click=move |_| set_count.update(|n| *n += 1)>
                            {count}
                        </button>
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn Single(cx: Scope) -> impl IntoView {
    let one_second = create_resource(cx, || WAIT_ONE_SECOND, first_wait_fn);
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div>
            <Suspense fallback=|| "Loading 1...">
            {move || {
                one_second.read(cx).map(|_| view! {cx,
                    <p id="loaded-1">"One Second: Loaded 1!"</p>
                })
            }}
            </Suspense>
            <p id="following-message">"Children following Suspense should hydrate properly."</p>
            <div>
                <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </div>
        </div>
    }
}

#[component]
fn InsideComponent(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div>
            <p id="inside-message">"Suspense inside another component should work."</p>
            <InsideComponentChild/>
            <p id="following-message">"Children following Suspense should hydrate properly."</p>
            <div>
                <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </div>
        </div>
    }
}

#[component]
fn InsideComponentChild(cx: Scope) -> impl IntoView {
    let one_second = create_resource(cx, || WAIT_ONE_SECOND, first_wait_fn);
    view! { cx,
        <Suspense fallback=|| "Loading 1...">
        {move || {
            one_second.read(cx).map(|_| view! {cx,
                <p id="loaded-1">"One Second: Loaded 1!"</p>
            })
        }}
        </Suspense>
    }
}

#[component]
fn None(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! { cx,
        <div>
            <Suspense fallback=|| "Loading 1...">
                <p id="inside-message">"Children inside Suspense should hydrate properly."</p>
                <button on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </Suspense>
            <p id="following-message">"Children following Suspense should hydrate properly."</p>
            <div>
                <button id="second-count" on:click=move |_| set_count.update(|n| *n += 1)>
                    {count}
                </button>
            </div>
        </div>
    }
}
