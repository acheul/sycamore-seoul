mod comps;
mod resizer;
mod scrollbar;

use crate::*;
use sycamore_router::{Route, Router, HistoryIntegration};

#[component]
fn Index<G: Html>() -> View<G> {
  view! {
    div(class="full center") {
      div() {
        p() { a(class="index", href="/resizer") { "Test Resizer" } }
        p() { a(class="index", href="/chgcls") { "Test ChangeClass" } }
        p() { a(class="index", href="/scroll") { "Test Scroll" } }
      }
    }
  }
}

#[derive(Clone, Copy, Route, Debug)]
enum Routes {
  #[to("/")] Index,
  #[to("/resizer")] Resizer,
  #[to("/chgcls")] ChangeClass,
  #[to("/scroll")] Scroll,
  #[not_found]
  NotFound,
}

fn switch<G: Html>(route: ReadSignal<Routes>) -> View<G> {
  let view = create_memo(on(route, move || match route.get() {
    Routes::Index => view! { Index },
    Routes::Resizer => view! { resizer::TestResizer },
    Routes::ChangeClass => view! { comps::ChangeClassView },
    Routes::Scroll => view! { scrollbar::Scroll },
    Routes::NotFound => view! { "NotFound" },
  }));
  view! { (view.get_clone()) }
}

#[component]
pub fn App<G: Html>() -> View<G> {
  view! {
    main() {
      Router(
        integration=HistoryIntegration::new(),
        view=switch,
      )
    }
  }
}