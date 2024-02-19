use crate::*;


fn panel_arrow<'a>(is_lateral: bool, to_left: bool) -> &'a str {
  if is_lateral {
    if to_left {"←"} else {"→"}
  } else {
    if to_left {"↑"} else {"↓"}
  }
}

/// TestResizer
/// 
/// ```
/// -----------------------------------------------------
/// | Panel1(↓)                   | Panel2(←)           |
/// --------------------------------------------------- |
/// |           | Panel4(↓) (Parcels4(↔))               |
/// | Panel3(→) | ------------------------------------- |
/// |           | Panel5(→) (Parcels5(↕)) | parcels6(↕) |
/// -----------------------------------------------------
/// ```
/// 
#[component]
pub fn TestResizer<G: Html>() -> View<G> {

  view! {
    div(class="full flex-y overflow-clip") {

      // Panel1 (Vertical)
      Panel(
        is_lateral=false, to_pixel=false,
      ) {
        div(class="full flex-x", style="justify-content: flex-end;") {
          div(class="flex-grow1") {
            div(style="padding: 5px; margin-left: 10px;") {
              p() {
                span(style="margin-right: 15px;") {"Resizer of"}
                span(class="highlight") { "Sycamore Seoul"}
              }
              p() { "All lines are resize-able"}
            }
          }

          Panel(is_lateral=true, to_left=true, to_pixel=false,
            use_phrase=true,
          )
        }
      }

      // Than Panel1
      div(class="flex-grow1 flex-x overflow-clip") {

        // Panel3 (lateral)
        Panel(
          is_lateral=true, to_pixel=true,
          use_phrase=true,
        )

        // Than Panel3
        div(class="flex-grow1 flex-y overflow-clip") {

          // Panel4 (vertical)
          Panel(
            is_lateral=false, to_pixel=false
          ) {
            // Parcels4 (lateral)
            Parcels(is_lateral=true, to_pixel=false)
          }

          // Than Panel4
          div(class="flex-grow1 flex-x overflow-clip") {

            // Panel5 (lateral)
            Panel(
              is_lateral=true, to_pixel=false,
            ) {
              // Parcels5 (vertical)
              Parcels(is_lateral=false, to_pixel=false)
            }

            // Than Panel5
            div(class="flex-grow1") {
              // Parcels6 (vertical)
              Parcels(is_lateral=false, to_pixel=true)
            }
          }
        }
      }
    }
  }
}


#[component(inline_props)]
fn Panel<G: Html>(
  panel_moving: Option<Signal<bool>>,
  panel_length: Option<Signal<StyleLength>>,
  is_lateral: bool,
  to_left: Option<bool>,
  to_pixel: bool,
  children: Children<G>,
  use_phrase: Option<bool>,
) -> View<G> {

  // designateds

  let min_len=StyleLength::Pixel(50.);
  let max_len=StyleLength::Pixel(1000.);

  let (
    class, change_class_on_move,
    scrollbar_class
  ) = 
    if is_lateral {
      (
        "panel-x", (Some("resizer-static2"), "resizer-moving2"),
        "scrollbar scrollbar-x",
      )
    } else {
      (
        "panel-y", (Some("resizer-static"), "resizer-moving"),
        "scrollbar scrollbar-y",
      )
    };

  let to_left = to_left.unwrap_or(false);

  let resizer_class = if is_lateral {
    if to_left { "resizer-left resizer-static2" } else { "resizer-right resizer-static2"}
  } else {
    if to_left { "resizer-top resizer-static" } else { "resizer-bottom resizer-static"}
  };

  let scrollbar_change_on_true=(Some("opacity0"), "opacity08");


  // props
  let use_phrase = use_phrase.unwrap_or(false);

  let panel_moving = panel_moving.unwrap_or(create_signal(false));
  let panel_length = panel_length.unwrap_or(create_signal(StyleLength::default()));

  // scrollbar
  let is_scrollable = create_signal(false);
  let update_scrollbar = create_signal(false);

  let window_resizing = create_signal(false);
  listen_window_resize_event(window_resizing);

  create_effect(on((window_resizing, panel_length), move || {
    update_scrollbar.set(true);
  }));

  // children
  let children = children.call();

  view! {
    div(class=class) {
      // resizer
      PanelResizer(
        moving=panel_moving,
        panel_length=panel_length,
        class=resizer_class,
        is_lateral=is_lateral, to_left=to_left, to_pixel=to_pixel,
        min_len=min_len,
        max_len=max_len,
        change_class_on_move=change_class_on_move,
      ) {}

      // in-panel
      // * As resizer is absolute to its parent, the parent should be {overflow: visual;} to let resizer fully revealed.
      //   This is why here is used another wrapper of {overflow: (not visual);}
      div(class=&format!("full {} xscrollbar rel", 
        if is_lateral { "overflow-x" } else { "overflow-y"}
      )) {
        // ScrollBar
        ScrollBarComponent(
          is_lateral=is_lateral,
          take_orthogonal=if is_lateral { true } else { false },
          is_scrollable=is_scrollable,
          update_scrollbar=*update_scrollbar,
          class=scrollbar_class,
          change_on_true=(*is_scrollable, scrollbar_change_on_true.0, scrollbar_change_on_true.1)
        )

        (children)

        div() {
          (if use_phrase {
            view! {
              div(style="margin-left: 5px;") {
                div(class="flex-align-items-center") {
                  p(style="margin-right: 10px;") {(format!("Panel ({})", panel_arrow(is_lateral, to_left)))}
                  (if let Some(v) = panel_length.with(|x| {
                    let v = x.style_value();
                    if v.len()>0 { Some(v)} else {None}
                  }) { view! {
                    p() { "current width: " (v)}
                  }} else { view! { }})
                }
              }
            }
          } else { view! { }})
        }
      }
    }
  }
}


#[component(inline_props)]
fn Parcels<G: Html>(
  parcels: Option<Signal<Vec<usize>>>, 
  parcel_lengths: Option<Signal<HashMap<usize, StyleLength>>>,
  is_lateral: bool,
  to_left: Option<bool>,
  to_pixel: bool
) -> View<G> {

  // designateds

  let min_len=StyleLength::Pixel(100.);
  let max_len=StyleLength::Percent(95.);

  let (
    class, 
    parcel_class,
    change_class_on_move,
    scrollbar_class
  ) = 
    if is_lateral {
      (
        "full flex-x overflow-x xscrollbar rel", 
        "parcel-x flex-grow1",
        (Some("resizer-static2"), "resizer-moving2"),
        "scrollbar scrollbar-x",
      )
    } else {
      (
        "full flex-y overflow-y xscrollbar rel", 
        "parcel-y flex-grow1",
        (Some("resizer-static"), "resizer-moving"),
        "scrollbar scrollbar-y",
      )
    };

  let to_left = to_left.unwrap_or(false);

  let resizer_class = if is_lateral {
    if to_left { "resizer-left resizer-static2" } else { "resizer-right resizer-static2"}
  } else {
    if to_left { "resizer-top resizer-static" } else { "resizer-bottom resizer-static"}
  };

  let scrollbar_change_on_true=(Some("opacity0"), "opacity08");


  // props
  let parcels = parcels.unwrap_or(create_signal(vec![0, 1, 2]));
  let parcel_lengths = parcel_lengths.unwrap_or(create_signal(HashMap::new()));

  // scrollbar
  let is_scrollable = create_signal(false);
  let update_scrollbar = create_signal(false);

  let window_resizing = create_signal(false);
  listen_window_resize_event(window_resizing);

  create_effect(on((window_resizing, parcels), move || {
    update_scrollbar.set(true);
  }));


  view! {
    div(class=class) {
      // ScrollBar!
      ScrollBarComponent(
        is_lateral=is_lateral,
        take_orthogonal=false,
        is_scrollable=is_scrollable,
        update_scrollbar=*update_scrollbar,
        class=scrollbar_class,
        change_on_true=(*is_scrollable, scrollbar_change_on_true.0, scrollbar_change_on_true.1)
      )

      //
      Keyed(
        iterable=*parcels,
        view=move |p| view! {
          Parcel(
            p=p, parcels=parcels, parcel_lengths=parcel_lengths, 
            is_lateral=is_lateral, to_left=to_left, to_pixel=to_pixel, min_len=min_len, max_len=max_len, parcel_class=parcel_class, resizer_class=resizer_class, change_class_on_move=change_class_on_move,
            is_scrollable=*is_scrollable
          )
        },
        key=|p| *p,
      )
    }
  }
}


#[component(inline_props)]
fn Parcel<G: Html>(
  p: usize, parcels: Signal<Vec<usize>>, parcel_lengths: Signal<HashMap<usize, StyleLength>>,
  is_lateral: bool, to_left: bool, to_pixel: bool, min_len: StyleLength, max_len: StyleLength, parcel_class: &'static str, resizer_class: &'static str, change_class_on_move: (Option<&'static str>, &'static str),
  is_scrollable: ReadSignal<bool>
) -> View<G> {


  let current_length = if is_lateral { "current width: "} else { "current height: "};

  let to_add = move |_| {
    parcels.update(|x| {
      let id = new_id(x);
      let i = x.iter().position(|x| x==&p).unwrap();
      x.insert(i+1, id);
    });
  };
  
  let to_subtract = move |_| {
    parcels.update(|x| {
      if x.len()>1 {
        x.retain(|x| x!=&p);
      }
    });
  };

  let is_terminal = create_memo(on(parcels, move || {
    if to_left {
      parcels.with(|x| x.first().map(|x| x==&p).unwrap_or(false))
    } else {
      parcels.with(|x| x.last().map(|x| x==&p).unwrap_or(false))
    }
  }));

  let resizer = create_memo(move || {
    
    view! {
      ParcelsResizer(
        parcel_lengths=parcel_lengths,
        parcel_name="parcel",
        class=resizer_class,
        change_class_on_move=change_class_on_move,
        is_lateral=is_lateral,
        to_left=to_left,
        to_pixel=to_pixel,
        min_len=min_len,
        max_len=max_len,
      )
    }
  });
  
  view! {
    div(class=parcel_class, data-parcel=p) {
      // prevent resizing when is-scrollable!
      (if !is_terminal.get() && !is_scrollable.get() {
        view! { (resizer.get_clone()) }
      } else {
        view! { }
      })
      div(class="full overflow-clip") {
        div(style="padding: 5px;") {
          div(class="flex-align-items-center", style="margin: 5px;") {
            (view! {div(
              class="circle-bttn center",
              on:click=to_add
            ) {"+"}})
            (if parcels.with(|x| x.len()>1) {
              view! {
                div(
                  class="circle-bttn center",
                  on:click=to_subtract
                ) {"-"}
              }
            } else {
              view! {}
            })
            p(style="margin-left: 10px; margin-right: 10px;") {"Parcel " (p)}
            (if let Some(x) = parcel_lengths.with(|x| x.get(&p).map(|x| x.style_value())) {
              view! { p() { (current_length) (x) }}
            } else { view! { }})
          }
        }
      }
    }
  }
}