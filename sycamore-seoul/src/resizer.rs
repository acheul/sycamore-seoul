use crate::*;

mod panel;
use panel::*;

mod parcels;
use parcels::*;

mod comps;
pub use comps::*;


/// Resizer's event handler setter
/// 
/// # Description
/// 
/// There are two types of resizer:
/// 
/// 1. Panel-Resizer
///   * The resizer changes "length(width/height)[px/%]" of the panel, which is its parent element.
///   * The panel's own parent, say "wrap", would have { display: flex; } and its siblings would have { flex-grow: 1; }
///   
///   ```
///     /* example structure*/
///     // wrap { display: flex; }
///     //   panel { position: relative; }
///     //     panel-resizer { position: absolute; }
///     //   other { flex-grow: 1; }
///   ```
/// 
/// 2. Parcel-Resizer
///   * The resizer changes "length(width/height)[px/%]" of "parcel", which is its parent element.
///   * Each parcel is supposed to be inside "wrap" and there are other sibling parcels.
///   * Resizing one parcel simultaneously updates other parcels' lengths.
///   
///   ```
///     /* example structure*/
///     // wrap(parcels) { display: flex; }
///     //  ..parcel
///     //  each parcel { position: relative; flex-grow: 1; }
///     //    parcel-resizer { position: absolute; }
///     //  parcel..
///   ```
/// 
/// # Use
/// Build the struct in raw format or via method `new()`, and then call either `set_panel_resizer` or `set_parcels_resizer`.
/// * Using `set_panel_resizer` and `set_parcels_resizer`,
///   pass optional arguments to let it know if you want to have some Signals convey infos like is-moving(bool) or newly-updated-length(f64/HashMap<_, f64>)
/// 
/// # Sycamore Component
/// There are Sycamore native component functions wrapping all the logics.
/// * PanelResizer
/// * ParcelsResizer
/// 
/// # Fields
/// * is_lateral(bool): is resizing in lateral direction or vertical direction? ([lateral/vertical])
/// * to_left(bool): is the "resizer" element located at the [left/top] or [right/bottom] side of the super element?
/// * to_pixel(bool): set style in pixel or percent?
/// * min_len: min limitation(Look at `StyleLength`)
/// * max_len: max limitation(StyleLength)
/// * change_class_on_move: on moving, convert class from old to new, and vice versa on stop. (Look at `ChangeClass`)
/// * resizer_rf: the NodeRef of actual resizer element.
/// 
/// # How it works
/// * All jobs are conducted on the resizer element, which is captured by NodeRef of Sycamore.
/// * Three event handlers jointly work: mousedown, mousemove, and mouseup
///   1. On MouseDown of resizer element:
///      * Add MouseMove and MouseUp event handlers to document.
///      * Do some works to notify initiation of resize, such as to change class of element.
///   2. MouseMove of document:
///      * Handle actual resizing:
///        * capture mouse movement, calculate new length(s), conduct limitation check, and set style of new length(s).
///      * Do some works to pass new length's info via Signal of Sycamore.
///   3. MouseUp of document:
///      * Remove MouseMove and MoueUp event handlers from document. (Raw pointers are used for this purpose. They are handled in secure way.)
///      * Do works to notify end of resize.
/// 
/// # Style
///   * The resizer element is supposed to be {"position: absolute"} and its parent is NOT to be {"position: static"}
///   * It would be nice to make resizer have {"z-index: (some big value)"}, especially when resizer's width spans over parent's border.
///     * In this case, if the parent is NOT {"overflow: visible"}, the resizer's visiblity would be contained inside parent's border.
/// 
/// # Limitation check
///   * On Mousemove, newly calculated length goes through limitaion check:
///     * (1) Is it between [0 ~ wrapping element's length]?
///       - (The wrapping element is parent of parent of resizer element).
///     * (2) Is it between given arguments of min_len and max_len?
///     * For parcels type, all parcels affected go under this check.
///   * If check fails, nothing changes.
/// 
/// # About Overflowing of Wrap Element
/// * The "wrap" element might overflow. And it makes resizing logic very ambiguous.
///   - Let's say there are two parcels, A and B.
///     - On moving a resizer between A and B to B's side, is it expanding A by shrinking B or by expanding the wraping element?
///     - If the movement is expanding the wrapping element, then by what movement can we shrink B's size?
///     - Hard to be deterministic.
/// * Thus, it's bettet not to allow resizing when wrapping element is overflowing(scrollable).
/// 
#[derive(Debug, Clone)]
pub struct Resizer<G: GenericNode> {
  pub is_lateral: bool,
  pub to_left: bool,
  pub to_pixel: bool,
  pub min_len: Option<StyleLength>,
  pub max_len: Option<StyleLength>,
  pub change_class_on_move: Option<(Option<&'static str>, &'static str)>,
  pub resizer_rf: NodeRef<G>,
}

impl<G: GenericNode> Resizer<G> {

  /// Build new struct
  /// 
  pub fn new(is_lateral: bool, to_left: bool, to_pixel: bool, min_len: Option<StyleLength>, max_len: Option<StyleLength>, change_class_on_move: Option<(Option<&'static str>, &'static str)>, resizer_rf: NodeRef<G>) -> Self {
    Self { is_lateral, to_left, to_pixel, min_len, max_len, change_class_on_move, resizer_rf }
  }

  /// Set panel-type resizer's event handlers
  /// 
  /// # Args (signals)
  /// * moving: update if is it moving(resizing) or not
  /// * panel_length: update the adjusted panel length(px) at every moving step.
  /// * skip_set_style(bool): If it's true, css property wouldn't be changed: only given signals would be updated.
  /// 
  pub fn set_panel_resizer(self, 
    moving: Option<Signal<bool>>, 
    panel_length: Option<Signal<StyleLength>>, 
    skip_set_style: bool
  ) {

    // expand self
    let Self { is_lateral, to_left, to_pixel, min_len, max_len, change_class_on_move, resizer_rf } = self;

    // mousemove closure
    let cb_mousemove = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
      
      if let Some(style_len) = handle_panel_mousemove(is_lateral, to_left, to_pixel, min_len, max_len, resizer_rf, e, skip_set_style) {
        if let Some(signal) = panel_length {
          signal.set(style_len);
        }
      }
    });

    let cb_mousemove = Box::into_raw(Box::new(cb_mousemove));

    // set each event handlers
    Self::set_event_handlers(resizer_rf, change_class_on_move, cb_mousemove, moving);
  }


  /// Set parcels-type resizer's event handlers
  /// 
  /// # Args (signals)
  /// * moving: update if is it moving(resizing) or not
  /// * parcel_lengths: hashbrown::HashMap collected from <parcel-element's dataset value: adjusted percent length>
  /// * parcel_name: the name of parcel-element's dataset to identify each parcel.
  /// * skip_set_style(bool): If it's true, css property wouldn't be changed: only given signals would be updated.
  /// 
  pub fn set_parcels_resizer<P>(self, 
    moving: Option<Signal<bool>>, 
    parcel_lengths: Option<Signal<HashMap<P, StyleLength>>>, 
    parcel_name: Option<&'static str>, 
    skip_set_style: bool
  )
  where P: std::cmp::Eq + std::hash::Hash + FromStr
  {
    // expand self
    let Self { is_lateral, to_left, to_pixel, min_len, max_len, change_class_on_move, resizer_rf } = self;

    // mousemove closure
    let cb_mousemove = Closure::<dyn FnMut(_)>::new(move |e: web_sys::MouseEvent| {

      if let Some(map) = handle_parcels_mousemove(is_lateral, to_left, to_pixel, min_len, max_len, resizer_rf, e, parcel_name, skip_set_style) {
        if let Some(signal) = parcel_lengths {
          signal.update(|x| x.extend(map)); // use signal.update() instead of signal.set();
        }
      }
    });

    let cb_mousemove = Box::into_raw(Box::new(cb_mousemove));

    // set each event handlers
    Self::set_event_handlers(resizer_rf, change_class_on_move, cb_mousemove, moving);
  }


  /// Retreive struct fields and each type's cb_mousemove, then handle other common parts.
  /// 
  fn set_event_handlers(resizer_rf: NodeRef<G>, change_class_on_move: Option<(Option<&'static str>, &'static str)>, cb_mousemove: *mut Closure<dyn FnMut(MouseEvent)>, moving: Option<Signal<bool>>) {

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

/// resizer's parent element
fn resizer_parent_element<G: GenericNode>(resizer_rf: NodeRef<G>) -> Option<Element> {
  resizer_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<Element>()).map(|x| x.parent_element()).flatten()
}

/// Element's Scroll length
/// * Use this to get wrapping element's length 
/// 
fn scroll_length(element: &Element, is_lateral: bool) -> f64 {

  let v = if is_lateral {
    element.scroll_width()
  } else {
    element.scroll_height()
  };
  v as f64
}

/// get client length of given element
/// 
fn get_length(element: &Element, is_lateral: bool) -> f64 {
  let rect = element.get_bounding_client_rect();
  if is_lateral { rect.width() } else { rect.height() }
}

/// get (parent_length, length, gap) of given element
/// 
fn get_lengths(
  element: &Element,
  e: MouseEvent,
  is_lateral: bool, 
  to_left: bool
) -> Option<(f64, f64, f64)> {

  // wrapper's scroll length
  let Some(parent) = element.parent_element() else { return None };
  let par_len = scroll_length(&parent, is_lateral);

  let rect = element.get_bounding_client_rect();

  Some(if is_lateral {
    let cur = e.client_x() as f64;
    let gap = if to_left { rect.left()-cur } else { cur-rect.right() };
    (par_len, rect.width(), gap)
  } else {
    let cur = e.client_y() as f64;
    let gap = if to_left { rect.top()-cur } else { cur-rect.bottom() };
    (par_len, rect.height(), gap)
  })
}