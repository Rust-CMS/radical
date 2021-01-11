<template>
	<section>
		<h1>Setup</h1>
		<form class="setup_input_container" @submit="submit">
			<input
				type="text"
				placeholder="Database Connection URL"
				v-model="config.mysql_url"
			/>
			<input
				type="text"
				placeholder="Database Name"
				v-model="config.mysql_database"
			/>
			<input
				type="text"
				placeholder="Database Port"
				v-model="config.mysql_port"
			/>
			<input
				type="text"
				placeholder="Database Username"
				v-model="config.mysql_username"
			/>
			<input
				type="text"
				placeholder="Database Password"
				v-model="config.mysql_password"
			/>
			<button type="submit">Save</button>
		</form>
	</section>
</template>

<script>
import Axios from "axios";
export default {
	name: "SetupStart",
	data() {
		return {
			config: {
				mysql_username: "",
				mysql_password: "",
				mysql_database: "",
				mysql_url: "",
				mysql_port: "",
			},
		};
    },
    async created() {
        this.config = await this.get_current_data();
    },
	methods: {
		async get_current_data() {
			try {
				let req = await Axios.get("/v1/localConfig");

				let req_data = req.data.message;

                if (!req_data) throw new Error();
            
                return req_data;
			} catch (e) {
                console.log("Unimplemented error handler.");
            }
		},
		async submit(e) {
			e.preventDefault();

			await Axios.put("/v1/localConfig", this.config);
			await Axios.put("/v1/config/setup", {
				config_key: "setup",
				config_val: "site"
			});
		},
	},
};
</script>

<style>
.setup_input_container {
	display: flex;
	flex-direction: column;
}
</style>