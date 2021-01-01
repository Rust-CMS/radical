<template>
    <div>
        <img :src="content">
    </div>
</template>

<script>
export default {
    name: "image",
    props: {
        content: {
            type: String,
            required: false
        }
    }
}
</script>