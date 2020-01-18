<script lang="ts">
  import { onMount } from "svelte";
  import { writable } from "svelte/store";
  import { authRequest } from "./api/authHandler";
  import Join from "./auth/Join.svelte";
  import Login from "./auth/Login.svelte";

  export let url = "";
  let email: string = "";
  let auth: boolean = false;
  onMount(() => {
    console.log("App mounted");
    getLoggedUser();
    console.log("auth:" + auth);
  });

  async function getLoggedUser() {
    const user = await authRequest("GET");
    if (user === undefined) {
      console.log("1111");
      auth = false;
    }
    auth = user === undefined ? false : true;
    console.log("11" + user);

    const { email: uEmail } = user ? null : user;

    return "aa";
  }

  async function deleteUser() {
    let response: Response = await authRequest("DELETE");
    console.log(response);
    let text: string = await response.text();
    let data = text;
    auth = false;
    return data;
  }

  import Router from "svelte-spa-router/Router.svelte";
  const routes = {
    // Exact path
    "/join": Join,
    "/login": Login
  };
</script>

<style>
  h1 {
    color: purple;
  }
</style>

{#if auth === false}
  <Router {routes} />
{:else}
  {#await email}

    <p>Loading...</p>

  {:then value}

    <h1>Hello {email}!{value}</h1>
    <a on:click={deleteUser}>Logout</a>

  {/await}
{/if}
