
// #[macro_export]
// macro_rules! spf {
//     (( $spf:stmt ) { $($body:tt)* }) => {
//         let time = std::time::Instant::now();

//         { $($body)* }

//         let time_elapsed = time.elapsed().as_secs_f64()();
//         if $sfp > time_elapsed {
//             let time_to_sleep = $spf - time_elapsed;
//             std::thread::sleep(std::time::Duration::from_secs_f64(time_to_sleep))
//         }
//     };
// }

#[macro_export]
macro_rules! fps30 {
    ($($body:tt)*) => {
        use crate::constants::FRAME_TIME_30FPS_IN_MICROS;
        let time = std::time::Instant::now();

        { $($body)* }

        let time_elapsed = time.elapsed().as_micros();
        if FRAME_TIME_30FPS_IN_MICROS > time_elapsed {
            let time_to_sleep = FRAME_TIME_30FPS_IN_MICROS - time_elapsed;
            std::thread::sleep(std::time::Duration::from_micros(time_to_sleep as u64));
        }
    };
}

#[macro_export]
macro_rules! fps60 {
    ($($body:tt)*) => {
        use crate::constants::FRAME_TIME_60FPS_IN_MICROS;
        let time = std::time::Instant::now();

        { $($body)* }

        let time_elapsed = time.elapsed().as_micros();
        if FRAME_TIME_60FPS_IN_MICROS > time_elapsed {
            let time_to_sleep = FRAME_TIME_60FPS_IN_MICROS - time_elapsed;
            std::thread::sleep(std::time::Duration::from_micros(time_to_sleep as u64));
        }
    };
}

#[macro_export]
macro_rules! get_fr {
    ($($body:tt)*) => {
        use crate::funcs::helpers::randomf;
        let check = randomf(0.0, 1.0) < 0.02;
        let time = std::time::Instant::now(); 

        { $($body)* }

        if check {
            let time_elapsed = time.elapsed().as_secs_f32();
            println!("frame rate: {:.1}\n", 1.0f32 / time_elapsed);
        }
    };
}

#[macro_export]
macro_rules! const_char_ptr {
    ($s:expr) => {
        {
            let s = std::ffi::CString::new($s).unwrap();
            s.into_raw() as *const std::os::raw::c_char
        }
    }
}

#[macro_export]
macro_rules! mut_char_ptr {
    ($s:expr) => {
        {
            let s = std::ffi::CString::new($s).unwrap();
            s.into_raw() as *mut std::os::raw::c_char
        }
    }
}

#[macro_export]
macro_rules! c_str {
    ($lit:expr) => {
        unsafe {
            std::ffi::CStr::from_ptr(concat!($lit, "\0").as_ptr()
                                     as *const std::os::raw::c_char)
        }
    }
}