use crate::*;

/// ChangeClass
/// - test its macros
/// 
#[component]
pub fn ChangeClassView<G: Html>() -> View<G> {

  let signal = create_signal(false);

  let rf1 = create_node_ref();
  let rf2 = create_node_ref();

  change_class_on_true!(rf1, *signal, "chgcls0", "chgcls1");
  change_class_on_true!(rf2, *signal, "chgcls1");

  view! {
    button(class="rectbttn", on:click=move |_| signal.set(!signal.get())) {(if signal.get() {"+"} else {"-"})}
    div(ref=rf1, class="chgcls-box") {}
    div(ref=rf2, class="chgcls-box") {}
  }
}