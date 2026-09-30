# راهنمای نصب و راه‌اندازی: BioNose-Edge

**زبان‌ها:** [English](INSTALL.md) (پیش‌فرض) · [فارسی](INSTALL.fa.md)

> این سند ترجمه و همراه مستند [<bdi>INSTALL.md</bdi>](INSTALL.md) است. در موارد مغایرت، متن نسخه انگلیسی به عنوان مرجع فنی اولویت دارد.

این راهنما مراحل ساخت، تست و یکپارچه‌سازی <bdi>**BioNose-Edge**</bdi> را بر روی سیستم‌عامل‌های میزبان (<bdi>Linux</bdi>، <bdi>Windows</bdi> و <bdi>macOS</bdi>) و ریزکنترل‌گرهای جاسازی‌شده (<bdi>ESP32-S3</bdi>، <bdi>ARM Cortex-M</bdi> و <bdi>RISC-V</bdi>) تشریح می‌کند.

---

## ۱. پیش‌نیازهای سیستمی

| ابزار | حداقل نسخه | کاربرد |
| :--- | :---: | :--- |
| **جعبه‌ابزار Rust** | <bdi>1.80+</bdi> | کامپایل کتابخانه هسته و اجرای تست‌های خودکار |
| **تارگت میزبان** | <bdi>x86_64</bdi> یا <bdi>aarch64</bdi> | اجرای بنچمارک تورنمنت روی دیتاست واقعی <bdi>UCI</bdi> |
| **تارگت جاسازی‌شده** | <bdi>thumbv7em-none-eabihf</bdi> یا <bdi>riscv32imc-unknown-none-elf</bdi> | کامپایل روی سخت‌افزار بدون سیستم‌عامل با وضعیت <bdi>#![no_std]</bdi> |
| **ابزار Git** | <bdi>2.30+</bdi> | کلون کردن مخزن و ردیابی تاریخچه نسخه |

---

## ۲. راه‌اندازی و اعتبارسنجی روی سیستم میزبان

### گام اول: کلون مخزن گیت
```sh
git clone https://github.com/Ali-Rashidi/BioNose-Edge.git
cd BioNose-Edge
```

### گام دوم: اجرای تست‌های واحد و مجموعه استرس خصمانه
```sh
# اجرای تمامی ۲۴ تست واحد و تست‌های نفوذ سخت‌افزاری
cargo test --all

# اجرای اختصاصی ۸ تست تزریق خطای خصمانه
cargo test --test adversarial_stress_tests
```

### گام سوم: اجرای بنچمارک تورنمنت فیزیکی ۳۶ ماهه
```sh
# اجرای تورنمنت ارزیابی با ۱۳٬۹۱۰ اندازه‌گیری واقعی دیتاست UCI
cargo run --release -p bionose-cli
```

---

## ۳. کراس‌کامپایل برای ریزکنترل‌گرهای لبه‌ای (<bdi>#![no_std]</bdi>)

کتابخانه هسته <bdi>**bionose-core**</bdi> به صورت کاملاً مستقل از تخصیص حافظه پویا و در سطح خالص <bdi>#![no_std]</bdi> بدون وابستگی به کتابخانه استاندارد مهندسی شده است.

### تارگت اول: میکروکنترلرهای ARM Cortex-M4 و Cortex-M7 (با FPU سخت‌افزاری)
```sh
# ۱. نصب تارگت سخت‌افزاری
rustup target add thumbv7em-none-eabihf

# ۲. بررسی صحت کامپایل بدون فیچرهای پیش‌فرض
cargo check -p bionose-core --target thumbv7em-none-eabihf --no-default-features
```

### تارگت دوم: پردازنده‌های ۳۲ بیتی RISC-V (مانند ESP32-C3)
```sh
# ۱. نصب تارگت پردازنده
rustup target add riscv32imc-unknown-none-elf

# ۲. بررسی صحت کامپایل
cargo check -p bionose-core --target riscv32imc-unknown-none-elf --no-default-features
```

### تارگت سوم: پردازنده‌های دو هسته‌ای Espressif ESP32-S3 (معماری Xtensa)
با استفاده از ابزار رسمی <bdi>esp-rs</bdi>:
```sh
# نصب جعبه‌ابزار اکستنسا
cargo install espup
espup install

# بیلد برای تارگت اختصاصی ESP32-S3
cargo build -p bionose-core --target xtensa-esp32s3-none-elf --no-default-features --release
```

---

## ۴. یکپارچه‌سازی در فریم‌ور نهایی

کتابخانه را به بخش نیازمندی‌های فایل <bdi>Cargo.toml</bdi> پروژه سخت‌افزاری خود اضافه کنید:

```toml
[dependencies]
bionose-core = { git = "https://github.com/Ali-Rashidi/BioNose-Edge.git", default-features = false }
```

### نمونه کد کاربردی در محیط جاسازی‌شده

```rust
#![no_std]
use bionose_core::{AdaptiveNoseConfig, AdaptiveNoseEngine, ModbusSlave};

// ۱. مقداردهی اولیه موتور با ۱۶ سنسور و ۶ دسته گاز
let config = AdaptiveNoseConfig::industrial_default();
let mut engine = AdaptiveNoseEngine::<16, 6>::new(&config);

// ۲. خواندن مقادیر آرایه سنسور از مبدل آنالوگ به دیجیتال
let sensor_resistances: [f32; 16] = read_hardware_adc();

// ۳. استنتاج فوق سریع در کمتر از ۲ میکروثانیه
let inference = engine.infer(&sensor_resistances);

// ۴. بسته‌بندی تله‌متری پروتکل صنعتی Modbus RTU
let mut telemetry = engine.to_modbus_telemetry(&inference, 1, 18);
let slave = ModbusSlave::new(1);
```

---

## ۵. چک‌لیست اعتبارسنجی قطعی پیش از استقرار

پیش از فلش کردن فریم‌ور بر روی سخت‌افزار صنعتی، از موفقیت تمام گیت‌های زیر اطمینان حاصل کنید:

```sh
cargo test --all
cargo check -p bionose-core --no-default-features
cargo clippy --all -- -D warnings
cargo fmt --all -- --check
```
