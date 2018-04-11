<template>
  <div class="container" :style="{width: width + 'px', height: height + 'px'}"
      @mousedown="onMouseDown"
      @mousemove="onMouseMove"
      @mouseup="onMouseUp"
      ref="container">
    <h1 v-if="renderTime != 0">{{ renderTime.toFixed(3) }} ms</h1>
    <canvas :width="width" :height="height" ref="canvas"></canvas>
    <canvas :width="width" :height="height" ref="dragRect"></canvas>
  </div>
</template>

<script>

const R1 = [-2.0, -1.0];
const R2 = [1.0, 1.0];
const ITERATIONS = 100;

export default {
  props: {
    width: {
      type: Number,
      default: 640,
    },
    height: {
      type: Number,
      default: 500,
    },
  },
  data() {
    return {
      renderTime: 0,
      startPoint: null,
      r1: R1,
      r2: R2,
    };
  },
  mounted() {
    this.renderMandelbrot();
    document.addEventListener('keypress', this.onKeyPress);
  },
  beforeDestroy() {
    document.removeEventListener('keypress', this.onKeyPress);
  },
  methods: {
    getTarget() {
      return this.$refs.container;
    },
    onMouseDown(evt) {
      this._dragRectContext = this.$refs.dragRect.getContext('2d');
      this._rect = this.$refs.container.getBoundingClientRect();
      let x = evt.clientX - this._rect.left;
      let y = evt.clientY - this._rect.top
      this.startPoint = [x, y];
    },
    onMouseMove(evt) {
      if (this.startPoint) {
        let rect = this._rect;
        let x = evt.clientX - rect.left;
        let y = evt.clientY - rect.top
        this.endPoint = [x, y];
        this.updateDragRect(this.startPoint, [x, y]);
      }
    },
    onMouseUp(evt) {
      this.updateDragRect();
      if (this.startPoint && this.endPoint) {
        let [x0, y0] = this.startPoint,
            [x1, y1] = this.endPoint;

        let topLeftX = Math.min(x0, x1),
            topLeftY = Math.min(y0, y1),
            bottomRightX = Math.max(x0, x1),
            bottomRightY = Math.max(y0, y1);

        let r1 = pixel2Point(topLeftX, topLeftY, [this.width, this.height], this.r1, this.r2);
        let r2 = pixel2Point(bottomRightX, bottomRightY, [this.width, this.height], this.r1, this.r2);
        this.r1 = r1;
        this.r2 = r2;
        this.renderMandelbrot();
      }
      this.startPoint = null;
      this.endPoint = null;
      this._dragRectContext = null;
    },
    onKeyPress(evt) {
      if (evt.key == ' ' && !v_eq(this.r1, this.r2)) {
        this.r1 = R1;
        this.r2 = R2;
        this.renderMandelbrot();
      }
    },
    updateDragRect(startPoint, endPoint) {
      let ctx = this._dragRectContext;
      ctx.clearRect(0, 0, this.width, this.height);
      if (startPoint && endPoint) {
        let [x0, y0] = startPoint;
        let [x1, y1] = endPoint;
        ctx.strokeStyle = '#00ff00';
        ctx.strokeRect(Math.min(x0, x1), Math.min(y0, y1), Math.abs(x0 - x1), Math.abs(y0 - y1));
      }
    },
    renderMandelbrot() {
      let ctx = this.$refs.canvas.getContext('2d');
      let t0 = performance.now();
      let imgData = ctx.createImageData(this.width, this.height);
      let { data } = imgData;
      for (let i = 0; i < this.width; i++) {
        for (let j = 0; j < this.height; j++) {
          let p = i * 4 + j * 4 * this.width;
          let point = pixel2Point(i, j, [this.width, this.height], this.r1, this.r2);
          let v = escapeTime(point, ITERATIONS);
          if (v != -1) {
            v = 255 - v;
          } else {
            v = 0;
          }
          data[p] = v;
          data[p + 1] = v;
          data[p + 2] = v;
          data[p + 3] = 255;
        }
      }

      ctx.putImageData(imgData, 0, 0);
      this.renderTime = performance.now() - t0;
    },
  }
};

function v_eq(v1, v2) {
  return v1 && v2  && v1[0] == v2[0] && v1[1] == v2[1];
}

function pixel2Point(x, y, bounds, topLeft, bottomRight) {

    let [width, height] = [bottomRight[0] - topLeft[0],
                           topLeft[1] - bottomRight[1]];
    return [
      topLeft[0] + x * width / bounds[0],
      topLeft[1] - y * height / bounds[1]
    ];
}

function escapeTime(c, limit) {
    let za = 0.0,
      zb = 0.0;
    for (let i = 0; i < limit; i++) {
      let zax = za * za - zb * zb + c[0];
      let zbx = 2 * za * zb + c[1];
      za = zax;
      zb = zbx;
      if (za * za + zb * zb > 4.0) {
        return i;
      }
    }
    return -1;
}
</script>

<style lang="scss" scoped>
.container {
  position: relative;
  display: inline-block;
  background-color: white;
  h1 {
    position: absolute;
    top: 10px;
    left: 10px;
    z-index: 10;
    margin: 0;
    pointer-events: none;
    color: #007eff;
  }
  canvas {
    position: absolute;
    top: 0;
    left: 0;
  }
}
</style>

