<template>
    <div>
        <ModuleMatching />
    </div>
</template>

<script>
import ModuleMatching from "/home/spencer/prg/rust/rust-cms/frontend/src/components/ModuleMatching.vue"

/**
 * This is the component where most of the page-wide styling should be done.
 * Also, if there is unique content that is consistent across all pages (but doesn't exist on root) then put it here.
 * @displayName Page
 */
export default {
    name: "page",
    components: {
        ModuleMatching
    }
}
</script>