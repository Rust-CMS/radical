<template>
	<div>
		<component
			v-for="(module, i) in modules"
			:key="i"
			:is="module.identifier"
			:content="module.content"
		></component>
	</div>
</template>

<script>
/**
 * This matches each module with its respective component.
 * To add a custom component (before compile time),
 * 1. Create a new file.vue with the name of the new component.
 * 2. Add a <script> tag and add a name member to the `export default` object.
 *    This name member should match the name of the file, and module name.
 * 3. Add an insert query to the migration that describes your module.
 *    The name of this module should match the `name` member on the object.
 * @displayName Module matcher.
 */
import { get } from "axios";

import Paragraph from "./Paragraph";
import Header_Title from "./Header_Title";
import Image from "./Image";

const modules = {
	1: "paragraph",
	2: "header_title",
	3: "image",
};

export default {
	components: {
		header_title: Header_Title,
		Paragraph,
		Image,
	},
	async created() {
		await this.get_modules();
	},
	data() {
		return {
			modules: [],
		};
	},
	methods: {
		async get_modules() {
			let res = await get(
				`http://localhost:9090/v1/pages/${this.$route.params.id}/modules`
			);

			// Helper for mapping identifiers to custom components.
			this.modules = res.data.message.modules.map(
				(module) => {
					return {
						identifier: modules[module.module_type_id],
						content: module.content,
					};
				}
			);
		},
	},
};
</script>