<template>
    <section>
        <h1>Select a Site</h1>
    </section>
</template>

<script>
import Axios from 'axios'
export default {
    name: "SiteSetup",
    created() {
        this.get_sites();
    },
    methods: {
        async get_sites() {
            let req = await Axios.get("/sites");

            let sites = req.data;

            console.log(sites);
        }
    }
}
</script>