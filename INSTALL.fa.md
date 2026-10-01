# راهنمای نصب و یکپارچه‌سازی سخت‌افزاری: BioNose-Edge

**زبان‌ها:** [English](INSTALL.md) (پیش‌فرض) · [فارسی](INSTALL.fa.md)

> این سند مرجع جامع فنی برای کامپایل، اعتبارسنجی، بسته‌بندی و پیاده‌سازی <bdi>**BioNose-Edge**</bdi> روی سیستم‌عامل‌های میزبان (<bdi>Linux</bdi>، <bdi>Windows</bdi> و <bdi>macOS</bdi>) و معماری‌های گوناگون ریزکنترل‌گرهای سخت‌افزاری جاسازی‌شده (<bdi>ARM Cortex-M</bdi>، <bdi>RISC-V</bdi> و <bdi>Espressif Xtensa</bdi>) است.

---

## ۱. پیش‌نیازهای زنجیره ابزار

| مؤلفه | حداقل نسخه | نیاز کارکردی |
| :--- | :---: | :--- |
| **زنجیره ابزار Rust** | <bdi>1.80+</bdi> | کامپایل هسته پردازشی در وضعیت <bdi>#![no_std]</bdi> و ابزار <bdi>bionose-cli</bdi> |
| **تارگت سیستم میزبان** | <bdi>x86_64</bdi> یا <bdi>aarch64</bdi> | اجرای بنچمارک‌های تحلیلی، آزمون‌های خصمانه و ابزارهای خط فرمان |
| **تارگت‌های سخت‌افزاری جاسازی‌شده** | رجوع به بخش ۳ | کامپایل مستقیم برای حافظه و پردازنده سخت‌افزار بدون سیستم‌عامل (<bdi>Bare-Metal</bdi>) |
| **ابزار کنترل نسخه Git** | <bdi>2.30+</bdi> | مدیریت مخزن و ردیابی تغییرات کدهای منبع |

---

## ۲. راه‌اندازی و اعتبارسنجی در سیستم میزبان

### گام اول: دریافت مخزن
```sh
git clone https://github.com/Ali-Rashidi-80/bionose-edge.git
cd bionose-edge
```

### گام دوم: اجرای حلقه خودکار راست‌آزمایی
```sh
# بررسی فرمت‌بندی کدها
cargo fmt --all -- --check

# لینتر سخت‌گیرانه کلیپی بدون هیچ هشدار
cargo clippy --all-targets --all-features -- -D warnings

# اجرای تمام ۳۴ آزمون واحد، یکپارچگی و استرس
cargo test --all --verbose

# اجرای اختصاصی آزمون‌های تزریق خطای خصمانه
cargo test -p bionose-core --test adversarial_stress_tests --verbose
```

### گام سوم: اجرای بنچمارک ۳۶ ماهه فیزیکی دیتاست UCI
```sh
# اجرای تورنمنت ارزیابی روی ۱۳,۹۱۰ نمونه آزمایشگاهی واقعی
cargo run --release -p bionose-cli
```

---

## ۳. ماتریس جامع تارگت‌های سخت‌افزاری ریزکنترل‌گرها

کتابخانه اصلی <bdi>bionose-core</bdi> در وضعیت خالص <bdi>#![no_std]</bdi> نوشته شده و به هیچ وجه از تخصیص پویای حافظه هپ (<bdi>alloc</bdi>) استفاده نمی‌کند. کل نیازمندی ریاضی آن متکی به کتابخانه درونی هسته زبان و محاسبات اعشاری خالص <bdi>libm</bdi> است.

### جدول انطباق و سازگاری معماری‌های سخت‌افزاری

| خانواده معماری | سه‌گانه تارگت Rust | تراشه‌ها و نمونه‌های صنعتی شاخص | وضعیت ممیز شناور | مصرف حافظه RAM | وضعیت آزمون |
| :--- | :--- | :--- | :---: | :---: | :---: |
| **ARM Cortex-M0 / M0+** | <bdi>thumbv6m-none-eabi</bdi> | <bdi>Raspberry Pi RP2040, STM32G0/L0/F0, SAMD21</bdi> | نرم‌افزاری (<bdi>libm</bdi>) | کمتر از ۱.۵ کیلوبایت | **سطح ۱ تأییدشده** |
| **ARM Cortex-M3** | <bdi>thumbv7m-none-eabi</bdi> | <bdi>STM32F103 ("Blue Pill"), STM32L1, LPC1768</bdi> | نرم‌افزاری (<bdi>libm</bdi>) | کمتر از ۱.۵ کیلوبایت | **سطح ۱ تأییدشده** |
| **ARM Cortex-M4 / M7** | <bdi>thumbv7em-none-eabi</bdi> | <bdi>STM32F301, NXP Kinetis K20/K64 (Non-FPU)</bdi> | نرم‌افزاری (<bdi>libm</bdi>) | کمتر از ۱.۵ کیلوبایت | **سطح ۱ تأییدشده** |
| **ARM Cortex-M4F / M7F** | <bdi>thumbv7em-none-eabihf</bdi> | <bdi>STM32F401/411, STM32H7, Nordic nRF52840</bdi> | سخت‌افزاری (<bdi>VFPv4/FPv5</bdi>) | کمتر از ۱.۵ کیلوبایت | **سطح ۱ تأییدشده** |
| **ARM Cortex-M33 / M55** | <bdi>thumbv8m.main-none-eabihf</bdi> | <bdi>Nordic nRF5340, STM32H5, STM32U5, LPC55S69</bdi> | سخت‌افزاری + <bdi>TrustZone</bdi> | کمتر از ۱.۵ کیلوبایت | **سطح ۱ تأییدشده** |
| **RISC-V 32-bit (IMC)** | <bdi>riscv32imc-unknown-none-elf</bdi> | <bdi>Espressif ESP32-C3, WCH CH32V103/V203/V307</bdi> | نرم‌افزاری (<bdi>libm</bdi>) | کمتر از ۱.۵ کیلوبایت | **سطح ۱ تأییدشده** |
| **RISC-V 32-bit (IMAC)** | <bdi>riscv32imac-unknown-none-elf</bdi> | <bdi>Espressif ESP32-C6 (Wi-Fi 6), ESP32-H2 (Thread)</bdi> | نرم‌افزاری + اتومیک | کمتر از ۱.۵ کیلوبایت | **سطح ۱ تأییدشده** |
| **Espressif Xtensa** | <bdi>xtensa-esp32s3-none-elf</bdi> | <bdi>ESP32, ESP32-S2, ESP32-S3 (AI Vector)</bdi> | سخت‌افزاری و برداری | کمتر از ۱.۵ کیلوبایت | **تأییدشده با espup** |

---

## ۴. دستورهای کامپایل متقابل سخت‌افزاری

### نصب زنجیره‌های ابزار ریزکنترل‌گرها
```sh
rustup target add \
  thumbv6m-none-eabi \
  thumbv7m-none-eabi \
  thumbv7em-none-eabi \
  thumbv7em-none-eabihf \
  thumbv8m.main-none-eabihf \
  riscv32imc-unknown-none-elf \
  riscv32imac-unknown-none-elf
```

### اعتبارسنجی کامپایل هر خانواده سخت‌افزاری

#### ۱. رزبری‌پای RP2040 و تراشه‌های ARM Cortex-M0+
```sh
cargo check -p bionose-core --no-default-features --target thumbv6m-none-eabi
```

#### ۲. تراشه STM32F103 و خانواده ARM Cortex-M3
```sh
cargo check -p bionose-core --no-default-features --target thumbv7m-none-eabi
```

#### ۳. تراشه‌های STM32F4 و Nordic nRF52840 (دارای بلوتوث کم‌مصرف)
```sh
cargo check -p bionose-core --no-default-features --target thumbv7em-none-eabihf
```

#### ۴. تراشه‌های پیشرفته امنیتی Nordic nRF5340 و STM32H5 (معماری Cortex-M33)
```sh
cargo check -p bionose-core --no-default-features --target thumbv8m.main-none-eabihf
```

#### ۵. تراشه کم‌هزینه اینترنت اشیاء Espressif ESP32-C3 (معماری RISC-V)
```sh
cargo check -p bionose-core --no-default-features --target riscv32imc-unknown-none-elf
```

#### ۶. تراشه‌های نوین Espressif ESP32-C6 و ESP32-H2 (معماری RISC-V با اتمیک سخت‌افزاری)
```sh
cargo check -p bionose-core --no-default-features --target riscv32imac-unknown-none-elf
```

#### ۷. پردازنده‌های دوهسته‌ای Espressif ESP32-S3 (معماری Xtensa)
با استفاده از زنجیره ابزار رسمی بنیاد اسپرسیف:
```sh
# نصب مدیر زنجیره ابزار espup
cargo install espup
espup install

# کامپایل بهینه برای هسته سخت‌افزاری ESP32-S3
cargo build -p bionose-core --target xtensa-esp32s3-none-elf --no-default-features --release
```

---

## ۵. تحلیل خصمانه مهندسی: وضعیت میکروکنترلرهای ۸ بیتی (<bdi>AVR / Arduino Uno</bdi>)

> [!WARNING]
> **محدودیت مهندسی بدون تعارف:**
> گرچه ساختار کتابخانه مانعی تئوریک برای کامپایل روی معماری‌های ۸ بیتی (<bdi>avr-unknown-gnu-atmega328</bdi>) ایجاد نمی‌کند، اما استقرار آن روی میکروکنترلرهای ۸ بیتی در خط تولید صنعتی به هیچ وجه توصیه نمی‌شود:
> - **کمبود شدید حافظه SRAM:** تراشه <bdi>ATmega328P</bdi> تنها دارای ۲,۰۴۸ بایت (۲ کیلوبایت) کل حافظه <bdi>SRAM</bdi> است. نگهداری وضعیت فیزیکی موتور شامل ضرایب کالیبراسیون و بردار خط مبنا حدود ۱.۲ کیلوبایت حافظه اشغال می‌کند که بیش از ۶۰ درصد کل حافظه تراشه را پر کرده و ریسک سرریز پشته حافظه را به‌شدت بالا می‌برد.
> - **جریمه سنگین محاسبات اعشاری:** در پردازنده ۸ بیتی فاقد واحد سخت‌افزاری ممیز شناور، هر ضرب یا تقسیم اعشاری نیازمند صدها سیکل ماشین است؛ این امر زمان استنتاج را از ۱.۸ میکروثانیه (در میکروکنترلر ۳۲ بیتی با فرکانس ۲۴۰ مگاهرتز) به بیش از ۱۵ میلی‌ثانیه افزایش می‌دهد.
> - **توصیه صریح صنعتی:** موتور <bdi>bionose-core</bdi> به‌طور اختصاصی برای **ریزکنترل‌گرهای ۳۲ بیتی و ۶۴ بیتی** طراحی شده است، جایی که کمتر از ۱ درصد حافظه را اشغال کرده و استنتاج را در کمتر از ۲ میکروثانیه به انجام می‌رساند.

---

## ۶. نحوه اتصال به فرم‌ور سخت‌افزار (<bdi>Firmware</bdi>)

کتابخانه هسته را به فایل <bdi>Cargo.toml</bdi> در پروژه سخت‌افزاری خود اضافه کنید:

```toml
[dependencies]
bionose-core = { version = "0.1.0", default-features = false }
```

### نمونه پیاده‌سازی مینیمال در سورس فرم‌ور سخت‌افزار

```rust
#![no_std]
#![no_main]

use bionose_core::{AdaptiveNoseConfig, AdaptiveNoseEngine, ModbusSlave};
use panic_halt as _; // یا هندلر خطای سخت‌افزار شما

#[entry]
fn main() -> ! {
    // ۱. مقداردهی اولیه موتور با ۱۶ کانال سنسور و ۶ دسته گاز
    let config = AdaptiveNoseConfig::industrial_default();
    let mut engine = AdaptiveNoseEngine::<16, 6>::new(&config);

    // ۲. حلقه بی‌نهایت سخت‌افزاری
    loop {
        // خواندن مقاومت فیزیکی سنسورها از طریق مبدل آنالوگ به دیجیتال
        let sensor_resistances: [f32; 16] = read_hardware_adc();

        // استنتاج بلادرنگ در کمتر از ۲ میکروثانیه
        let inference = engine.infer(&sensor_resistances);

        // تولید فریم تله‌متری استاندارد مدباس
        let mut telemetry = engine.to_modbus_telemetry(&inference, 1, 18);
        let slave = ModbusSlave::new(1);

        // ارسال داده به گذرگاه صنعتی RS485
        transmit_rs485(telemetry.as_slice());
    }
}
```

---

## ۷. بسته‌بندی خودکار و اعتبارسنجی انتشار

جهت تضمین آمادگی صددرصدی پکیج قبل از انتشار رسمی در مخزن پکیج‌های راست:

```sh
# بررسی جامع بسته‌بندی بدون چشم‌پوشی از آزمون‌ها
cargo package -p bionose-core
```
