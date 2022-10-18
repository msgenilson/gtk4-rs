// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use std::cmp;

glib::wrapper! {
    #[derive(Debug, Hash)]
    pub struct TreePath(Boxed<ffi::GtkTreePath>);

    match fn {
        copy => |ptr| ffi::gtk_tree_path_copy(ptr),
        free => |ptr| ffi::gtk_tree_path_free(ptr),
        type_ => || ffi::gtk_tree_path_get_type(),
    }
}

impl TreePath {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_new")]
    pub fn new() -> TreePath {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_tree_path_new()) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_new_first")]
    pub fn new_first() -> TreePath {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_tree_path_new_first()) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_new_from_indicesv")]
    #[doc(alias = "new_from_indicesv")]
    pub fn from_indices(indices: &[i32]) -> TreePath {
        assert_initialized_main_thread!();
        let length = indices.len() as usize;
        unsafe {
            from_glib_full(ffi::gtk_tree_path_new_from_indicesv(
                indices.to_glib_none().0,
                length,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_new_from_string")]
    #[doc(alias = "new_from_string")]
    pub fn from_string(path: &str) -> Option<TreePath> {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_tree_path_new_from_string(path.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_append_index")]
    pub fn append_index(&mut self, index_: i32) {
        unsafe {
            ffi::gtk_tree_path_append_index(self.to_glib_none_mut().0, index_);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_compare")]
    fn compare(&self, b: &TreePath) -> i32 {
        unsafe { ffi::gtk_tree_path_compare(self.to_glib_none().0, b.to_glib_none().0) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_down")]
    pub fn down(&mut self) {
        unsafe {
            ffi::gtk_tree_path_down(self.to_glib_none_mut().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_get_depth")]
    #[doc(alias = "get_depth")]
    pub fn depth(&self) -> i32 {
        unsafe { ffi::gtk_tree_path_get_depth(mut_override(self.to_glib_none().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_is_ancestor")]
    pub fn is_ancestor(&self, descendant: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_ancestor(
                mut_override(self.to_glib_none().0),
                mut_override(descendant.to_glib_none().0),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_is_descendant")]
    pub fn is_descendant(&self, ancestor: &TreePath) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_path_is_descendant(
                mut_override(self.to_glib_none().0),
                mut_override(ancestor.to_glib_none().0),
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_next")]
    pub fn next(&mut self) {
        unsafe {
            ffi::gtk_tree_path_next(self.to_glib_none_mut().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_prepend_index")]
    pub fn prepend_index(&mut self, index_: i32) {
        unsafe {
            ffi::gtk_tree_path_prepend_index(self.to_glib_none_mut().0, index_);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_prev")]
    pub fn prev(&mut self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_path_prev(self.to_glib_none_mut().0)) }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_to_string")]
    #[doc(alias = "to_string")]
    pub fn to_str(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_tree_path_to_string(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "gtk_tree_path_up")]
    pub fn up(&mut self) -> bool {
        unsafe { from_glib(ffi::gtk_tree_path_up(self.to_glib_none_mut().0)) }
    }
}

impl Default for TreePath {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for TreePath {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for TreePath {}

impl PartialOrd for TreePath {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for TreePath {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}
