mod panel;
mod parcels;
mod stylelength;
mod comps;

use panel::*;
use parcels::*;
pub use stylelength::*;
pub use comps::*;

use super::*;


/// Resizer's event handler setter
/// 
/// There are two types of resizer:
///  1. Panel: resize one panel relatively to other part under common container.
///
///   * Structure of elements:
///       * common container { display: flex; }
///         * panel { position: relative }
///           * resizer { style: absolute }
///         * other { flex-grow: 1 }
/// 
/// 2. Parcels: resize each parcel relatively to other parcel(s) under common container.
///   
///   * Structure of elements:
///     * common container { display: flex; }
///       * other parcels { flex-grow: 1 }
///       * parcel { flex-grow: 1; position: relative; }
///         * resizer { style: absolute }
///       * other parcels { flex-grow: 1 }
///
///
/// # How it works
/// This setter work on the 'resizer' element via its NodeRef.
///   * The 'resizer' element is supposed to be located under the resize-able super element, which is either Panel-like or Parcel-like.
///   * Set the 'resizer's style property `{position: absolute}`.
/// 
/// Three event handlers jointly work: mousedown, mousemove, and mouseup
/// 1. mousedown handler to the 'resizer' element
///     * on mousedown, add mousemove & mouseup handlers to document.
///     * convert class
/// 2. mousemove handler to the document
///     * handle actual resizing
/// 3. mouseup handler to the document
///     * convert class
///     * remove mousemove & mouseup handlers from document.
/// 
/// # Resize-able or not?
/// * Conduct resizing when the adjusted length ("to-length") is
///   * (1) between [0 ~ parent's length] and
///   * (2) between [min-limit ~ max-limit];
/// * Resizing is implemented by setting the css width/height style (in percent(%) format length).
/// 
/// # Fields
/// * is_lateral(bool): is resizing in lateral direction or vertical direction? ([lateral/vertical])
/// * to_left(bool): is the "resizer" element located at the [left/top] or [right/bottom] side of the super element?
/// * min_length limitation(StyleLength)
/// * max_length limititation(StyleLength)
/// * replace_class: on moving, convert class from old to new, and vice versa on stop.
/// * resizer_rf: the NodeRef which this setter attached to.
/// 
/// # Use
/// * Build the struct in raw format or via `new()`, and then call `set_panel_resizer` or `set_parcels_resizer` right away.
/// * In `set_panel/parcels_resizer`, let it know if you want to have some signals convey infos like is-moving(bool) or adjusted percent lengths(f64).
///
#[derive(Debug, Clone)]
pub struct Resizer<G: GenericNode> {
  pub is_lateral: bool, // [lateral/horizontal]
  pub to_left: bool, // [to_left/top] <-> [right/bottom]
  pub min_length: Option<StyleLength>,
  pub max_length: Option<StyleLength>,
  pub change_class_on_move: Option<(Option<&'static str>, &'static str)>,
  pub resizer_rf: NodeRef<G>,
}

impl<G: GenericNode> Resizer<G> {

  /// Build a new struct
  /// 
  pub fn new(is_lateral: bool, to_left: bool, min_length: Option<StyleLength>, max_length: Option<StyleLength>, change_class_on_move: Option<(Option<&'static str>, &'static str)>, resizer_rf: NodeRef<G>) -> Self {
    Self { is_lateral, to_left, min_length, max_length, change_class_on_move, resizer_rf }
  }

  /// Set panel-type event handlers
  /// 
  /// # Args (signals)
  /// * moving: update if is it moving(resizing) or not
  /// * panel_length: update the adjusted panel length at every moving step.
  /// * skip_set_style(bool): If it's true, css property wouldn't be changed: only given signals would be updated.
  /// 
  pub fn set_panel_resizer(self, moving: Option<Signal<bool>>, panel_length: Option<Signal<f64>>, skip_set_style: bool) {

    // extend self
    let Self { is_lateral, to_left, min_length, max_length, change_class_on_move, resizer_rf } = self;

    // mousemove closure
    let cb_mousemove = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
      if let Some(pl) = handle_panel_mousemove(is_lateral, to_left,  min_length, max_length, resizer_rf, e, skip_set_style) {
        if let Some(signal) = panel_length {
          signal.set(pl);
        }
      }
    });

    let cb_mousemove = Box::into_raw(Box::new(cb_mousemove));

    // set each event handlers
    Self::_set_event_handlers(resizer_rf, change_class_on_move, cb_mousemove, moving);
  }

  /// Set parcels-type event handlers
  /// 
  /// # Args (signals)
  /// * moving: update if is it moving(resizing) or not
  /// * parcel_lengths: hashbrown::HashMap collected from <parcel-element's dataset value: adjusted percent length>
  /// * parcel_name: the name of parcel-element's dataset to identify it.
  /// * skip_set_style(bool): If it's true, css property wouldn't be changed: only given signals would be updated.
  /// 
  pub fn set_parcels_resizer<P>(self, moving: Option<Signal<bool>>, parcel_lengths: Option<Signal<HashMap<P, f64>>>, parcel_name: Option<&'static str>, skip_set_style: bool)
  where P: std::cmp::Eq + std::hash::Hash + FromStr
  {
    // extend self
    let Self { is_lateral, to_left, min_length, max_length, change_class_on_move, resizer_rf } = self;

    // mousemove closure
    let cb_mousemove = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {

      if let Some(map) = handle_parcels_mousemove(is_lateral, to_left,  min_length, max_length, resizer_rf, e, parcel_name, skip_set_style) {
        if let Some(signal) = parcel_lengths {
          signal.update(|x| x.extend(map)); // use signal.update() instead of signal.set();
        }
      }
    });

    let cb_mousemove = Box::into_raw(Box::new(cb_mousemove));

    // set each event handlers
    Self::_set_event_handlers(resizer_rf, change_class_on_move, cb_mousemove, moving);
  }

  /// Retreive struct fields and each type's cb_mousemove, then handle other common parts.
  /// 
  fn _set_event_handlers(resizer_rf: NodeRef<G>, change_class_on_move: Option<(Option<&'static str>, &'static str)>, cb_mousemove: *mut Closure<dyn FnMut(MouseEvent)>, moving: Option<Signal<bool>>) {

    // Must be inside the on_mount scope
    on_mount(move || {

      // mouseup
      let cb_mouseup: *mut Closure<dyn FnMut(MouseEvent)> = Box::into_raw(Box::new(Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {})));

      unsafe {
        *cb_mouseup = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
          
          // convert class & set moving false
          if let Some((old, new)) = change_class_on_move {
            ChangeClass::replace(resizer_rf, old, new, false);
          }
          moving.map(|x| x.set(false));
          
          let document = gloo_utils::document();
          document.remove_event_listener_with_callback("mousemove", (*cb_mousemove).as_ref().unchecked_ref()).unwrap_throw();
          document.remove_event_listener_with_callback("mouseup", (*cb_mouseup).as_ref().unchecked_ref()).unwrap_throw();
        });
      }
      
      // mousedown
      let cb_mousedown = Closure::<dyn FnMut(_)>::new(move |_: MouseEvent| {
        unsafe {
          let document = gloo_utils::document();
          document.add_event_listener_with_callback("mousemove", (*cb_mousemove).as_ref().unchecked_ref()).unwrap_throw();
          document.add_event_listener_with_callback("mouseup", (*cb_mouseup).as_ref().unchecked_ref()).unwrap_throw();
        }
        // convert class & set moving true
        if let Some((old, new)) = change_class_on_move {
          ChangeClass::replace(resizer_rf, old, new, true);
        }
        moving.map(|x| x.set(true));
      });

      // set mousedown handler
      let target = resizer_rf.get::<DomNode>().unchecked_into::<EventTarget>();
      target.add_event_listener_with_callback("mousedown", cb_mousedown.as_ref().unchecked_ref()).unwrap_throw();
      
      // consume raw pointers on clean-up
      on_cleanup(move || {
        target.remove_event_listener_with_callback("mousedown", cb_mousedown.as_ref().unchecked_ref()).unwrap_throw();
        unsafe {
          let _ = Box::from_raw(cb_mousemove);
          let _ = Box::from_raw(cb_mouseup);
        }
      });
    });
  }
}


// helpers

/// Set css style of the percent_length at the element.
/// Return the percent_length.
pub fn set_percent_length_style(is_lateral: bool, element: &HtmlElement, to_percent_length: f64) -> f64 {

  let property = if is_lateral { "width" } else { "height" };
  element.style().set_property(property, &format!("{to_percent_length:.2}%")).unwrap_throw();
  to_percent_length
}


/// Return (parent_length, length, gap)
fn get_lengths(is_lateral: bool, to_left: bool, element: &Element, e: MouseEvent) -> Option<(f64, f64, f64)> {

  let Some(parent) = element.parent_element() else { return None };
  let parent_rect = parent.get_bounding_client_rect();
  let rect = element.get_bounding_client_rect();

  Some(if is_lateral {
    let cur = e.client_x() as f64;
    let gap = if to_left { rect.left()-cur } else { cur-rect.right() };
    (parent_rect.width(), rect.width(), gap)
  } else {
    let cur = e.client_y() as f64;
    let gap = if to_left { rect.top()-cur } else { cur-rect.bottom() };
    (parent_rect.height(), rect.height(), gap)
  })
}

fn get_length(is_lateral: bool, element: &Element) -> f64 {
  let rect = element.get_bounding_client_rect();
  if is_lateral { rect.width() } else { rect.height() }
}