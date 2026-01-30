import { expect, test, describe } from "bun:test";
import {
  getGpuiVersion,
  div,
  px,
  hsla,
  rgba,
  FlexDirection,
  Visibility,
  AppHandle,
} from "../index";

describe("GPUI NAPI Module Tests", () => {
  test("get version", () => {
    expect(getGpuiVersion()).toBe("0.2.2");
  });

  test("units and colors", () => {
    const p = px(10);
    expect(p.value).toBe(10);

    const color = hsla(180, 0.5, 0.5, 1.0);
    expect(color.h).toBe(180);
    expect(color.a).toBe(1.0);

    const color2 = rgba(255, 0, 0, 1.0);
    expect(color2.r).toBe(255);
  });

  test("enums", () => {
    expect(FlexDirection.Row).toBe(0);
    expect(Visibility.Visible).toBe(0);
    expect(Visibility.Hidden).toBe(1);
  });

  test("element chaining", () => {
    const element = div()
      .flex()
      .wFull()
      .hFull()
      .bg("#ff0000");
    
    expect(element).toBeDefined();
  });

  test("app and window handles", () => {
    const app = new AppHandle();
    expect(app).toBeDefined();
    expect(typeof app.run).toBe("function");
  });
});
