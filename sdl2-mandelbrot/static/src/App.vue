<template>
  <div id="app">
    <div>
      <mandelbrot :width="640" :height="500"/>
      <mandelbrot-native :width="640" :height="500"/>
    </div>
  </div>
</template>

<script>
import Mandelbrot from './Mandelbrot';
import MandelbrotNative from './Mandelbrot-Native';

export default {
  components: { Mandelbrot, MandelbrotNative }
}
</script>

<style>
html {
  background-color: #ddd;
}
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}
input {
  width: 50px;
  padding: 10px;
  margin: 0 5px;
}
</style>
