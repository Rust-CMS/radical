import Axios from 'axios';
import Vue from 'vue';
import VueRouter from 'vue-router';
import Root from '../pages/Root.vue';
import SetupStart from "../pages/setup/SetupStart.vue";
import SiteSetup from "../pages/setup/SiteSetup.vue";
Vue.use(VueRouter);

const routes = [
	{
		path: "/start",
		name: "start",
		component: SetupStart
	},
	{
		path: "/site",
		name: "site",
		component: SiteSetup
	},
	{
		path: '/',
		name: 'Root',
		component: Root,
		beforeEnter: verifySetup
	}
]


var verifySetup = async (to, from, next) => {
	let allowed_names = ["setup", "site", "assets", "sites"];

	if (allowed_names.includes(to.name))
		return next();

	let req = await Axios.get("./v1/config/setup").catch(() => to("Setup"));

	let req_data = req.data.message;

	if (!req_data)
		return next("start");

	let conf_val = req_data.config_val;

	const setup_steps = ["start", "site", "Root"]

	if (setup_steps.includes(conf_val))
		next(conf_val)

	return next();
}

const router = new VueRouter({
	mode: 'history',
	base: process.env.BASE_URL,
	routes
})

router.beforeEach(verifySetup);

export default router
