use super::*;

pub fn handle_parcels_mousemove<G: GenericNode, P>(
  is_lateral: bool, to_left: bool,
  min_length: Option<StyleLength>, max_length: Option<StyleLength>, 
  resizer_rf: NodeRef<G>,
  e: MouseEvent,
  parcel_name: Option<&str>,
  skip_set_style: bool
) -> Option<HashMap<P, f64>>
where P: std::cmp::Eq + std::hash::Hash + FromStr
{
  let Some(resizer) = resizer_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<Element>()) else { return None };
  let Some(element) = resizer.parent_element() else { return None };

  // get expanding element's values
  let Some((parent_length, to_length, to_percent_length, gap, element, to_left)) = get_parcel_to_lengths(is_lateral, to_left, element, e) else { return None };

  // max length check
  if max_length.map(|x| x.check_max_limit(to_length, to_percent_length)).unwrap_or(true) {

    // (1) shrinking side's siblings
    let siblings = get_siblings(to_left, &element);
    
    let mut res = false;

    let siblings_to_percent_length: Vec<f64> = siblings.iter().map(|sibling| {

      // If any former sibling is to be shrinked, skip others.
      if res {
        get_length(is_lateral, sibling)/parent_length*100.
      } else {
        let (length, to_lengths) = get_sibling_to_lengths(is_lateral, sibling, parent_length, gap);

        // min length check
        if let Some((to_length, to_percent_length)) = to_lengths {
          if min_length.map(|x| x.check_min_limit(to_length, to_percent_length)).unwrap_or(true) {
            res = true;
            return to_percent_length;
          }
        }
        length/parent_length*100.
      }
    }).collect();

    if res {

      // (2) static siblings
      let siblings2 = get_siblings(!to_left, &element);
      let siblings2_to_precent_length: Vec<f64> = siblings2.iter().map(|sibling| get_length(is_lateral, sibling)/parent_length*100.).collect();

      // set style for all the parcels
      let mut map = HashMap::new();

      // the one
      let element: HtmlElement = element.unchecked_into();
      if !skip_set_style {
        set_percent_length_style(is_lateral, &element, to_percent_length);
      }
      parcel_name.map(|name| update_map(&mut map, name, &element, to_percent_length));

      // siblings
      siblings.into_iter().zip(siblings_to_percent_length.into_iter())
        .chain(siblings2.into_iter().zip(siblings2_to_precent_length.into_iter()))
        .for_each(|(element, pl)| {

          let element: HtmlElement = element.unchecked_into();
          if !skip_set_style {
            set_percent_length_style(is_lateral, &element, pl);
          }
          parcel_name.map(|name| update_map(&mut map, name, &element, pl));
        });

      return Some(map);
    }
  }
  None
}


/// Return (parent_length, to_length, to_percent_length, gap, expanding-element)
/// 
/// * If given element is expanding, then keep it.
/// * Or, find the adjoining sibling element which is expanding and then use it instead.
///   * In this case, convert `to_left` and `gap` into opposite direction.
/// 
/// * Return None if gap==0. or parent_length<to_length;
/// 
fn get_parcel_to_lengths(is_lateral: bool, mut to_left: bool, mut element: Element, e: MouseEvent) -> Option<(f64, f64, f64, f64, Element, bool)> {

  let Some((parent_length, mut length, mut gap)) = get_lengths(is_lateral, to_left, &element, e) else { return None };

  if gap==0. { return None; }

  // change (element, gap-direction, to_left) if it's not expanding
  if gap<0. {
    if let Some(_element) = if to_left { element.previous_element_sibling() } else { element.next_element_sibling() } {
      element = _element
    } else {
      return None;
    }
    length = get_length(is_lateral, &element);
    gap = -gap;
    to_left = !to_left;
  }

  // the expanding length
  let to_length = length + gap;
  if parent_length<to_length { return None; }

  Some((parent_length, to_length, to_length/parent_length*100., gap, element, to_left))
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


/// Return given sibling's (current length and Option<(to_length, to_percent_length)>)
/// 
/// * If the sibling's length can be adjusted by substracting `gap` from it, return it in the option.
///   * Return None if to_length<0. or parent_length<to_length;
/// 
fn get_sibling_to_lengths(is_lateral: bool, sibling: &Element, parent_length: f64, gap: f64) -> (f64, Option<(f64, f64)>) {
  
  let length = get_length(is_lateral, sibling);

  // substracting gap from length is the `to_length` of the sibling.
  let to_length = length - gap;

  let to_lengths: Option<(f64, f64)> = 
    if to_length<0. || parent_length<to_length { None }
    else { Some((to_length, to_length/parent_length*100.)) };

  (length, to_lengths)
}


/// Update map with <parcel_name: pl>, collecting parcel_name from element's dataset.
/// Hereby the pl is percent length which the element would have.
fn update_map<P>(map: &mut HashMap<P, f64>, parcel_name: &str, elem: &HtmlElement, pl: f64)
where P: std::cmp::Eq + std::hash::Hash + FromStr
{
  if let Some(value) = elem.dataset().get(parcel_name) {
    if let Ok(p) = value.parse() {
      map.insert(p, pl);
    }
  }
}