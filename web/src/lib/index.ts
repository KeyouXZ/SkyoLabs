export enum Level {
  Safe = "SAFE",
  Warning = "WARNING",
  Dangerous = "DANGEROUS"
}

export interface ThresholdRange {
  min: number;
  max: number;
}

export interface Threshold {
  safe: ThresholdRange;
  warning: ThresholdRange;
  danger: ThresholdRange;
}

/**
 * Temperature Threshold
 */
export const TEMPERATURE_THRESHOLD: Readonly<Threshold> = Object.freeze({
  safe: { min: 20.0, max: 35.0 },
  warning: { min: 15.0, max: 40.0 },
  danger: { min: -Infinity, max: Infinity }
});

/**
 * Humidity Threshold
 * Careful, if the Threshold classification changes, consider updating this.
 */
export const HUMIDITY_THRESHOLD: Readonly<Threshold> = Object.freeze({
  safe: { min: 60.0, max: 70.0 },
  warning: { min: 55.0, max: 80.0 },
  danger: { min: 0.0, max: 100.0 }
});

/**
 * PPM Calibration first - According to WHO
 */
export const CO_THRESHOLD: Readonly<Threshold> = Object.freeze({
  safe: { min: 0.0, max: 9.0 },
  warning: { min: 9.0, max: 35.0 },
  danger: { min: 35.0, max: Infinity }
});

export interface SensorData {
  mq7?: number | null;
  temperature_c?: number | null;
  humidity_pct?: number | null;
  temp_level?: Level | null;
  hum_level?: Level | null;
  gas_level?: Level | null;
  level?: Level | null;
}
