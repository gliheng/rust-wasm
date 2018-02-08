var Module = {
    gallery: {
        pics: [
            {url: 'img/img0.jpg', preview: 'img/thumbs/img0.jpg'},
            {url: 'img/img1.jpg', preview: 'img/thumbs/img1.jpg'},
            {url: 'img/img2.jpg', preview: 'img/thumbs/img2.jpg'},
            {url: 'img/img3.jpg', preview: 'img/thumbs/img3.jpg'},
            {url: 'img/img4.jpg', preview: 'img/thumbs/img4.jpg'},
            {url: 'img/img5.jpg', preview: 'img/thumbs/img5.jpg'},
            {url: 'img/img6.jpg', preview: 'img/thumbs/img6.jpg'},
            {url: 'img/img7.jpg', preview: 'img/thumbs/img7.jpg'},
            {url: 'img/img8.jpg', preview: 'img/thumbs/img8.jpg'},
            {url: 'img/img9.jpg', preview: 'img/thumbs/img9.jpg'},
            {url: 'img/img10.jpg', preview: 'img/thumbs/img10.jpg'},
            {url: 'img/img11.jpg', preview: 'img/thumbs/img11.jpg'},
            {url: 'img/img12.jpg', preview: 'img/thumbs/img12.jpg'},
            {url: 'img/img13.jpg', preview: 'img/thumbs/img13.jpg'},
            {url: 'img/img14.jpg', preview: 'img/thumbs/img14.jpg'},
            {url: 'img/img15.jpg', preview: 'img/thumbs/img15.jpg'},
            {url: 'img/img16.jpg', preview: 'img/thumbs/img16.jpg'},
            {url: 'img/img17.jpg', preview: 'img/thumbs/img17.jpg'},
            {url: 'img/img18.jpg', preview: 'img/thumbs/img18.jpg'},
            {url: 'img/img19.jpg', preview: 'img/thumbs/img19.jpg'},
            {url: 'img/img20.jpg', preview: 'img/thumbs/img20.jpg'},
            {url: 'img/img21.jpg', preview: 'img/thumbs/img21.jpg'},
            {url: 'img/img22.jpg', preview: 'img/thumbs/img22.jpg'},
            {url: 'img/img23.jpg', preview: 'img/thumbs/img23.jpg'},
            {url: 'img/img24.jpg', preview: 'img/thumbs/img24.jpg'},
            {url: 'img/img25.jpg', preview: 'img/thumbs/img25.jpg'},
            {url: 'img/img26.jpg', preview: 'img/thumbs/img26.jpg'},
            {url: 'img/img27.jpg', preview: 'img/thumbs/img27.jpg'},
            {url: 'img/img28.jpg', preview: 'img/thumbs/img28.jpg'},
            {url: 'img/img29.jpg', preview: 'img/thumbs/img29.jpg'},
        ]
    },
    canvas: (function() {
        var canvas = document.getElementById('canvas');

        canvas.addEventListener("webglcontextlost", function(e) {
            alert('WebGL context lost. You will need to reload the page.');
            e.preventDefault();
        }, false);

        return canvas;
    })(),
    preRun: [function() {
        document.getElementById('loader').remove();
    }]
};
