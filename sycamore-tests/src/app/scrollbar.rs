use crate::*;


#[component]
pub fn Scroll<G: Html>() -> View<G> {

  let window_resizing = create_signal(false);
  listen_window_resize_event(window_resizing);

  let is_scrollable_y = create_signal(false);
  let is_scrollable_x = create_signal(false);

  let is_scrolling_y = create_signal(false);
  let is_scrolling_x = create_signal(false);

  // full scroll component - test sync_scroll_absolute_position
  let sync_y_left = create_signal(false);
  let sync_y = create_signal(false);
  let sync_x_top = create_signal(false);
  let sync_x = create_signal(false);

  create_effect(on((window_resizing, is_scrolling_x), move || { sync_y_left.set(true); sync_y.set(true); }));
  create_effect(on((window_resizing, is_scrolling_y), move || { sync_x_top.set(true); sync_x.set(true); }));

  let rf = create_node_ref();
  let rf_y_left = create_node_ref();
  let rf_y = create_node_ref();
  let rf_x_top = create_node_ref();
  let rf_x = create_node_ref();

  sync_scroll_absolute_position(*sync_y_left, rf, rf_y_left, true, true);
  sync_scroll_absolute_position(*sync_y, rf, rf_y, true, false);
  sync_scroll_absolute_position(*sync_x_top, rf, rf_x_top, false, true);
  sync_scroll_absolute_position(*sync_x, rf, rf_x, false, false);

  view! {
    div(ref=rf, class="full xscrollbar overflow-y overflow-x rel") {

      // full scroll component's scrollbars
      // left bar
      ScrollBarComponent(
        bar_rf=rf_y_left,
        is_lateral=false,
        take_orthogonal=false,
        is_scrollable=is_scrollable_y,
        update_scrollbar=*window_resizing,
        class="scrollbar3 scrollbar-y-left",
        change_on_true=(*is_scrollable_y, Some("opacity0"), "opacity08")
      )
      // top bar
      ScrollBarComponent(
        bar_rf=rf_x_top,
        is_lateral=true,
        take_orthogonal=false,
        is_scrollable=is_scrollable_x,
        update_scrollbar=*window_resizing,
        class="scrollbar3 scrollbar-x-top",
        change_on_true=(*is_scrollable_x, Some("opacity0"), "opacity08")
      )

      // right bar
      ScrollBarComponent(
        bar_rf=rf_y,
        is_lateral=false,
        take_orthogonal=false,
        is_scrollable=is_scrollable_y,
        is_scrolling=is_scrolling_y,
        update_scrollbar=*window_resizing,
        class="scrollbar2 scrollbar-y",
        change_on_true=(*is_scrollable_y, Some("opacity0"), "opacity08")
      )
      // bottom bar
      ScrollBarComponent(
        bar_rf=rf_x,
        is_lateral=true,
        take_orthogonal=false,
        is_scrollable=is_scrollable_x,
        is_scrolling=is_scrolling_x,
        update_scrollbar=*window_resizing,
        class="scrollbar2 scrollbar-x",
        change_on_true=(*is_scrollable_x, Some("opacity0"), "opacity08")
      )

      // Test more contained cases
      div(style="margin: 10px") {
        p() {
          span(style="margin-right: 15px;") {"ScrollBar of"}
          span(class="highlight") { "Sycamore Seoul"}
        }
      }
      TestScrollComponent(is_lateral=true)
      TestScrollComponent(is_lateral=false)
    }
  }
}


#[component(inline_props)]
fn TestScrollComponent<G: Html>(is_lateral: bool) -> View<G> {

  let ship_outer = if is_lateral {"xscrollbar ship-outer ship-outer-x"} else { "xscrollbar ship-outer ship-outer-y"};
  let ship_box = if is_lateral {"center ship-box ship-box-x"} else { "center ship-box ship-box-y"};
  let scrollbar = if is_lateral {"scrollbar scrollbar-x"} else {"scrollbar scrollbar-y"};

  let is_scrollable = create_signal(false);
  let iter: Signal<Vec<usize>> = create_signal((0..10).collect());

  let to_add = move |_| {
    iter.update(|x| {
      let id = new_id(x);
      x.push(id);
    });
  };
  
  let to_subtract = move |_| {
    iter.update(|x| {
      if x.len()>1 {
        let _ = x.pop();
      }
    });
  };

  view! {
    div(style="padding: 20px;") {
      div(class=ship_outer) {
        ScrollBarComponent(
          is_lateral=is_lateral,
          take_orthogonal=if is_lateral {true} else {false}, //
          is_scrollable=is_scrollable,
          update_scrollbar=*iter,
          class=scrollbar,
          change_on_true=(*is_scrollable, Some("opacity0"), "opacity08")
        )
        Keyed(
          iterable=*iter,
          view=move |i| view! {
            div(class=ship_box) {(i)}
          },
          key=|i| *i,
        )
      }
      div(class="flex-x") {
        div(class="rect-bttn center",  on:click=to_add) {"+"}
        div(class="rect-bttn center", on:click=to_subtract) {"-"}
      } 
    }
  }
}