<template>
    <div>
        <h1>Home</h1>
        <router-link v-for="(page, i) in pages" :key="i" :to="`/pages/${page.page_id}`">{{page.title}}<br></router-link> 
    </div>
</template>

<script>
import { get } from "axios";

export default {
    name: "root",
    data() {
        return {
            pages: {}
        }
    },
    async created() {
        await this.get_pages();
    },
    methods: {
        async get_pages() {
            let res = await get("http://localhost:9090/v1/pages");
            this.pages = res.data.message;
        }
    }
}
</script>