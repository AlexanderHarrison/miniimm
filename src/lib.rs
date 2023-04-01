const MAX_INLINE_STR_LEN: usize = 14;

/// Length of string must fit in a u32.
/// Allows 14 bytes to be stored inline.
/// Struct is 16 bytes.
pub struct MiniImmStr {
    inner: StrTypes,
}

enum StrTypes {
    Heap {
        len: u32,
        ptr: *mut u8,
    },
    Inline {
        len: u8,
        bytes: [u8; MAX_INLINE_STR_LEN],
    }
}

impl MiniImmStr {
    pub const fn empty() -> MiniImmStr {
        MiniImmStr {
            inner: StrTypes::Inline {
                len: 0,
                bytes: [0; MAX_INLINE_STR_LEN],
            }
        }
    }

    pub fn from_string(s: String) -> MiniImmStr {
        assert!(s.len() >> 32 == 0);

        if s.len() > MAX_INLINE_STR_LEN {
            MiniImmStr {
                inner: gen_heap_str(s.into_boxed_str())
            }
        } else {
            let mut bytes = [0; MAX_INLINE_STR_LEN];
            let sbytes = s.as_bytes();
            for i in 0..s.len() {
                bytes[i] = sbytes[i];
            }
            MiniImmStr {
                inner: StrTypes::Inline {
                    len: s.len() as u8,
                    bytes,
                }
            }
        }
    }

    pub fn is_inline(&self) -> bool {
        match self.inner {
            StrTypes::Heap { .. } => false,
            StrTypes::Inline { .. } => true,
        }
    }

    pub fn as_str(&self) -> &str {
        self
    }

    /// Copies `s` into a new MiniImmStr
    pub fn from_str(s: &str) -> MiniImmStr {
        assert!(s.len() >> 32 == 0);

        if s.len() > MAX_INLINE_STR_LEN {
            MiniImmStr {
                inner: gen_heap_str(s.to_string().into_boxed_str()),
            }
        } else {
            let mut bytes = [0; MAX_INLINE_STR_LEN];
            let sbytes = s.as_bytes();
            for i in 0..s.len() {
                bytes[i] = sbytes[i];
            }
            MiniImmStr {
                inner: StrTypes::Inline {
                    len: s.len() as u8,
                    bytes,
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        match &self.inner {
            StrTypes::Heap { len, ptr: _ } => *len as usize,
            StrTypes::Inline { len, bytes: _ } => *len as usize,
        }
    }
}

impl Into<String> for MiniImmStr {
    fn into(self) -> String {
        match self.inner {
            StrTypes::Heap { len, ptr } => {
                unsafe {
                    let bytes = Box::from_raw(std::slice::from_raw_parts_mut(ptr, len as usize));
                    std::str::from_boxed_utf8_unchecked(bytes)
                }.to_string()
            },
            StrTypes::Inline { len, bytes } => {
                // safety - checked during construction
                let s = unsafe { 
                    std::str::from_utf8_unchecked(&bytes[..len as usize])
                };
                s.to_string()
            }
        }
    }
}

impl Into<Box<str>> for MiniImmStr {
    fn into(self) -> Box<str> {
        match self.inner {
            StrTypes::Heap { len, ptr } => {
                unsafe {
                    let bytes = Box::from_raw(std::slice::from_raw_parts_mut(ptr, len as usize));
                    std::str::from_boxed_utf8_unchecked(bytes)
                }
            }
            StrTypes::Inline { len, bytes } => {
                // safety - checked during construction
                let s = unsafe { 
                    std::str::from_utf8_unchecked(&bytes[..len as usize])
                };
                s.to_string().into_boxed_str()
            }
        }
    }
}

impl std::fmt::Display for MiniImmStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        (**self).fmt(f)
    }
}

impl std::fmt::Debug for MiniImmStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        (**self).fmt(f)
    }
}

impl std::ops::Deref for MiniImmStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match &self.inner {
            StrTypes::Heap { ptr, len } => {
                unsafe {
                    let bytes = std::slice::from_raw_parts(*ptr, *len as usize);
                    std::str::from_utf8_unchecked(bytes)
                }
            }
            StrTypes::Inline{ len, bytes } => unsafe {
                // safety - checked during construction
                std::str::from_utf8_unchecked(&bytes[..*len as usize])
            }
        }
    }
}

impl std::hash::Hash for MiniImmStr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (**self).hash(state)
    }
}

fn gen_heap_str(s: Box<str>) -> StrTypes {
    assert!(s.len() >> 32 == 0);

    let len = s.len() as u32;
    let ptr = (Box::leak(s) as &'static mut str).as_mut_ptr();
    StrTypes::Heap { len, ptr }
}

impl std::ops::Drop for StrTypes {
    fn drop(&mut self) {
        match self {
            StrTypes::Heap{ len, ptr } => {
                drop(unsafe { 
                    let bytes = std::slice::from_raw_parts_mut(*ptr, *len as usize);
                    let s = std::str::from_utf8_unchecked_mut(bytes);
                    Box::from_raw(s as *mut str)
                })
            },
            _ => ()
        }
    }
}
