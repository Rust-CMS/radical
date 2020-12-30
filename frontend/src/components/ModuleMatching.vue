<template>
	<div>
		<component v-for="(module, i) in modules" :key="i" :is="module"></component>
	</div>
</template>

<script>
import { get } from "axios";

import Paragraph from "./Paragraph";
import Header_Title from "./Header_Title";

const modules = {
	1: "paragraph",
	2: "header_title",
	3: "image",
};

export default {
    components: {
        "header_title": Header_Title,
        Paragraph,
    },
    async created() {
        await this.get_modules();
    },
    data() {
        return {
            modules: []
        }
    },
	methods: {
		async get_modules() {
            let res = await get(`http://localhost:9090/v1/pages/${this.$route.params.id}/modules`);
            this.modules = res.data.message.modules.map(module => modules[module.module_type_id]);
		},
    },
};
</script>