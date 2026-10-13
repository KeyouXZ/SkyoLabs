<script lang="ts">
    import { StatusLevel } from "../index";
    import Dot from "@lucide/svelte/icons/dot";
    import type { Component } from "svelte";

    type Props = {
        title: string;
        icon: Component;
        status: StatusLevel;
        value: number;
        unit: string;
        range: string;
    };

    const { title, icon: Icon, status, value, unit, range }: Props = $props();
</script>

<div
    class="item-card"
    class:normal={status === StatusLevel.Normal}
    class:warning={status === StatusLevel.Warning}
    class:dangerous={status === StatusLevel.Dangerous}
>
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

    <!-- TODO: Progression Bar -->
    <!-- <div class="progress-bar"></div> -->
</div>

<style>
    .item-card {
        display: flex;
        border-radius: 0.75rem;
        justify-content: center;

        flex-direction: column;
        padding: 1.75rem;

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
        gap: 1rem;
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

    /* COLOR NORMAL */
    .normal.item-card {
        background: rgba(244, 241, 222, 0.95);
    }

    .normal .item-header-item .icon,
    .normal .status {
        background: rgba(42, 157, 143, 0.072);
    }

    .normal p,
    .normal .icon,
    .normal .main-item .value {
        color: #005f73;
    }

    .normal .main-item .range {
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
</style>
