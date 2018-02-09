pub mod mean;
#[cfg(feature = "fps")]
pub mod glyph_renderer;

use stdweb::unstable::TryInto;
use stdweb::web::TypedArray;
use stdweb::Once;
use sdl2::render::Texture;

pub fn get_window_dimensiton() -> (u32, u32) {
    let w = js! {
        return document.body.clientWidth;
    };
    let h = js! {
        return document.body.clientHeight;
    };
    (w.try_into().unwrap(), h.try_into().unwrap())
}

/// convert FingerMotion coordinates to px
pub fn convert(total: f32, ratio: f32) -> f32 {
    total * ratio
}

pub fn fetch<F, E> (url: &str, cbk: F, err: E)
    where F: FnOnce(String) + 'static,
          E: FnOnce() + 'static {
    js! {
        var url = @{url};
        var cbk = @{Once(cbk)};
        fetch(url)
            .then(rsp => rsp.arrayBuffer())
            .then(ab => new Uint8Array(ab))
            .then(function (data) {
                // FIXME: naive implementation
                var p = url.replace(new RegExp('/', 'g'), "_");
                FS.writeFile(p, data, {encoding: "binary"});
                cbk(p);
            }, function() {
                err();
            });
    };
}


pub struct SizedTexture(pub u32, pub u32, pub Texture);
unsafe impl Send for SizedTexture {}
