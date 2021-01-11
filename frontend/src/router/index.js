import Axios from 'axios'
import Vue from 'vue'
import VueRouter from 'vue-router'
import Root from '../pages/Root.vue'
import SetupStart from "../pages/setup/SetupStart.vue"
Vue.use(VueRouter)

const routes = [
	{
		path: "/setup",
		name: "Setup",
		component: SetupStart
	},
	{
		path: '/',
		name: 'Root',
		component: Root,
		beforeEnter: verifySetup
	}
]


var verifySetup = async (to, from, next) => {
	let allowed_names = ["Setup"];

	if (allowed_names.includes(to.name)) 
		return next();

	let req = await Axios.get("/config/setup").catch(() => to("Setup"));

	let req_data = req.data.message;

	if (!req_data)
		return next("Setup");

	if (req_data.config_val === "start")
		return next("Setup");

	return next();
}

const router = new VueRouter({
	mode: 'history',
	base: process.env.BASE_URL,
	routes
})

router.beforeEach(verifySetup);

export default router
