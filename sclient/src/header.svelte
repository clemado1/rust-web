<script lang="ts">
    import { writable } from 'svelte/store'
    import { link } from 'svelte-spa-router/Router.svelte'

    import { onMount } from 'svelte'
    // @ts-ignore
    import { authRequest } from './api/authHandler.ts'
    import Router from 'svelte-spa-router/Router.svelte'
    import { ChunkGenerator } from 'svelte-spa-chunk'
    import ChunkComponent from 'svelte-spa-chunk/Chunk.svelte'

    console.log('header')

    const Chunk = ChunkGenerator(ChunkComponent)

    export const routes = {
        '/login': Chunk(() => import('./auth/login.svelte')),
        '/join': Chunk(() => import('./auth/join.svelte')),
    }

    export let email: string = ''
    export let auth: boolean = false

    onMount(() => {
        console.log('App mounted')
        getLoggedUser()
        console.log('auth:' + auth)
    })

    async function getLoggedUser() {
        const user = await authRequest('GET')
        if (user === undefined) {
            console.log('1111')
            auth = false
        }
        auth = user === undefined ? false : true
        console.log('11' + user)

        const { email: uEmail } = user ? null : user

        return 'aa'
    }

    async function deleteUser() {
        let response: Response = await authRequest('DELETE')
        console.log(response)
        let text: string = await response.text()
        let data = text
        auth = false
        return data
    }
</script>

<Router {routes} />
{#if auth === false}
    <h1>Hello Stranger!</h1>
    <ul class="navigation-links">
        <li>
            <a href="/join" use:link>Sign Up</a>
        </li>
        <li>
            <a href="/login" use:link>Sign In</a>
        </li>
    </ul>
{:else}
    {#await email}

        <p>Loading...</p>

    {:then value}

        <h1>Hello {email}!{value}</h1>
        <a href="/" on:click={deleteUser}>Logout</a>

    {/await}
{/if}

<style>
    h1 {
        color: purple;
    }
</style>
