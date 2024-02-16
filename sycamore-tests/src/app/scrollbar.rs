use crate::*;


#[component]
pub fn Scroll<G: Html>() -> View<G> {
  view! {
    div(class="full") {
      TestScrollComponent(is_lateral=true)
      TestScrollComponent(is_lateral=false)
    }
  }
}

#[component(inline_props)]
fn TestScrollComponent<G: Html>(is_lateral: bool) -> View<G> {

  let ship_outer = if is_lateral {"ship-outer ship-outer-x"} else { "ship-outer ship-outer-y"};
  let ship_box = if is_lateral {"center ship-box ship-box-x"} else { "center ship-box ship-box-y"};
  let scroll_bar = if is_lateral {"scroll-bar scroll-bar-x"} else {"scroll-bar scroll-bar-y"};

  let is_scrollable = create_signal(false);
  let iter = create_signal(vec![0, 1, 2]);

  let update_signal = create_memo(on(iter, move || {
    true
  }));  

  view! {
    div(style="padding: 20px;") {
      div(class=ship_outer) {
        ScrollBarComponent(
          is_lateral=is_lateral,
          take_orthogonal=true,
          is_scrollable=is_scrollable,
          update_signal=update_signal,
          class=scroll_bar,
          change_on_true=(*is_scrollable, Some("opacity0"), "opacity1")
        )
        Keyed(
          iterable=*iter,
          view=move |i| view! {
            div(class=ship_box) {(i)}
          },
          key=|i| *i,
        )
      }
      div() {
        button(class="rectbttn", on:click=move |_| {
          iter.update(|x| if let Some(id) = get_new_id(x, None) {
            x.push(id)
          });
        }) {"+"}
        button(class="rectbttn", on:click=move |_| {
          iter.update(|x| {let _ = x.pop();})
        }) {"-"}
      } 
    }
  }
}


fn get_new_id(ids: &[usize], max_size: Option<usize>) -> Option<usize> {

  if let Some(max_size) = max_size {
    if ids.len()>=max_size {
      return None;
    }
  }
  
  // (len, len-1, .. 0)
  (0..ids.len()+1).rev().find(|id| !ids.contains(id))
}