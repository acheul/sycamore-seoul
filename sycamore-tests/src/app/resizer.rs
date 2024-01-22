use crate::*;

#[component]
pub fn TestResizer<G: Html>() -> View<G> {

  let length = create_signal(0.);

  create_effect(on(length, move || {
    if length.get()>0. {
      log!(format!("panel length: {:.2}%", length.get()));
    }
  }));

  let resizer_rf = create_node_ref();

  Resizer {
    is_panel: true,
    is_lateral: true,
    to_left: false,
    min_length: Some(StyleLength::Pixel(100.)),
    max_length: Some(StyleLength::Pixel(1000.)),
    replace_class_on_move: ("resizer-static", "resizer-moving"),
    resizer_rf,
  }.set_event_handler(Some(length), None);

  view! {
    div(class="full container") {
      div(class="panel") {
        div(ref=resizer_rf, class="resizer-right resizer-static") {}
        div(class="full parcels-vertical") {
          Parcels(
            is_lateral=false, to_left=true,
            min_length=StyleLength::Percent(5.), max_length=StyleLength::Percent(50.), parcel_class="parcel-vertical", resizer_class="resizer-top",
          )
        }
      }
      div(class="parcels") {
        Parcels(
          is_lateral=true, to_left=false,
          min_length=StyleLength::Pixel(100.), max_length=StyleLength::Pixel(1000.), parcel_class="parcel", resizer_class="resizer-right"
        )
      }
    }
  }
}


#[component(inline_props)]
fn Parcels<G: Html>(is_lateral: bool, to_left: bool, min_length: StyleLength, max_length: StyleLength, parcel_class: &'static str, resizer_class: &'static str) -> View<G> {

  let parcels = create_signal(vec![0, 1, 2, 3]);

  view! {
    Keyed(
      iterable=*parcels,
      view=move |p| view! {
        Parcel(
          is_lateral=is_lateral, to_left=to_left, min_length=min_length, max_length=max_length, parcel_class=parcel_class, resizer_class=resizer_class,
          parcels=parcels, p=p
        )
      },
      key=|p| *p,
    )
  }
}

#[component(inline_props)]
fn Parcel<G: Html>(
  is_lateral: bool, to_left: bool, min_length: StyleLength, max_length: StyleLength, parcel_class: &'static str, resizer_class: &'static str,
  parcels: Signal<Vec<i32>>, p: i32
) -> View<G> {

  let changed = create_signal(false);

  create_effect(on(changed, move || {
    if changed.get() {
      log!(p, "changed");
    }
  }));

  let is_terminal = create_memo(on(parcels, move || {
    if to_left {
      parcels.with(|x| x.get(0).map(|x| x==&p).unwrap_or(false))
    } else {
      parcels.with(|x| x.last().map(|x| x==&p).unwrap_or(false))
    }
  }));

  let resizer = create_memo(move || {
    let resizer_rf = create_node_ref();

    Resizer {
      is_panel: false,
      is_lateral: is_lateral,
      to_left: to_left,
      min_length: Some(min_length),
      max_length: Some(max_length),
      replace_class_on_move: ("resizer-static", "resizer-moving"),
      resizer_rf,
    }.set_event_handler(None, Some(changed));
    
    view! {
      div(ref=resizer_rf, class=[resizer_class, "resizer-static"].join(" ")) {}
    }
  });
  
  view! {
    div(class=[parcel_class, "center"].join(" ")) {
      (if !is_terminal.get() {
        view! { (resizer.get_clone()) }
      } else {
        view! { }
      })
      (p)
    }
  }
}