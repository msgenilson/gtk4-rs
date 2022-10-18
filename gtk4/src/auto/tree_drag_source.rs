// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TreePath;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkTreeDragSource")]
    pub struct TreeDragSource(Interface<ffi::GtkTreeDragSource, ffi::GtkTreeDragSourceIface>);

    match fn {
        type_ => || ffi::gtk_tree_drag_source_get_type(),
    }
}

impl TreeDragSource {
    pub const NONE: Option<&'static TreeDragSource> = None;
}

pub trait TreeDragSourceExt: 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_drag_source_drag_data_delete")]
    fn drag_data_delete(&self, path: &TreePath) -> bool;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_drag_source_drag_data_get")]
    fn drag_data_get(&self, path: &TreePath) -> Option<gdk::ContentProvider>;

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_drag_source_row_draggable")]
    fn row_draggable(&self, path: &TreePath) -> bool;
}

impl<O: IsA<TreeDragSource>> TreeDragSourceExt for O {
    fn drag_data_delete(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_source_drag_data_delete(
                self.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }

    fn drag_data_get(&self, path: &TreePath) -> Option<gdk::ContentProvider> {
        unsafe {
            from_glib_full(ffi::gtk_tree_drag_source_drag_data_get(
                self.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }

    fn row_draggable(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_drag_source_row_draggable(
                self.as_ref().to_glib_none().0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }
}

impl fmt::Display for TreeDragSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TreeDragSource")
    }
}
