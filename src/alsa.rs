/*
 * author: Aleksei Kozadaev (2022)
 */

mod alsa_extern {
    #[link(name = "asound")]
    #[link(name = "alsa", kind = "static")]
    extern "C" {
        pub fn get_volume() -> i32;
    }
}

pub fn volume() -> i32 {
    unsafe { alsa_extern::get_volume() }
}
