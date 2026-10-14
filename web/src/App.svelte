<script lang="ts">
    import {
        CO_THRESHOLD,
        HUMIDITY_THRESHOLD,
        Level,
        TEMPERATURE_THRESHOLD,
        type SensorData,
    } from "./lib";
    import Navbar from "./lib/components/Navbar.svelte";
    import Card from "./lib/components/Card.svelte";
    import { Droplet, Thermometer, Wind } from "@lucide/svelte";
    import { onMount } from "svelte";

    let loading = $state(true);
    let error = $state("");
    let isFetching = false;

    // Range Text
    const tempRangeText = $state(
        `Ideal: ${TEMPERATURE_THRESHOLD.safe.min}°C - ${TEMPERATURE_THRESHOLD.safe.max}°C`,
    );
    const humRangeText = $state(
        `Ideal: ${HUMIDITY_THRESHOLD.safe.min}% - ${HUMIDITY_THRESHOLD.safe.max}%`,
    );
    const coRangeText = $state(
        `Ideal: ${CO_THRESHOLD.safe.min} PPM - ${CO_THRESHOLD.safe.max} PPM`,
    );

    // Logic connect to ESP-32
    let sensorData = $state<SensorData>({
        level: Level.Safe,
        temp_level: Level.Safe,
        hum_level: Level.Safe,
        gas_level: Level.Safe,
    });

    const status = $derived(sensorData?.level ?? Level.Safe);
    const mainClass = $derived((sensorData.level ?? Level.Safe).toLowerCase());

    async function fetchSensors() {
        if (isFetching) return;
        isFetching = true;

        try {
            const res = await fetch("/api/sensors");

            if (!res.ok) throw new Error("Failed to fetch sensors data");

            sensorData = await res.json();
        } catch (err) {
            console.error(err);
            if (loading) error = "Failed to fetch sensors data";
        } finally {
            loading = false;
            isFetching = false;
        }
    }

    const THRESHOLDS = {
        temp: TEMPERATURE_THRESHOLD,
        hum: HUMIDITY_THRESHOLD,
        gas: CO_THRESHOLD,
    } as const;

    function calcProgress(
        value: number,
        type: "temp" | "hum" | "gas",
        level: Level,
    ): number {
        // Map Level enum to threshold key safely
        const lvlStr = String(level ?? "").toLowerCase();

        let levelKey: "safe" | "warning" | "danger" = "safe";
        if (lvlStr.includes("warn")) {
            levelKey = "warning";
        } else if (lvlStr.includes("danger")) {
            levelKey = "danger";
        }

        const range = THRESHOLDS[type][levelKey];

        // Guard against Infinity bounds (e.g., danger range)
        if (
            !isFinite(range.min) ||
            !isFinite(range.max) ||
            range.max === range.min
        ) {
            return 100;
        }

        const rawPercent =
            ((value - range.min) / (range.max - range.min)) * 100;

        // Clamp percentage between 0% and 100%
        const clamped = Math.min(100, Math.max(0, Math.round(rawPercent)));
        return clamped;
    }

    onMount(() => {
        fetchSensors();

        const interval = setInterval(fetchSensors, 1000);

        return () => {
            clearInterval(interval);
        };
    });
</script>

<main class={mainClass}>
    <Navbar {status} />

    <h1 class="slogan">Environmental Telemetry Dashboard</h1>

    <!-- TODO: Toast push notif -->

    {#if loading}
        <!-- Loading -->
        <div class="state-card">
            <div class="spinner"></div>

            <p>Loading data...</p>
        </div>
    {:else if error}
        <!-- Error -->
        <div class="error-card">
            <p>{error}</p>
        </div>
    {:else}
        <!-- TODO: Status in threshold counter -->
        <!-- <div></div> -->

        <section>
            <Card
                title="Temperature"
                icon={Thermometer}
                status={sensorData.temp_level!}
                value={Number(sensorData.temperature_c!.toFixed(1))}
                unit="°C"
                range={tempRangeText}
                progress={calcProgress(
                    sensorData.temperature_c!,
                    "temp",
                    sensorData.temp_level!,
                )}
            />
            <Card
                title="Humidity"
                icon={Droplet}
                status={sensorData.hum_level!}
                value={Number(sensorData?.humidity_pct!.toFixed(1))}
                unit="%"
                range={humRangeText}
                progress={calcProgress(
                    sensorData.humidity_pct!,
                    "hum",
                    sensorData.hum_level!,
                )}
            />
            <Card
                title="Carbon Monoxide"
                icon={Wind}
                status={sensorData.gas_level!}
                value={Number(sensorData.mq7!.toFixed(1))}
                unit=" PPM"
                range={coRangeText}
                progress={calcProgress(
                    sensorData.mq7!,
                    "gas",
                    sensorData.gas_level!,
                )}
            />
        </section>
    {/if}
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

    /* safe */
    main.safe {
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
        padding: 1rem 2rem 0 2rem;
        margin: 0;
        color: white;
        font-size: 3rem;
    }

    .safe .slogan {
        color: #005f73;
    }

    .warning .slogan,
    .dangerous .slogan {
        color: white;
    }

    /* Loading */
    .state-card {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;

        padding: 3rem;

        border: 1px solid var(--color-border);
        border-radius: var(--radius-lg);
        background: var(--color-surface);
    }

    .state-card p {
        margin: var(--space-4) 0 0;
        color: var(--color-text-muted);
        font-size: var(--font-size-sm);
    }

    .spinner {
        width: 1.5rem;
        height: 1.5rem;

        border: 2px solid var(--color-border);
        border-top-color: var(--color-primary);
        border-radius: 50%;

        animation: spin 700ms linear infinite;
    }

    @keyframes spin {
        to {
            transform: rotate(360deg);
        }
    }

    /* Error */
    .error-card {
        padding: var(--space-6);

        border: 1px solid #fecaca;
        border-radius: var(--radius-lg);
        background: var(--color-expense-bg);
    }

    .error-card p {
        margin: 0;
        color: var(--color-expense);
        font-size: var(--font-size-sm);
        font-weight: 500;
    }
</style>
