<template>
  <div class="container" :style="{width: width + 'px', height: height + 'px'}">
    <canvas :width="width" :height="height" ref="canvas"></canvas>
  </div>
</template>

<script>
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
  mounted() {
    Module.canvas = this.$refs.canvas;
  
    if (Module.calledRun) {
      Module.startApp();
    } else {
      Module.addOnPostRun(() => {
        Module.startApp();
      });
    }
  },
  methods: {
    getTarget() {
      return this.$refs.canvas;
    },
  }
}
</script>

<style lang="scss" scoped>
.container {
  position: relative;
  display: inline-block;
  background-color: white;
  canvas {
    position: absolute;
    top: 0;
    left: 0;
  }
}
</style>
