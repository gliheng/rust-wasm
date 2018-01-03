use stdweb::unstable::TryInto;
use stdweb::web::ArrayBuffer;
use stdweb::web::TypedArray;
use stdweb::Once;

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

pub fn fetch<F> (url: &str, cbk: F)
    where F: FnOnce(TypedArray<u8>) + 'static {
    js! {
        var cbk = @{Once(cbk)};
        fetch(@{url})
            .then(rsp => rsp.arrayBuffer())
            .then(ab => new Uint8Array(ab))
            .then(function (buf) {
                cbk(buf);
            });
    };
}
