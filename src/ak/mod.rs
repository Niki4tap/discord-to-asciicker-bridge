use std::ffi::CStr;

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct JoinRequest {
    pub(crate) token: u8,
    pub(crate) name: [u8; 31]
}

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct JoinResponse {
    pub(crate) token: u8,
    pub(crate) maxcli: u8,
    pub(crate) id: u16
}

#[derive(Debug, Clone)]
#[repr(C)]
#[allow(non_upper_case_globals)]
pub(crate) struct TalkRequest<const length: usize> {
    pub(crate) token: u8,
    pub(crate) len: u8,
    pub(crate) string: [u8; length]
}

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct PoseRequest {
    pub(crate) token: u8,
    pub(crate) anim: u8,
    pub(crate) frame: u8,
    pub(crate) am: u8,
    pub(crate) pos: [f32; 3],
    pub(crate) dir: f32,
    pub(crate) sprite: u16
}

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct TalkBroadcast {
    pub(crate) inner: Vec<u8>
}

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct ExitBroadcast {
    pub(crate) inner: Vec<u8>
}

#[derive(Debug, Clone)]
#[repr(C)]
pub(crate) struct JoinBroadcast {
    pub(crate) inner: Vec<u8>
}

impl JoinResponse {
    pub(crate) fn new(bytes: &[u8]) -> Self {
        Self {
            token: bytes[0],
            maxcli: bytes[1],
            id: (bytes[2]) as u16 + (bytes[3]) as u16
        }
    }
}

#[allow(non_upper_case_globals)]
impl<const length: usize> TalkRequest<length> {
    #[allow(dead_code)]
    pub(crate) fn new(string: &[u8; length]) -> Self {
        assert!(string.len() < 254);
        assert!(string.len() + 2 == length);
        Self{token: b'T', len: length as u8, string: *string}
    }
}

impl PoseRequest {
    pub(crate) fn new(anim: u8, frame: u8, am: u8, pos: [f32; 3], dir: f32, sprite: u16) -> Self {
        Self{token: b'P', anim, frame, am, pos, dir, sprite}
    }
}

impl JoinRequest {
    pub(crate) fn new(name: [u8; 31]) -> Self {
        Self{token: b'J', name: name}
    }
}

impl ExitBroadcast {
    pub(crate) fn new(brc: Vec<u8>) -> Self {
        Self {
            inner: brc
        }
    }

    #[allow(dead_code)]
    pub(crate) fn token(&self) -> u8 {
        self.inner[0]
    }

    pub(crate) fn id(&self) -> u16 {
        self.inner[3] as u16 + self.inner[4] as u16
    }

    pub(crate) const TOKEN: u8 = 101;
}

impl TalkBroadcast {
    pub(crate) fn new(brc: Vec<u8>) -> Self {
        Self {
            inner: brc
        }
    }

    #[allow(dead_code)]
    pub(crate) fn token(&self) -> u8 {
        self.inner[0]
    }

    #[allow(dead_code)]
    pub(crate) fn len(&self) -> u8 {
        self.inner[1]
    }

    pub(crate) fn id(&self) -> u16 {
        (self.inner[2] as u16) + (self.inner[3] as u16)
    }

    pub(crate) fn string(&self) -> Vec<u8> {
        self.inner[4..(self.inner[1] + 4) as usize].to_vec()
    }

    pub(crate) const TOKEN: u8 = 116;
}

impl JoinBroadcast {
    pub(crate) fn new(brc: Vec<u8>) -> Self {
        Self {
            inner: brc
        }
    }

    #[allow(dead_code)]
    pub(crate) fn token(&self) -> u8 {
        self.inner[0]
    }

    #[allow(dead_code)]
    pub(crate) fn anim(&self) -> u8 {
        self.inner[1]
    }

    #[allow(dead_code)]
    pub(crate) fn frame(&self) -> u8 {
        self.inner[2]
    }

    #[allow(dead_code)]
    pub(crate) fn am(&self) -> u8 {
        self.inner[3]
    }

    #[allow(dead_code)]
    pub(crate) fn pos(&self) -> [f32; 3] {
        [0.0, 0.0, 0.0]
    }

    pub(crate) fn id(&self) -> u16 {
        self.inner[20] as u16 + self.inner[21] as u16
    }

    #[allow(dead_code)]
    pub(crate) fn sprite(&self) -> u16 {
        self.inner[22] as u16 + self.inner[23] as u16
    }

    pub(crate) fn name(&self) -> &CStr {
        unsafe {&CStr::from_ptr(self.inner[24..56].as_ptr() as *const std::os::raw::c_char)}
    }

    pub(crate) const TOKEN: u8 = 106;
}