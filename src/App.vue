<template>
  <div id="app">
    <div>
      {{ a }} + {{ b }} = {{ result }}
    </div>
    <div>
      <input type="number" v-model.number="a">
      <input type="number" v-model.number="b">
    </div>
    
  </div>
</template>

<script>
export default {
  name: 'App',
  watch: {
    a() {
      this.$rust.compute(this.$data)
    },
    b() {
      this.$rust.compute(this.$data)
    }
  },
  data() {
    return {
      a: 0,
      b: 0,
      result:0
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
