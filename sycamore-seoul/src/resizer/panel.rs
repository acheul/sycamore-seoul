use super::*;


pub fn handle_panel_mousemove<G: GenericNode>(
  is_lateral: bool, 
  to_left: bool,
  to_pixel: bool,
  min_len: Option<StyleLength>, 
  max_len: Option<StyleLength>, 
  resizer_rf: NodeRef<G>, 
  e: MouseEvent, 
  skip_set_style: bool
)
-> Option<StyleLength>
{
  // resizer's parent element
  let Some(element) = resizer_parent_element(resizer_rf) else { return None };

  // calculate new length (px)
  let Some((par_len, len, gap)) = get_lengths(&element, e, is_lateral, to_left) else { return None };
  if gap==0. { return None; }
  let (to_len, is_expanding) = (len + gap, gap>0.);

  // limitation check
  // (1)
  if to_len<0. || par_len<to_len { return None; }

  // (2)
  if !(if is_expanding {
    max_len.map(|x| x.max_check(to_len, par_len)).unwrap_or(true)
  } else {
    min_len.map(|x| x.min_check(to_len, par_len)).unwrap_or(true) 
  }) {
    return None;
  }

  // set style
  let style_len = StyleLength::new(to_len, par_len, to_pixel);
  if !skip_set_style {
    style_len.set_style(&element.unchecked_into(), is_lateral);
  }
  Some(style_len)
}