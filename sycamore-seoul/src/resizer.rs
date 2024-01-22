mod panel;
mod parcels;
mod stylelength;

use panel::*;
use parcels::*;
pub use stylelength::*;

use super::*;


/// Resizer's event handler setter
/// 
/// There are two types of resizer:
///   1. Panel: resize one panel relatively to other part under common container.
///   ```
///     /* Structure of elements:
///     > common container { display: flex; }
///       > panel { position: relative }
///         > resizer { style: absolute }
///       > other { flex-grow: 1 } */
///   ```
///   2. Parcels: resize each parcel relatively to other parcel(s) under common container.
///   ```
///     /* Structure of elements:
///     > common container { display: flex; }
///       > other parcels { flex-grow: 1 }
///       > parcel { flex-grow: 1; position: relative; }
///         > resizer { style: absolute }
///       > other parcels { flex-grow: 1 } */
///   ```
/// 
///   * This setter work on the 'resizer' element via its NodeRef.
///     * The 'resizer' element is supposed to be located under the resize-able super element, which is Panel-like or Parcel-like.
///     * Set the 'resizer's position in absolute to the super's.
/// 
///
/// # Fields
///   * is_panel(bool): is Panel or Parcels?
///   * is_lateral(bool): is resizing in lateral direction or vertical direction? ([lateral/vertical])
///   * to_left(bool): is the "resizer" element located at the [left/top] or [right/bottom] side of the super element?
///   * min_length limitation(StyleLength)
///   * max_length limititation(StyleLength)
///   * replace_class: on moving, convert old to new, and vice versa on stop.
///   * resizer_rf: the NodeRef which this setter attached to.
/// 
/// # Use
///   * Build the struct in raw format or via `new()`, and then call `set_event_handler()` function right away.
///   * In `set_event_handler`, let it know if you want to set the adjusted percent_length to a sycamore Signal<f64> and trigger alarming Signal<bool>.
///     * Recommend to use Signal<f64> for Panel case and Signal<bool> for Parcels case.
#[derive(Debug, Clone)]
pub struct Resizer<G: GenericNode> {
  pub is_panel: bool,
  pub is_lateral: bool, // [lateral/horizontal]
  pub to_left: bool, // [to_left/top] <-> [right/bottom]
  pub min_length: Option<StyleLength>,
  pub max_length: Option<StyleLength>,
  pub replace_class_on_move: (&'static str, &'static str),
  pub resizer_rf: NodeRef<G>,
}

impl<G: GenericNode> Resizer<G> {

  /// Build a new struct
  pub fn new(is_panel: bool, is_lateral: bool, to_left: bool, min_length: Option<StyleLength>, max_length: Option<StyleLength>, replace_class_on_move: (&'static str, &'static str), resizer_rf: NodeRef<G>) -> Self {
    Self { is_panel, is_lateral, to_left, min_length, max_length, replace_class_on_move, resizer_rf }
  }

  /// Set event handlers
  /// 
  /// 1. mousedown handler to the 'resizer' element
  ///   * on mousedown, attach mousemove & mouseup handlers to document.
  ///   * convert class
  /// 2. mousemove handler to the document
  ///   * handler resizing
  /// 3. mouseup handler to the document
  ///   * convert class
  ///   * remove mousemove & mouseup handlers from document.
  /// 
  /// # Args
  ///   * percent_length_signal: If set, update the signal with the adjusted percent_length of the resizer's super element.
  ///   * alarm_signal: If set, trigger the signal when there was any change of the length.
  /// 
  /// # Resize-able or not?
  ///   * Conduct resizing when the adjusted length ("to-length") is
  ///     (1) between 0 ~ parent's length and
  ///     (2) between min-limit ~ max-limit;
  ///   * Resizing is implemented by setting the css width/height style (in percent type length).
  /// 
  pub fn set_event_handler(self, percent_length_signal: Option<Signal<f64>>, alarm_signal: Option<Signal<bool>>) {

    // Must be inside the on_mount scope
    on_mount(move || {

      let Self { is_panel, is_lateral, to_left, min_length, max_length, replace_class_on_move, resizer_rf } = self;
      let (old, new) = replace_class_on_move;
      
      let adjusted_percent_length = Box::into_raw(Box::from(0.));
      let any_change = Box::into_raw(Box::from(false));
      
      // each closures

      // mousemove
      let cb_mousemove = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {
        let pl = if is_panel {
          handle_panel_mousemove(is_lateral, to_left,  min_length, max_length, resizer_rf, e)
        } else {
          handle_parcels_mousemove(is_lateral, to_left, min_length, max_length, resizer_rf, e)
        };

        // set adjusted_percent_length
        unsafe {
          if let Some(pl) = pl {
            *adjusted_percent_length = pl;
            *any_change = true;
          }
        }
      });

      let cb_mousemove = Box::into_raw(Box::new(cb_mousemove));
    

      // mouseup
      let cb_mouseup: *mut Closure<dyn FnMut(MouseEvent)> = Box::into_raw(Box::new(Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {})));

      unsafe {
        *cb_mouseup = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
          
          // convert class
          replace_class(resizer_rf, old, new, false);
          
          // set signals
          if *any_change {
            if let Some(signal) = percent_length_signal {
              signal.set(*adjusted_percent_length);
            }
            if let Some(signal) = alarm_signal {
              signal.set(true);
            }
            *any_change = false;
          }
          
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
        // convert class
        replace_class(resizer_rf, old, new, true);
      });


      let target = resizer_rf.get::<DomNode>().unchecked_into::<EventTarget>();
      target.add_event_listener_with_callback("mousedown", cb_mousedown.as_ref().unchecked_ref()).unwrap_throw();
      
      on_cleanup(move || {
        target.remove_event_listener_with_callback("mousedown", cb_mousedown.as_ref().unchecked_ref()).unwrap_throw();
        unsafe {
          let _ = Box::from_raw(cb_mousemove);
          let _ = Box::from_raw(cb_mouseup);
          let _ = Box::from_raw(adjusted_percent_length);
          let _ = Box::from_raw(any_change);
        }
      });
    });
  }
}


// helpers

/// Set css style of the percent_length at the element.
/// Return the percent_length.
fn set_percent_length_style(is_lateral: bool, element: Element, to_percent_length: f64) -> f64 {

  let property = if is_lateral { "width" } else { "height" };

  let element: HtmlElement = element.unchecked_into();
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