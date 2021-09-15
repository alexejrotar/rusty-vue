import Vue from 'vue'
import App from './App.vue'
const Rust = import('../pkg')

Vue.config.productionTip = false

Rust.then(rust => {
  Vue.prototype.$rust = rust

  new Vue({
    render: h => h(App),
  }).$mount('#app')
}).catch(err => {
  console.log(err)
})

