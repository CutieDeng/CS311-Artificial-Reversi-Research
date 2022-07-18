// #[no_mangle]
#[repr(C)] 
#[derive(Clone)]
pub struct Tm {
    /// 当前时刻的秒数
    pub tm_sec: i32, 
    /// 当前时刻的分钟数
    pub tm_min: i32, 
    /// 当前时刻的小时数
    pub tm_hour: i32, 
    /// 当天位于当月的第几天
    pub tm_mday: i32, 
    /// 当天处于哪月
    pub tm_mon: i32, 
    /// 今夕是何年
    pub tm_year: i32, 
    /// 今天的星期
    pub tm_wday: i32, 
    /// 今天处于今年的第几天
    pub tm_yday: i32, 
    /// TODO 
    pub tm_isdst: i32, 
}

#[repr(C)]
pub struct Timeval {
    pub tv_sec: i64, 
    pub tv_usec: i64, 
}

impl Timeval {
    pub fn subtract(&self, other: &Self) -> i64 {
        ( self.tv_sec - other.tv_sec ) * 1000000 + (self.tv_usec - other.tv_usec) 
    }
}

#[repr(C)] 
pub struct Timezone {
    tz_minuteswest: i32, 
    tz_dsttime: i32, 
}

impl core::fmt::Debug for Tm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time {{ {}/{}/{} {}:{}:{} }}", self.tm_year + 1900, self.tm_mon + 1, self.tm_mday, 
            self.tm_hour, self.tm_min, self.tm_sec) 
    }
}

impl core::fmt::Debug for Timeval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = unsafe { localtime(&(self.tv_sec) as *const i64 as *const u64) }; 
        write!(f, "Timeval {{ {}/{}/{} {:02}:{:02}:{:02}.{:03} }}", t.tm_year + 1900, t.tm_mon + 1, t.tm_mday, t.tm_hour, t.tm_min, t.tm_sec, 
            self.tv_usec / 1000) 
    }
}

extern "C" {
    pub fn time(_: *mut u64) -> u64; 
    pub fn localtime(_: *const u64) -> &'static Tm; 

    pub fn strlen(_: *const u8) -> usize; 
    fn ctime(_: *const u64) -> *const u8; 
    fn gettimeofday(_: *mut Timeval, _: *mut Timezone) -> i32; 
}

pub fn get_millis_of_time() -> Option<Timeval> {
    let mut tv = Timeval {
        tv_sec: 0, 
        tv_usec: 0, 
    }; 
    if unsafe { gettimeofday(&mut tv as *mut Timeval, std::ptr::null_mut()) } == 0 {
        Some(tv)
    } else {
        None
    }
}

pub fn ctime_to_str(s: *const u64) -> String {
    parse_cstyle_str (unsafe {ctime(s)})
}

fn parse_cstyle_str(s: *const u8) -> String {
    let mut v: Vec<u8> = Vec::with_capacity(unsafe {strlen(s)} ); 
    let mut i: isize = 0; 
    let mut c: u8; 
    while {
        c = unsafe { *s.wrapping_offset(i) }; 
        c != 0 
    } {
        v.push(c); 
        i += 1; 
    }
    String::from_utf8_lossy(&v).into()
}