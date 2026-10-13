<script lang="ts">
    import { StatusLevel } from "../index";
    import { ZodiacAquarius } from "@lucide/svelte";

    const { status } = $props();
</script>

<nav
    class:normal={status === StatusLevel.Normal}
    class:warning={status === StatusLevel.Warning}
    class:dangerous={status === StatusLevel.Dangerous}
>
    <ZodiacAquarius />
    <p>SkyoLabs</p>
</nav>

<div class="horizontal-line"></div>

<style>
    nav {
        display: flex;
        flex-direction: row;
        margin: 0;
        padding: 1.5rem;
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

    .normal p {
        color: #005F73;
    }

    .warning p, .dangerous p {
        color: white;
    }

    .horizontal-line {
        width: 100%;
        height: 1px;
        background: rgba(0, 0, 0, 0.2);
        margin: 0;
    }
</style>
