<template>
  <div id="app">
    <div>
      {{ a }} + {{ b }} = {{ result }}
    </div>
    <div>
      <button @click="compute()">Compute</button>
      <input type="number" v-model.number="a">
      <input type="number" v-model.number="b">
    </div>
    
  </div>
</template>

<script>
export default {
  name: 'App',
  /*
  computed: {
    result() {
      return this.$rust.add(1,1)
    }
  }
  */
  data() {
    return {
      a: 0,
      b: 0,
      result:0
    }
  },
  methods: {
    compute() {
      this.$rust.compute(this.$data)
    }
  }
}
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
