use crate::*;

/// ScrollBar
/// 
/// * Structure of elements:
///   * parent (which will be overflowed and scrolled) {position: relative}
///     * scroll-bar {position: absolute}
/// 
/// # Functionals
///   * (a)update bar's style (width/height and left/top)
///   * (b)manually scroll parent element
/// 
/// # How to work:
/// 1. on_mount:
///    * (a)update bar's style
/// 
/// 2. a signal to listen -> (a)update bar's style.
/// 
/// 3. listens to **scroll event** at parent
///    * (a)update bar's style
/// 
/// 4. listens to **wheel event** at parent
///    * take orthogonal movement if configured to do so.
///      * (b)manually scroll parent element
/// 
/// 5. listens to **mousedown event** at the bar.
///    * set mousemove & mouseup event to document.
///    * while moving, (b)manually scroll parent element
/// 
/// # Fields
/// * bar_rf: NodeRef of the bar element
/// * is_lateral: scroll direction is lateral(x) or vertical(y)?
/// * take_orthogonal: would consume orthogonal movement on wheel event?
/// * min_length: min length of the bar
/// * is_scrollable: is the parent element scrollable? (scroll-length>client-length)
/// 
pub struct ScrollBar<G: GenericNode> {
  bar_rf: NodeRef<G>,
  is_lateral: bool,
  take_orthogonal: bool,
  min_length: Option<StyleLength>,
  is_scrollable: Option<Signal<bool>>,
}

impl<G: GenericNode> ScrollBar<G> {

  pub fn new(bar_rf: NodeRef<G>, is_lateral: bool, take_orthogonal: bool, min_length: Option<StyleLength>, is_scrollable: Option<Signal<bool>>) -> Self {
    Self { bar_rf, is_lateral, take_orthogonal, min_length, is_scrollable }
  }

  pub fn set_scroll_bar<T>(self, signal: Option<ReadSignal<T>>) {
    let Self { bar_rf, is_lateral, take_orthogonal, min_length, is_scrollable } = self;

    on_mount(move || {

      // init bar's style
      if let Some(bar) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<HtmlElement>()) {
        Self::update_bar_style(is_lateral, &bar, min_length, is_scrollable);
      }

      // listen a signal
      if let Some(signal) = signal {
        create_effect(on(signal, move || {
          if let Some(bar) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<HtmlElement>()) {
            Self::update_bar_style(is_lateral, &bar, min_length, is_scrollable);
          }
        }));
      }

      // parent

      let cb_scroll = Closure::<dyn FnMut(_)>::new(move |_: Event| {
        if let Some(bar) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<HtmlElement>()) {
          Self::update_bar_style(is_lateral, &bar, min_length, is_scrollable);
        }
      });

      let cb_wheel = Closure::<dyn FnMut(_)>::new(move |e: WheelEvent| {
        
        let delta = if is_lateral { e.delta_y() } else { e.delta_x() };
        if delta != 0. {
          if let Some(parent) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<Node>().parent_element()).flatten() {
            Self::update_scroll(&parent, is_lateral, delta);
          }
        }
      });

      // bar

      let x = None::<f64>;
      let x = Box::into_raw(Box::new(x));

      let cb_mousemove = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {

        unsafe {
          let x1 = if is_lateral { e.client_x() } else { e.client_y() } as f64;
          if let Some(x0) = (*x).replace(x1) {
            let delta = x1-x0;

            if let Some(parent) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<Node>().parent_element()).flatten() {
              
              let (w, sw) = if is_lateral {
                (parent.client_width(), parent.scroll_width())
              } else {
                (parent.client_height(), parent.scroll_height())
              };
              let (w, sw) = (w as f64, sw as f64);
              let r = w/sw; // w:sw ratio

              // divide by w/sw ratio.
              let delta = delta/r;
              
              Self::update_scroll(&parent, is_lateral, delta);
            }
          }
        }
      });

      let cb_mousemove = Box::into_raw(Box::new(cb_mousemove));

      let cb_mouseup: *mut Closure<dyn FnMut(MouseEvent)> = Box::into_raw(Box::new(Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {})));

      unsafe {
        *cb_mouseup = Closure::<dyn FnMut(_)>::new(move |_: web_sys::MouseEvent| {
          
          let _ = (*x).take();
          
          let document = gloo_utils::document();
          document.remove_event_listener_with_callback("mousemove", (*cb_mousemove).as_ref().unchecked_ref()).unwrap_throw();
          document.remove_event_listener_with_callback("mouseup", (*cb_mouseup).as_ref().unchecked_ref()).unwrap_throw();
        });
      }
      
      let cb_mousedown = Closure::<dyn FnMut(_)>::new(move |e: MouseEvent| {
        unsafe {
          let document = gloo_utils::document();
          document.add_event_listener_with_callback("mousemove", (*cb_mousemove).as_ref().unchecked_ref()).unwrap_throw();
          document.add_event_listener_with_callback("mouseup", (*cb_mouseup).as_ref().unchecked_ref()).unwrap_throw();
        
          let x1 = if is_lateral { e.client_x() } else { e.client_y() } as f64;
          let _ = (*x).replace(x1);
        }
      });

      // set listeners
      if let Some(parent) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<Node>().parent_element()).flatten() {
        parent.add_event_listener_with_callback("scroll", cb_scroll.as_ref().unchecked_ref()).unwrap_throw();
        if take_orthogonal {
          parent.add_event_listener_with_callback_and_add_event_listener_options("wheel", cb_wheel.as_ref().unchecked_ref(), AddEventListenerOptions::new().passive(true)).unwrap_throw();
        }
      }      
      if let Some(bar) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<EventTarget>()) {
        bar.add_event_listener_with_callback("mousedown", cb_mousedown.as_ref().unchecked_ref()).unwrap_throw();
      }

      on_cleanup(move || {
        if let Some(parent) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<Node>().parent_element()).flatten() {
          parent.remove_event_listener_with_callback("scroll", cb_scroll.as_ref().unchecked_ref()).unwrap_throw();
          if take_orthogonal {
            parent.remove_event_listener_with_callback("wheel", cb_wheel.as_ref().unchecked_ref()).unwrap_throw();
          }
        }   
        if let Some(bar) = bar_rf.try_get::<DomNode>().map(|x| x.unchecked_into::<EventTarget>()) {
          bar.remove_event_listener_with_callback("mousedown", cb_mousedown.as_ref().unchecked_ref()).unwrap_throw();
        }

        unsafe {
          let _ = Box::from_raw(cb_mousemove);
          let _ = Box::from_raw(cb_mouseup);
          let _ = Box::from_raw(x);
        }
      });
    });
  }

  fn update_scroll(elem: &Element, is_lateral: bool, delta: f64) {
    if is_lateral {
      elem.scroll_by_with_x_and_y(delta, 0.);
    } else {
      elem.scroll_by_with_x_and_y(0., delta);
    }
  }

  fn update_bar_style(is_lateral: bool, bar: &HtmlElement, min_length: Option<StyleLength>, is_scrollable: Option<Signal<bool>>) {

    let min_length = min_length.unwrap_or(StyleLength::Pixel(20.));

    let Some(parent) = bar.parent_element() else { return };
    let (w, sw, sl) = if is_lateral {
      (parent.client_width(), parent.scroll_width(), parent.scroll_left())
    } else {
      (parent.client_height(), parent.scroll_height(), parent.scroll_top())
    };
    let (w, sw, sl) = (w as f64, sw as f64, sl as f64);
    let r = w/sw; // w:sw ratio

    let b = sw>w;
    if let Some(is_scrollable) = is_scrollable {
      if b != is_scrollable.get() {
        is_scrollable.set(b);
      }
    }

    // bar's width/height in percent
    let bar_len = w*r;
    let mut bar_pl = r*100.;
    if !min_length.check_min_limit(bar_len, bar_pl) {
      bar_pl = min_length.to_percent_length(w);
    }

    // bar's left/top in percent
    let bar_left = sl*(1.+r)/w*100.; // 당연히 100.%를 넘을 수도 있음. scroll width는 width보다 길 수 있으니까.

    // set style
    let (width, left) = if is_lateral { ("width", "left")} else {("height", "top")};
    bar.style().set_property(width, &format!("{bar_pl:.2}%")).unwrap_throw();
    bar.style().set_property(left, &format!("{bar_left:.2}%")).unwrap_throw();
  }
}


// component

#[derive(Props)]
pub struct ScrollBarProps {
  is_lateral: bool,
  take_orthogonal: bool,
  min_length: Option<StyleLength>,
  is_scrollable: Option<Signal<bool>>,
  update_signal: Option<ReadSignal<bool>>,
  class: &'static str,
  change_on_true: Option<(ReadSignal<bool>, Option<&'static str>, &'static str)>
}

/// ScrollBarComponent
/// * Component of ScrollBar
/// 
/// # Props
/// * is_lateral: bool,
/// * take_orthogonal: bool,
/// * min_length: Option<StyleLength>,
/// * is_scrollable: Option<Signal<bool>>,
/// * update_signal: Option<ReadSignal<bool>>,
/// * class: &'static str,
/// * change_on_true: Option<(ReadSignal<bool>, Option<&'static str>, &'static str)>
#[component]
pub fn ScrollBarComponent<G: Html>(props: ScrollBarProps) -> View<G> {

  let rf = create_node_ref();

  ScrollBar {
    bar_rf: rf,
    is_lateral: props.is_lateral,
    take_orthogonal: props.take_orthogonal,
    min_length: props.min_length,
    is_scrollable: props.is_scrollable,
  }.set_scroll_bar(props.update_signal);

  if let Some((bool_signal, old, new)) = props.change_on_true {
    ChangeClass::on_true(rf, bool_signal, old, new);
  }

  view! {
    div(ref=rf, class=props.class)
  }
}