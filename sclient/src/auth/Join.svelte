<script lang="ts">
  import { inviteUser } from "../api/authHandler";
  let error_boolean = false;

  async function handleSubmit(event: Event) {
    console.log(event);
    console.log(event.target);
    console.log(event.target.email.value);
    console.log(event.target.password.value);

    let response: Response = await inviteUser(event.target.email.value);
    console.log(response);
    let text: string = await response.text();
    let data = text;
    return data;
  }

  function validateMessageEmail(event: Event) {
    let textbox = event.target;
    error_boolean = false;
    if (textbox.value === "") {
      textbox.setCustomValidity("Required email address");
    } else if (textbox.validity.typeMismatch) {
      error_boolean = true;
      textbox.setCustomValidity("please enter a valid email address");
    } else {
      textbox.setCustomValidity("");
    }
    return true;
  }
</script>

{#if result === undefined}
  <form
    on:submit|preventDefault={handleSubmit}
    on:invalid={validateMessageEmail}
    on:changed={validateMessageEmail}
    on:input={validateMessageEmail}>
    <label for="email">Email</label>
    <input required type="email" id="email" />
    {#if error_boolean}
      <h1>check your email address</h1>
    {/if}
    <button type="submit">Send email</button>
  </form>
{:else}
  {#await result}

    <p>Loading...</p>

  {:then value}

    <p>we send you email</p>

  {:catch error}
    {error.message}
  {/await}
{/if}
