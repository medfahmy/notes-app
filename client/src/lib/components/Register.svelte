<script lang="ts">
    let user = {
        username: "",
        password: "",
    };

    let confirmPassword: string;

    const handleSubmit = () => {
        console.log(user);
    };
</script>

<div class="container">
    <input bind:value={user.username} placeholder="username" />
    <input bind:value={user.password} placeholder="password" />
    <input bind:value={confirmPassword} placeholder="confirm password" />

    <button on:click|preventDefault={handleSubmit} disabled={user.password != confirmPassword}>
        register
    </button>
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        align-items: center;
        min-height: 90vh;
        background: #222e50 no-repeat;
        background-size: cover;
        padding-top: 10vh;
    }
</style>
