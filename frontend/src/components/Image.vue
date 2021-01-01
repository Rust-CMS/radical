<template>
    <div>
        <img :src="content">
    </div>
</template>

<script>
export default {
    name: "image",
    props: {
        src: {
            type: String,
            required: true
        }
    }
}
</script>