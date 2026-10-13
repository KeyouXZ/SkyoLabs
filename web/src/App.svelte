<script lang="ts">
    import { StatusLevel } from "./lib";
    import Navbar from "./lib/components/Navbar.svelte";
    import Card from "./lib/components/Card.svelte";
    import { Droplet, Thermometer, Wind } from "@lucide/svelte";

    // const status = StatusLevel.Warning;
    const status = StatusLevel.Normal;

    // TODO: Logic connect to ESP-32
</script>

<main
    class:normal={status === StatusLevel.Normal}
    class:warning={status === StatusLevel.Warning}
    class:dangerous={status === StatusLevel.Dangerous}
>
    <Navbar status={status} />

    <h1 class="slogan">Environmental Telemetry Dashboard</h1>

    <!-- TODO: Toast push notif -->

    <!-- TODO: Status in threshold counter -->
    <!-- <div></div> -->

    <section>
        <Card
            title="Temperature"
            icon={Thermometer}
            status={StatusLevel.Warning}
            value={40}
            unit="°C"
            range="Ideal: 30°C - 35°C"
        />
        <Card
            title="Humadity"
            icon={Droplet}
            status={StatusLevel.Normal}
            value={30}
            unit="%"
            range="Ideal: 40% - 60%"
        />
        <Card
            title="Carbon Monoxide"
            icon={Wind}
            status={StatusLevel.Dangerous}
            value={30}
            unit="%"
            range="Ideal: 0 PPM - 9 PPM"
        />
    </section>
</main>

<style>
    :global(body) {
        margin: 0;
        padding: 0;
    }

    main {
        display: flex;
        flex-direction: column;
        min-height: 100vh;
        background-size: 200% 200%;
        animation: gradient-flow 8s ease infinite;
    }

    /* NORMAL */
    main.normal {
        background-image: linear-gradient(
            180deg,
            #ffffff 0%,
            #f1f1f1 35%,
            #d9d9d9 70%,
            #bfbfbf 100%
        );
    }

    /* WARNING */
    main.warning {
        background-image: linear-gradient(
            180deg,
            #f70d12 0%,
            #ae1c16 35%,
            #912217 70%,
            #3b2222 100%
        );
    }

    /* DANGEROUS */
    main.dangerous {
        background-image: linear-gradient(
            180deg,
            #ff006e 0%,
            #c00035 35%,
            #780020 70%,
            #24000a 100%
        );
    }

    @keyframes gradient-flow {
        0% {
            background-position: 50% 0%;
        }
        50% {
            background-position: 50% 100%;
        }
        100% {
            background-position: 50% 0%;
        }
    }

    @media (prefers-reduced-motion: reduce) {
        main {
            animation: none;
        }
    }

    section {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
        gap: 1rem;
        padding: 1rem;
        margin: 1rem;
    }

    .slogan {
        padding: 1rem;
        margin: 1rem;
        color: white;
        font-size: 3rem;
    }

    .normal .slogan {
        color: #005F73;
    }

    .warning .slogan, .dangerous .slogan {
        color: white;
    }
</style>
