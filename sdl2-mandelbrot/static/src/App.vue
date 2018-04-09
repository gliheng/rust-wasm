<template>
  <div id="app">
    <div>
      <mandelbrot ref="jsM" :width="640" :height="500"/>
      <mandelbrot-wasm ref="wasmM" :width="640" :height="500"/>
    </div>
  </div>
</template>

<script>
import Mandelbrot from './Mandelbrot';
import MandelbrotWasm from './Mandelbrot-Wasm';

function linkNodes(nodes, events) {
  for (let node of nodes) {
    for (let evtName of events) {
      node.addEventListener(evtName, function(evt) {
        if (evt.relatedTarget) {
          return;
        }
        dispatch(evtName, evt, nodes, node);
      });
    }
  }
}

function dispatch(evtName, evt, nodes, except) {
  let {left, top} = except.getBoundingClientRect();
  for (let node of nodes) {
    if (node == except) continue;

    let {left: left1, top: top1} = node.getBoundingClientRect();
    var e = new MouseEvent(evtName, {
      bubbles: true,
      cancelable: true,
      relatedTarget: except,
      clientX: evt.clientX - left + left1,
      clientY: evt.clientY - top + top1,
    });
    node.dispatchEvent(e);
  }
}

export default {
  components: { Mandelbrot, MandelbrotWasm },
  mounted() {
    linkNodes(
      [this.$refs.jsM.$vnode.elm, this.$refs.wasmM.$vnode.elm],
      ['mousedown', 'mousemove', 'mouseup'],
    );
  },
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
