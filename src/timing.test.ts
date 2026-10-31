import { describe, expect, test } from "bun:test";
import { computeFrameStep, parseFps } from "./timing";

describe("computeFrameStep", () => {
  test("does not advance before the target interval has elapsed", () => {
    // 10 FPS => 100ms interval
    expect(computeFrameStep(1099, 1000, 10)).toEqual({ advance: false, lastTime: 1000 });
  });

  test("advances exactly at the target interval", () => {
    expect(computeFrameStep(1100, 1000, 10)).toEqual({ advance: true, lastTime: 1100 });
  });

  test("carries the remainder of the interval over to the next frame", () => {
    // delta = 130ms, interval = 100ms => remainder 30ms is kept
    expect(computeFrameStep(1130, 1000, 10)).toEqual({ advance: true, lastTime: 1100 });
    // delta = 250ms => remainder 50ms
    expect(computeFrameStep(1250, 1000, 10)).toEqual({ advance: true, lastTime: 1200 });
  });

  test("uses the fps to compute the interval", () => {
    // 50 FPS => 20ms interval
    expect(computeFrameStep(1025, 1000, 50)).toEqual({ advance: true, lastTime: 1020 });
    expect(computeFrameStep(1019, 1000, 50)).toEqual({ advance: false, lastTime: 1000 });
  });
});

describe("parseFps", () => {
  test("parses a decimal integer", () => {
    expect(parseFps("30", 10)).toBe(30);
    expect(parseFps("08", 10)).toBe(8);
  });

  test("returns the fallback for non-numeric input", () => {
    expect(parseFps("", 10)).toBe(10);
    expect(parseFps("abc", 12)).toBe(12);
  });

  test("clamps values to the default range [1, 60]", () => {
    expect(parseFps("0", 10)).toBe(1);
    expect(parseFps("-5", 10)).toBe(1);
    expect(parseFps("1", 10)).toBe(1);
    expect(parseFps("60", 10)).toBe(60);
    expect(parseFps("61", 10)).toBe(60);
    expect(parseFps("1000", 10)).toBe(60);
  });

  test("respects a custom range", () => {
    expect(parseFps("3", 10, 5, 20)).toBe(5);
    expect(parseFps("25", 10, 5, 20)).toBe(20);
    expect(parseFps("12", 10, 5, 20)).toBe(12);
  });
});
