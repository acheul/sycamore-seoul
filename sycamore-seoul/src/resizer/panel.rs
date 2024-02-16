use super::*;


pub fn handle_panel_mousemove<G: GenericNode>(is_lateral: bool, to_left: bool, min_length: Option<StyleLength>, max_length: Option<StyleLength>, resizer_rf: NodeRef<G>, e: MouseEvent, skip_set_style: bool)
-> Option<f64>
{
  let Some(resizer) = resizer_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<Element>()) else { return None };
  let Some(element) = resizer.parent_element() else { return None };
  let Some((to_length, to_percent_length, is_expanding)) = get_panel_to_lengths(is_lateral, to_left, &element, e) else { return None };

  let limit_check = if is_expanding {
    max_length.map(|x| x.check_max_limit(to_length, to_percent_length)).unwrap_or(true)
  } else {
    min_length.map(|x| x.check_min_limit(to_length, to_percent_length)).unwrap_or(true)
  };

  if limit_check{
    if skip_set_style {
      Some(to_percent_length) 
    } else {
      Some(set_percent_length_style(is_lateral, &element.unchecked_into(), to_percent_length))
    }
  } else {
    None
  }
}


/// Return (to_length, to_percent_length, is-expanding).
/// Return None if to_length<0. or parent_length<length;
/// 
fn get_panel_to_lengths(is_lateral: bool, to_left: bool, element: &Element, e: MouseEvent) -> Option<(f64, f64, bool)> {
  
  let Some((parent_length, length, gap)) = get_lengths(is_lateral, to_left, element, e) else { return None };
  let to_length = length + gap;

  if to_length<0. || parent_length<to_length { return None; }

  Some((to_length, to_length/parent_length*100., to_length>length))
}