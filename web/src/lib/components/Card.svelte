<script lang="ts">
    import { Level } from "../index";
    import Dot from "@lucide/svelte/icons/dot";
    import type { Component } from "svelte";

    type Props = {
        title: string;
        icon: Component;
        status: Level;
        value: number;
        unit: string;
        range: string;
        progress: number;
    };

    const { title, icon: Icon, status, value, unit, range, progress }: Props = $props();

    const statusClass = $derived(
        (status).toLowerCase(),
    );
</script>

<div
    class="item-card {statusClass}">
    <!-- HEADER -->
    <div class="item-header">
        <div class="item-header-item">
            <div class="icon">
                <Icon />
            </div>
            <p class="title">{title}</p>
        </div>

        <div class="status">
            <div class="icon">
                <Dot />
            </div>
            <p>{status.toString()}</p>
        </div>
    </div>

    <!-- VALUE -->
    <div class="main-item">
        <h1 class="value">{value}{unit}</h1>
        <p class="range">{range}</p>
    </div>

    <!-- Progression Bar -->
    <div class="progress-bar">
        <div class="value {statusClass}" style="width: {progress}%;"></div>
    </div>
</div>

<style>
    .item-card {
        display: flex;
        border-radius: 0.75rem;
        justify-content: center;

        flex-direction: column;
        padding: 1.5rem;

        /* Background blur */
        backdrop-filter: blur(12px);
        -webkit-backdrop-filter: blur(12px);
        box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);

        gap: 2rem;
    }

    .item-header,
    .item-header-item {
        display: flex;
        flex-direction: row;
        align-items: center;
        gap: 0.5rem;
    }

    .item-header {
        justify-content: space-between;
    }

    .item-header-item {
        justify-content: center;
    }

    .icon {
        display: flex;
        align-items: center;
    }

    .status {
        display: flex;
        flex-direction: row;
        border-radius: 0.75rem;
        padding: 0.1rem 0.7rem 0.1rem 0.2rem;
        align-items: center;

        gap: 0.1rem;
    }

    .status p {
        margin: 0;
        font-size: 0.75rem;
        line-height: 1;
    }

    .status .icon :global(svg) {
        width: 16px;
        height: 16px;
    }

    .item-header-item .icon {
        border-radius: 0.75rem;
        padding: 0.5rem;
    }

    .main-item {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    .main-item .value {
        font-size: 4rem;
        line-height: 1;
        margin: 0;
    }

    .main-item .range {
        font-size: 1rem;
        line-height: 1;
        margin: 0;
    }

    /* COLOR SAFE */
    .safe.item-card {
        background: rgba(244, 241, 222, 0.95);
    }

    .safe .item-header-item .icon,
    .safe .status {
        background: rgba(42, 157, 143, 0.072);
    }

    .safe p,
    .safe .icon,
    .safe .main-item .value {
        color: #005f73;
    }

    .safe .main-item .range {
        color: rgba(0, 95, 115, 0.7);
    }

    /* COLOR WARNING */
    .warning.item-card {
        background: rgba(244, 241, 222, 0.95);
    }

    .warning .item-header-item .icon,
    .warning .status {
        background: rgba(174, 32, 18, 0.08);
    }

    .warning p,
    .warning .icon,
    .warning .main-item .value {
        color: #ae2012;
    }

    .warning .main-item .range {
        color: rgba(174, 32, 18, 0.7);
    }

    /* DANGEROUS */
    .dangerous.item-card {
        background: rgba(128, 0, 32, 0.95);
        border: 1px solid #800020;
    }

    .dangerous .item-header-item .icon,
    .dangerous .status {
        background: rgba(255, 255, 255, 0.12);
    }

    .dangerous p,
    .dangerous .icon,
    .dangerous .main-item .value {
        color: #ffffff;
    }

    .dangerous .main-item .range {
        color: rgba(255, 255, 255, 0.8);
    }

    /* PROGRESS BAR*/
    .progress-bar {
        width: 100%;
        background-color: rgba(0, 95, 115, 0.471);
        height: 0.5rem;
        align-items: center;
        border-radius: 0.75rem;
    }

    .progress-bar .value {
        height: 100%;
        border-radius: 0.75rem;
        transition: width 0.4s ease-in-out;
    }

    .progress-bar .value.safe {
        background-color: #005f73;
    }

    .progress-bar .value.warning {
        background-color: #AE2012;
    }

    .progress-bar .value.dangerous {
        background-color: grey;
    }

</style>
