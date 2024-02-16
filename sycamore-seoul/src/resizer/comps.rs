use crate::*;

// resizer components

#[derive(Props)]
pub struct PanelResizerProps{
  moving: Option<Signal<bool>>,
  panel_length: Option<Signal<f64>>,
  class: &'static str,
  is_lateral: bool,
  to_left: bool,
  min_length: Option<StyleLength>,
  max_length: Option<StyleLength>,
  change_class_on_move: Option<(Option<&'static str>, &'static str)>,
  skip_set_style: Option<bool>,
}

/// PanelResizerComponent
/// * Panel type
/// * check `Resizer` for more information.
/// 
/// # Props
/// * moving: Option<Signal<bool>>,
/// * panel_length: Option<Signal<f64>>,
/// * class: &'static str,
/// * is_lateral: bool,
/// * to_left: bool,
/// * min_length: Option<StyleLength>,
/// * max_length: Option<StyleLength>,
/// * change_class_on_move: Option<(Option<&'static str>, &'static str)>,
/// * skip_set_style: Option<bool>,
#[component]
pub fn PanelResizerComponent<G: Html>(props: PanelResizerProps) -> View<G> {

  let rf = create_node_ref();

  Resizer {
    is_lateral: props.is_lateral,
    to_left: props.to_left,
    min_length: props.min_length,
    max_length: props.max_length,
    change_class_on_move: props.change_class_on_move,
    resizer_rf: rf,
  }.set_panel_resizer(props.moving, props.panel_length, props.skip_set_style.unwrap_or(false));

  view! {
    div(ref=rf, class=props.class)
  }
}


#[derive(Props)]
pub struct ParcelsResizerProps<G: Html, P>
where P: std::cmp::Eq + std::hash::Hash + FromStr + 'static
{
  rf: Option<NodeRef<G>>,
  moving: Option<Signal<bool>>,
  parcel_lengths: Option<Signal<HashMap<P, f64>>>,
  parcel_name: Option<&'static str>,
  class: &'static str,
  is_lateral: bool,
  to_left: bool,
  min_length: Option<StyleLength>,
  max_length: Option<StyleLength>,
  change_class_on_move: Option<(Option<&'static str>, &'static str)>,
  skip_set_style: Option<bool>,
}


/// ParcelsResizerComponent
/// * Parcels type
/// * check `Resizer` for more information.
/// 
/// # Props
/// * rf: Option<NodeRef<G>>,
/// * moving: Option<Signal<bool>>,
/// * parcel_lengths: Option<Signal<HashMap<P, f64>>>,
/// * parcel_name: Option<&'static str>,
/// * class: &'static str,
/// * is_lateral: bool,
/// * to_left: bool,
/// * min_length: Option<StyleLength>,
/// * max_length: Option<StyleLength>,
/// * change_class_on_move: Option<(Option<&'static str>, &'static str)>,
/// * skip_set_style: Option<bool>,
/// 
#[component]
pub fn ParcelsResizerComponent<G: Html, P>(props: ParcelsResizerProps<G, P>) -> View<G>
where P: std::cmp::Eq + std::hash::Hash + FromStr
{
  let rf = props.rf.unwrap_or(create_node_ref());

  Resizer {
    is_lateral: props.is_lateral,
    to_left: props.to_left,
    min_length: props.min_length,
    max_length: props.max_length,
    change_class_on_move: props.change_class_on_move,
    resizer_rf: rf
  }.set_parcels_resizer(props.moving, props.parcel_lengths, props.parcel_name, props.skip_set_style.unwrap_or(false));

  view! {
    div(ref=rf, class=props.class)
  }
}
