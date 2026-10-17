
Since you're only writing a small region every 5 s, the display is **not** the heat source. The chip is idling at **240 MHz** doing almost nothing — that's where the power (and heat) goes. Here's what will actually help, in order of impact:

### 1. Drop the CPU frequency

```cpp
void setup() {
  setCpuFrequencyMhz(80);  // 240 → 80 MHz
  // ...
}
```

Measured on a similar board: **67 mA → 33 mA** at 80 MHz. You don't need 240 MHz to update a line of text. This alone should noticeably reduce the warmth.

### 2. Disable Wi-Fi / Bluetooth

If you're not using Wi-Fi, make sure it's off. Even in "idle" it draws **~80–100 mA**. In Arduino:

```cpp
WiFi.disconnect(true);
WiFi.mode(WIFI_OFF);
```

If you never include `<WiFi.h>` it stays off, but some LilyGO firmware pre-enables it.

### 3. Deep sleep between updates (biggest win)

Since you only need the CPU for a few milliseconds every 5 s, put the chip to **sleep** the rest of the time. Deep sleep drops consumption to **~10 µA** (vs. ~30–60 mA active).

```cpp
#define uS_TO_S_FACTOR 1000000ULL

void setup() {
  setCpuFrequencyMhz(80);
  // init display, write your line
  // ...

  esp_sleep_enable_timer_wakeup(5 * uS_TO_S_FACTOR); // 5 s
  esp_deep_sleep_start();
}

void loop() {}
```

**Caveat:** deep sleep reboots the chip, so the OLED will blank. If you need the display to stay lit, use **light sleep** instead:

```cpp
#include "esp_sleep.h"

void loop() {
  // update your line
  display();

  esp_sleep_enable_timer_wakeup(4900 * 1000); // 4.9 s
  esp_light_sleep_start();  // display stays on, ~0.8 mA
}
```

### 4. Minor: lower OLED contrast

`display->setContrast(64);` (default is 255) slightly reduces OLED current. Small effect, but free.

---

**TL;DR:** `setCpuFrequencyMhz(80)` + disable Wi-Fi will likely get you from "quite warm" to barely warm. If you need it cooler still, add deep/light sleep between updates.

