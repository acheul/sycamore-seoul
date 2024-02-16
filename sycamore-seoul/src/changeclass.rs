use super::*;

/// ChangeClass
/// 
#[derive(Debug, Clone, Copy)]
pub struct ChangeClass;

impl ChangeClass {

  /// must be inside the on_mount scope
  pub fn replace<G: GenericNode>(rf: NodeRef<G>, old: Option<&str>, new: &str, to_new: bool) {
    if let Some(node) = rf.try_get::<G>() {

      if to_new {
        node.add_class(new);
        if let Some(old) = old {
          node.remove_class(old);
        }
      } else {
        if let Some(old) = old {
          node.add_class(old);
        }
        node.remove_class(new);
      }
    }
  }


  pub fn when<G: GenericNode, T, F>(rf: NodeRef<G>, signal: ReadSignal<T>, condition: F, old: Option<&'static str>, new: &'static str)
  where F: Fn(&T)->bool + 'static
  {
    on_mount(move || {
      create_effect(on(signal, move || {
        let to_new = signal.with(|x| condition(x));
        Self::replace(rf, old, new, to_new);
      }))
    })
  }

  pub fn on_true<G: GenericNode>(rf: NodeRef<G>, signal: ReadSignal<bool>, old: Option<&'static str>, new: &'static str) {
    Self::when(rf, signal, |x| *x, old, new);
  }

  pub fn on_value<G: GenericNode, T: PartialEq>(rf: NodeRef<G>, signal: ReadSignal<T>, value: T, old: Option<&'static str>, new: &'static str) {
    Self::when(rf, signal, move |x| x==&value, old, new);
  }
}


/// ChangeClass::on_true
/// 
#[macro_export]
macro_rules! change_class_on_true {
  ($rf:expr, $signal:expr, $new:expr) => {
    ChangeClass::on_true($rf, $signal, None, $new);
  };
  ($rf:expr, $signal:expr, $old:expr, $new:expr) => {
    ChangeClass::on_true($rf, $signal, Some($old), $new);
  };  
}

/// ChangeClass::on_value
/// 
#[macro_export]
macro_rules! change_class_on_value {
  ($rf:expr, $signal:expr, $value:expr, $new:expr) => {
    ChangeClass::on_value($rf, $signal, $value, None, $new);
  };
  ($rf:expr, $signal:expr, $value:expr, $old:expr, $new:expr) => {
    ChangeClass::on_value($rf, $signal, $value, Some($old), $new);
  };  
}