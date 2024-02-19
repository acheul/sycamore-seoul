use crate::*;

// resizer components

// panel

/// PanelResizerProps
/// 
#[derive(Props)]
pub struct PanelResizerProps{
  moving: Option<Signal<bool>>,
  panel_length: Option<Signal<StyleLength>>,
  class: &'static str,
  is_lateral: bool,
  to_left: bool,
  to_pixel: bool,
  min_len: Option<StyleLength>,
  max_len: Option<StyleLength>,
  change_class_on_move: Option<(Option<&'static str>, &'static str)>,
  skip_set_style: Option<bool>,
}

/// PanelResizer
/// * Panel type
/// * Loot at `Resizer`'s `set_panel_resizer` for more information.
/// 
/// # Props
/// * moving: Option<Signal<bool>>,
/// * panel_length: Option<Signal<StyleLength>>,
/// * class: &'static str,
/// * is_lateral: bool,
/// * to_left: bool,
/// * to_pixel: bool,
/// * min_len: Option<StyleLength>,
/// * max_len: Option<StyleLength>,
/// * change_class_on_move: Option<(Option<&'static str>, &'static str)>,
/// * skip_set_style: Option<bool>,
/// 
#[component]
pub fn PanelResizer<G: Html>(props: PanelResizerProps) -> View<G> {

  let rf = create_node_ref();

  Resizer {
    is_lateral: props.is_lateral,
    to_left: props.to_left,
    to_pixel: props.to_pixel,
    min_len: props.min_len,
    max_len: props.max_len,
    change_class_on_move: props.change_class_on_move,
    resizer_rf: rf,
  }.set_panel_resizer(props.moving, props.panel_length, props.skip_set_style.unwrap_or(false));

  view! {
    div(ref=rf, class=props.class)
  }
}


// parcels

/// ParcelsResizerProps
/// 
#[derive(Props)]
pub struct ParcelsResizerProps<G: Html, P>
where P: std::cmp::Eq + std::hash::Hash + FromStr + 'static
{
  rf: Option<NodeRef<G>>,
  moving: Option<Signal<bool>>,
  parcel_lengths: Option<Signal<HashMap<P, StyleLength>>>,
  parcel_name: Option<&'static str>,
  class: &'static str,
  is_lateral: bool,
  to_left: bool,
  to_pixel: bool,
  min_len: Option<StyleLength>,
  max_len: Option<StyleLength>,
  change_class_on_move: Option<(Option<&'static str>, &'static str)>,
  skip_set_style: Option<bool>,
}


/// ParcelsResizer
/// * Parcels type
/// * Look at `Resizer`'s `set_parcels_resizer` for more information.
/// 
/// # Props
/// * rf: Option<NodeRef<G>>,
/// * moving: Option<Signal<bool>>,
/// * parcel_lengths: Option<Signal<HashMap<P, StyleLength>>>,
/// * parcel_name: Option<&'static str>,
/// * class: &'static str,
/// * is_lateral: bool,
/// * to_left: bool,
/// * to_pixel: bool,
/// * min_len: Option<StyleLength>,
/// * max_len: Option<StyleLength>,
/// * change_class_on_move: Option<(Option<&'static str>, &'static str)>,
/// * skip_set_style: Option<bool>,
/// 
#[component]
pub fn ParcelsResizer<G: Html, P>(props: ParcelsResizerProps<G, P>) -> View<G>
where P: std::cmp::Eq + std::hash::Hash + FromStr
{
  let rf = props.rf.unwrap_or(create_node_ref());

  Resizer {
    is_lateral: props.is_lateral,
    to_left: props.to_left,
    to_pixel: props.to_pixel,
    min_len: props.min_len,
    max_len: props.max_len,
    change_class_on_move: props.change_class_on_move,
    resizer_rf: rf
  }.set_parcels_resizer(props.moving, props.parcel_lengths, props.parcel_name, props.skip_set_style.unwrap_or(false));

  view! {
    div(ref=rf, class=props.class)
  }
}