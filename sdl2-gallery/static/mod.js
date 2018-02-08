var Module = {
    gallery: {
        pics: [
            {url: 'img/img1.jpg', preview: 'img/img1.jpg'},
            {url: 'img/img2.jpg', preview: 'img/img2.jpg'},
            {url: 'img/img3.jpg', preview: 'img/img3.jpg'},
            {url: 'img/img4.jpg', preview: 'img/img4.jpg'},
            {url: 'img/img5.jpg', preview: 'img/img5.jpg'},
        ]
    },
    canvas: (function() {
        var canvas = document.getElementById('canvas');

        canvas.addEventListener("webglcontextlost", function(e) {
            alert('WebGL context lost. You will need to reload the page.');
            e.preventDefault();
        }, false);

        return canvas;
    })()
};
