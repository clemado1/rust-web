<script lang="ts">
    import { inviteUser } from '../api/authHandler'
    let error_boolean: boolean = false
    let email: string
    let message: string
    let result: boolean = false

    async function handleSubmit(event: Event) {
        console.log(event)
        console.log(email)

        if (error_boolean == false) {
            let response: Response = await inviteUser(email)
            console.log(response)
            let text: string = await response.text()
            let data = text
            result = true

            return data
        }
    }

    function validateMessageEmail(event: Event) {
        console.log(event)
        const regExp = /^[0-9a-zA-Z]([-_.]?[0-9a-zA-Z])*@[0-9a-zA-Z]([-_.]?[0-9a-zA-Z])*.[a-zA-Z]{2,3}$/i

        error_boolean = false
        if (email === '') {
            error_boolean = true
            message = 'Required email address'
        } else if (regExp.test(email) == false) {
            error_boolean = true
            message = 'please enter a valid email address'
        } else {
            message = ''
            error_boolean = false
        }
        return true
    }
</script>

{#if result === false}
    <form
        on:submit|preventDefault={handleSubmit}
        on:invalid={validateMessageEmail}
        on:changed={validateMessageEmail}
        on:input={validateMessageEmail}
    >
        <label for="email">Email</label>
        <input required type="email" id="email" bind:value={email} />
        {#if error_boolean}
            <h1>{message}</h1>
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
