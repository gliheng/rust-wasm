<template>
  <div id="app">
    <div>
      <h1>js</h1>
      <mandelbrot ref="jsM" :width="640" :height="500"/>
    </div>
    <div>
      <h1>rust</h1>
      <mandelbrot-wasm ref="wasmM" :width="640" :height="500"/>
    </div>
  </div>
</template>

<script>
import Mandelbrot from './Mandelbrot';
import MandelbrotWasm from './Mandelbrot-Wasm';

function linkNodes(nodes, targets, events) {
  for (let node of nodes) {
    for (let evtName of events) {
      node.addEventListener(evtName, function(evt) {
        if (evt.relatedTarget) {
          return;
        }
        dispatch(evtName, evt, targets, nodes, node);
      });
    }
  }
}

function dispatch(evtName, evt, targets, nodes, except) {
  let { left, top } = except.getBoundingClientRect();
  nodes.forEach((node, i) => {
    if (node == except) return;

    let {left: left1, top: top1} = node.getBoundingClientRect();
    var e = new MouseEvent(evtName, {
      bubbles: true,
      cancelable: true,
      relatedTarget: except,
      clientX: evt.clientX - left + left1,
      clientY: evt.clientY - top + top1,
    });
    targets[i].dispatchEvent(e);
  });
}

export default {
  components: { Mandelbrot, MandelbrotWasm },
  mounted() {
let a = this.$refs.jsM;
let b = this.$refs.wasmM;
    linkNodes(
      [a.$vnode.elm, b.$vnode.elm],
      [a.getTarget(), b.getTarget()],
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
  display: flex;
  justify-content: center;
}
input {
  width: 50px;
  padding: 10px;
  margin: 0 5px;
}
</style>
