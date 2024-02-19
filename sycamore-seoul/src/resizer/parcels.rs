use super::*;

pub fn handle_parcels_mousemove<G: GenericNode, P>(
  is_lateral: bool, 
  mut to_left: bool,
  to_pixel: bool,
  min_len: Option<StyleLength>, 
  max_len: Option<StyleLength>, 
  resizer_rf: NodeRef<G>,
  e: MouseEvent,
  parcel_name: Option<&str>,
  skip_set_style: bool
) -> Option<HashMap<P, StyleLength>>
where P: std::cmp::Eq + std::hash::Hash + FromStr
{
  // resizer's parent element
  let Some(mut element) = resizer_parent_element(resizer_rf) else { return None };

  // get current length state and mouse movement's gap
  let Some((par_len, mut len, mut gap)) = get_lengths(&element, e, is_lateral, to_left) else { return None };
  if gap==0. { return None; }

  // get expanding element
  // If given element is shrinking, replace it with expanding side's element
  // and flip direction of gap & to_left.
  if gap<0. {
    element = {
      let Some(element_) = (if to_left { element.previous_element_sibling() } else { element.next_element_sibling() }) else { return None };
      element_
    };

    len = get_length(&element, is_lateral);
    gap = -gap;
    to_left = !to_left;
  }

  // calculate new length (expanding)
  let to_len = len + gap;

  // limitation check
  // (1)
  if par_len<to_len { return None; }
  // (2)
  if !max_len.map(|x| x.max_check(to_len, par_len)).unwrap_or(true) {
    return None;
  }

  // siblings: shrinking side (facing) and static side (backwards)

  // (1) shrinking
  // calculate new length of first element which can be shrinked.
  // others' lengths will be just recalucated from their current length.
  // If nothing can be shrinked, return nothing.
  let siblings = get_siblings(to_left, &element);

  let mut any_chg = false;

  let siblings_to_len: Vec<f64> = siblings.iter().map(|sibling| {
    let len = get_length(sibling, is_lateral);
    if !any_chg {
      // calculate shirinked new length
      let to_len = len - gap;
      // limitation check
      if to_len>0. {
        if min_len.map(|x| x.min_check(to_len, par_len)).unwrap_or(true) {
          any_chg = true; // change any_chg
          return to_len;
        }
      }
    }
    len
  }).collect::<Vec<_>>();

  // If nothing can be shrinked, nothing can be changed.
  if !any_chg {
    return None;
  }

  // (2) static siblings
  let siblings2 = get_siblings(!to_left, &element);
  let siblings2_to_len: Vec<f64> = siblings2.iter().map(|x| get_length(x, is_lateral)).collect();


  // set styles & make map
  let mut map = if parcel_name.is_some() { Some(HashMap::new()) } else { None };

  // (1) oneself
  set_parcel_style_and_update_map(&element.unchecked_into(), to_len, par_len, map.as_mut(), is_lateral, to_pixel, parcel_name, skip_set_style);

  // (2) siblings
  siblings.into_iter().zip(siblings_to_len.into_iter())
    .chain(siblings2.into_iter().zip(siblings2_to_len.into_iter()))
    .for_each(|(element, to_len)| {

      set_parcel_style_and_update_map(&element.unchecked_into(), to_len, par_len, map.as_mut(), is_lateral, to_pixel, parcel_name, skip_set_style);
    });

  // return
  map
}



fn get_siblings(to_left: bool, element: &Element) -> Vec<Element> {

  let mut siblings: Vec<Element> = Vec::new();

  if let Some(sibling) = get_sibling(to_left, element) {
    siblings.push(sibling);
  }

  while let Some(Some(sibling)) = siblings.last().map(|x| get_sibling(to_left, x)) {
    siblings.push(sibling);
  }

  siblings
}


fn get_sibling(to_left: bool, element: &Element) -> Option<Element> {
  if to_left { element.previous_element_sibling() } else { element.next_element_sibling() }
}

/// Set parcel's style (percent) and update Map<parcel-name: to_percent>
/// 
/// Update map: collecting parcel_name from element's dataset.
/// 
fn set_parcel_style_and_update_map<P>(
  element: &HtmlElement,
  to_len: f64,
  par_len: f64,
  map: Option<&mut HashMap<P, StyleLength>>,
  is_lateral: bool,
  to_pixel: bool,
  parcel_name: Option<&str>,
  skip_set_style: bool
)
where P: std::cmp::Eq + std::hash::Hash + FromStr
{
  let style_len = StyleLength::new(to_len, par_len, to_pixel);

  // set style (percent)
  if !skip_set_style {
    style_len.set_style(element, is_lateral);
  }

  // update map
  let Some(parcel_name) = parcel_name else { return };
  let Some(map) = map else { return };

  if let Some(value) = element.dataset().get(parcel_name) {
    if let Ok(p) = value.parse() {
      map.insert(p, style_len);
    }
  }
}