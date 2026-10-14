<script lang="ts">
  type Props = {
    size?: number | string;
    width?: number | string;
    height?: number | string;
    color?: string;
    strokeWidth?: number | string;
    class?: string;
  };

  const {
    size,
    width = size ?? 26,
    height = size ?? 22,
    color = "#F4F1DE",
    strokeWidth = 1.8,
    class: className = "",
  }: Props = $props();
</script>

<svg
  {width}
  {height}
  viewBox="0 0 26 22"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
  class="wave-icon {className}"
>
  <path
    d="M0.759277 5.18571C4.39564 -0.528578 8.39564 -0.528578 12.7593 5.18571C17.1229 10.9 21.1229 10.9 24.7593 5.18571M0.759277 16.6143C4.39564 10.9 8.39564 10.9 12.7593 16.6143C17.1229 22.3286 21.1229 22.3286 24.7593 16.6143"
    stroke={color}
    stroke-width={strokeWidth}
  />
</svg>

<style>
  .wave-icon {
    display: inline-block;
    vertical-align: middle;
    transition: stroke 0.2s ease-in-out;
  }
</style>
