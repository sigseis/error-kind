// =============================================================================
// MACROS
// =============================================================================

// -----------------------------------------------------------------------------
macro_rules! impl_error_adapter_traits {
    (
        others={
            alloc=[$($alloc:ident),* $(,)?],
            core=[$($core:ident),* $(,)?],
        },
        type=$type:ident,
    ) => {
        // ---------------------------------------------------------------------
        impl<K: Eq> Eq for $type<K> {}

        // ---------------------------------------------------------------------
        impl<K: core::hash::Hash> core::hash::Hash for $type<K> {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: core::hash::Hasher,
            {
                self.error_kind_ref().hash(state)
            }
        }

        // ---------------------------------------------------------------------
        impl<K: Ord> Ord for $type<K> {
            #[inline]
            fn cmp(&self, rhs: &Self) -> core::cmp::Ordering {
                self.error_kind_ref().cmp(rhs.error_kind_ref())
            }
        }

        // ---------------------------------------------------------------------
        impl<K: PartialEq> PartialEq<K> for $type<K> {
            #[inline]
            fn eq(&self, rhs: &K) -> bool {
                self.error_kind_ref().eq(rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl<K: PartialEq> PartialEq<$type<K>> for $type<K> {
            #[inline]
            fn eq(&self, rhs: &$type<K>) -> bool {
                self.error_kind_ref().eq(rhs.error_kind_ref())
            }
        }

        // ---------------------------------------------------------------------
        $(
            impl<K: PartialEq> PartialEq<crate::envelopes::$core<K>> for $type<K> {
                #[inline]
                fn eq(&self, rhs: &crate::envelopes::$core<K>) -> bool {
                    self.error_kind_ref().eq(rhs.error_kind_ref())
                }
            }
        )*

        // ---------------------------------------------------------------------
        $(
            #[cfg(feature = "alloc")]
            impl<K: PartialEq> PartialEq<crate::envelopes::$alloc<K>> for $type<K> {
                #[inline]
                fn eq(&self, rhs: &crate::envelopes::$alloc<K>) -> bool {
                    self.error_kind_ref().eq(rhs.error_kind_ref())
                }
            }
        )*

        // ---------------------------------------------------------------------
        impl<K: PartialOrd> PartialOrd<K> for $type<K>
        {
            #[inline]
            fn partial_cmp(&self, rhs: &K) -> Option<core::cmp::Ordering> {
                self.error_kind_ref().partial_cmp(rhs)
            }
        }

        // ---------------------------------------------------------------------
        impl<K: PartialOrd> PartialOrd<$type<K>> for $type<K> {
            #[inline]
            fn partial_cmp(&self, rhs: &$type<K>) -> Option<core::cmp::Ordering> {
                self.error_kind_ref().partial_cmp(rhs.error_kind_ref())
            }
        }

        // ---------------------------------------------------------------------
        $(
            impl<K: PartialOrd> PartialOrd<crate::envelopes::$core<K>> for $type<K> {
                #[inline]
                fn partial_cmp(&self, rhs: &crate::envelopes::$core<K>) -> Option<core::cmp::Ordering> {
                    self.error_kind_ref().partial_cmp(rhs.error_kind_ref())
                }
            }
        )*

        // ---------------------------------------------------------------------
        $(
            #[cfg(feature = "alloc")]
            impl<K: PartialOrd> PartialOrd<crate::envelopes::$alloc<K>> for $type<K> {
                #[inline]
                fn partial_cmp(&self, rhs: &crate::envelopes::$alloc<K>) -> Option<core::cmp::Ordering> {
                    self.error_kind_ref().partial_cmp(rhs.error_kind_ref())
                }
            }
        )*
    };
}

// =============================================================================
// RE-EXPORTS
// =============================================================================

// -----------------------------------------------------------------------------
pub(crate) use impl_error_adapter_traits;
