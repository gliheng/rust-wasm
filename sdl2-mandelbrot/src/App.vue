<template>
  <div id="app">
    <div>
      <input type="number" v-model="n1" @change="calc">
      <span>+</span>
      <input type="number" v-model="n2" @change="calc">
    </div>
    <h1>{{ num }}</h1>
  </div>
</template>

<script>
import wasm from './main.rs';

let getModule = wasm.initialize().then(module => {
  const add = module.cwrap('add', 'number', ['number', 'number'])
  return {
    add
  };
});

export default {
  data() {
    return {
      n1: 0,
      n2: 0,
      num: 0,
    };
  },
  methods: {
    async calc() {
      let m = await getModule;
      this.num = m.add(this.n1, this.n2);
    },
  }
}
</script>

<style>
#app {
  font-family: 'Avenir', Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
input {
  width: 50px;
  padding: 10px;
  margin: 0 5px;
}
</style>
