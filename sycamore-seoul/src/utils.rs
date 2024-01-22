use super::*;

pub fn replace_class<G: GenericNode>(rf: NodeRef<G>, old: &str, new: &str, to_new: bool) {
  if let Some(node) = rf.try_get::<G>() {
    node.add_class(if to_new {new} else {old});
    node.remove_class(if to_new {old} else {new});
  }
}