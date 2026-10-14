<script lang="ts">
    import { ZodiacAquarius } from "@lucide/svelte";
    import Logo from "./Logo.svelte";

    const { status } = $props();

    const statusClass = $derived(status.toLowerCase());
</script>

<nav class={statusClass}>
    <!-- <ZodiacAquarius /> -->
    <div class="logo">
        <Logo size={22} />
    </div>
    <p>SkyoLabs</p>
</nav>

<div class="horizontal-line"></div>

<style>
    nav {
        display: flex;
        flex-direction: row;
        margin: 0;
        padding: 1rem 2rem 1rem 2rem;
        justify-content: flex-start;
        align-items: center;
        gap: 1rem;
    }

    p {
        font-size: 1.7rem;
        font-weight: bold;
        line-height: 1;
        margin: 0;
    }

    .safe p {
        color: #005f73;
    }

    .warning p,
    .dangerous p {
        color: white;
    }

    .horizontal-line {
        width: 100%;
        height: 1px;
        background: rgba(0, 0, 0, 0.2);
        margin: 0;
    }

    .logo {
        padding: 0.5rem;
        border-radius: 0.75rem;
    }

    .safe .logo {
        background-color: #005f73;
    }

    .warning .logo {
        background-color: #ae2012;
    }

    .dangerous .logo {
        background-color: #800020;
    }
</style>
