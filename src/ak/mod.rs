#[derive(Debug)]
#[repr(C)]
pub(crate) struct JoinRequest {
    pub(crate) token: u8,
    pub(crate) name: [u8; 31]
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct JoinResponse {
    pub(crate) token: u8,
    pub(crate) maxcli: u8,
    pub(crate) id: u16
}

#[derive(Debug)]
#[repr(C)]
#[allow(non_upper_case_globals)]
pub(crate) struct TalkRequest<const length: usize> {
    pub(crate) token: u8,
    pub(crate) len: u8,
    pub(crate) string: [u8; length]
}

#[derive(Debug)]
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

#[derive(Debug)]
#[repr(C)]
pub(crate) struct TalkBroadcast {
    pub(crate) token: u8,
    pub(crate) len: u8,
    pub(crate) id: u16,
    pub(crate) string: Vec<u8>
}

#[derive(Debug)]
#[repr(C)]
pub(crate) struct JoinBroadcast {
    pub(crate) token: u8,
    pub(crate) anim: u8,
    pub(crate) frame: u8,
    pub(crate) am: u8,
    pub(crate) pos: [f32; 3],
    pub(crate) dir: f32,
    pub(crate) id: u16,
    pub(crate) sprite: u16,
    pub(crate) name: Box<Vec<u8>>
}

impl JoinRequest {
    pub(crate) fn new(name: [u8; 31]) -> Self {
        Self{token: b'J', name: name}
    }
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

impl TalkBroadcast {
    pub(crate) fn new(brc: Vec<u8>) -> Self {
        Self {
            token: brc[0],
            len: brc[1],
            id: (brc[2] as u16) + (brc[3] as u16),
            string: brc[4..(brc[1] + 4) as usize].to_vec()
        }
    }
}

impl JoinBroadcast {
    pub(crate) fn new(brc: Vec<u8>) -> Self {
        Self {
            token: brc[0],
            anim: brc[1],
            frame: brc[2],
            am: brc[3],
            pos: [0.0, 0.0, 0.0],
            dir: 0.0,
            id: brc[20] as u16 + brc[21] as u16,
            sprite: brc[22] as u16 + brc[23] as u16,
            name: Box::new(brc[24..56].to_vec())
        }
    }
}