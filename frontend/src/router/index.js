import Vue from 'vue'
import VueRouter from 'vue-router'
import Root from '../pages/Root.vue'
import Page from "../pages/Page.vue"

Vue.use(VueRouter)

const routes = [
  {
    path: '/',
    name: 'Root',
    component: Root
  },
  {
    path: "/pages/:id",
    name: "Page",
    component: Page
  }
]

const router = new VueRouter({
  mode: 'history',
  base: process.env.BASE_URL,
  routes
})

export default router
