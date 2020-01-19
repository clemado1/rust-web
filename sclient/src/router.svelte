<script lang="ts">
    import Router from 'svelte-spa-router/Router.svelte'
    import { ChunkGenerator } from 'svelte-spa-chunk'
    import ChunkComponent from 'svelte-spa-chunk/Chunk.svelte'

    const Chunk = ChunkGenerator(ChunkComponent)
    console.log('router')

    export const routes = {
        '/': Chunk(() => import('./index.svelte')),
        '/login': Chunk(() => import('./auth/login.svelte')),
        '/join': Chunk(() => import('./auth/join.svelte')),
    }
</script>

<Router {routes} />
