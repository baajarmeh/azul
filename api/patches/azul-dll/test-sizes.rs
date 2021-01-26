    impl AzString {
        #[inline]
        pub fn as_str(&self) -> &str {
            unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
        }
        #[inline]
        pub fn as_bytes(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self.vec.ptr, self.vec.len) }
        }
    }

    impl ::std::fmt::Debug for AzCallback                   { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzLayoutCallback             { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzGlCallback                 { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzIFrameCallback             { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzTimerCallback              { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzWriteBackCallback          { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzRefAny                     {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "RefAny {{\r\n")?;
            write!(f, "    _internal_ptr: 0x{:x}\r\n", self._internal_ptr as usize)?;
            write!(f, "    _internal_len: {}\r\n", self._internal_len)?;
            write!(f, "    _internal_layout_size: {}\r\n", self._internal_layout_size)?;
            write!(f, "    _internal_layout_align: {}\r\n", self._internal_layout_align)?;
            write!(f, "    type_name: \"{}\"\r\n", self.type_name.as_str())?;
            write!(f, "    type_id: {}\r\n", self.type_id)?;
            write!(f, "    sharing_info: {:x}\r\n", &self.sharing_info as *const _ as usize)?;
            write!(f, "    custom_destructor: 0x{:x}\r\n", self.custom_destructor as usize)?;
            write!(f, "}}\r\n")?;
            Ok(())
        }
    }
