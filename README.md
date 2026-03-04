# Haze Browser 🌫️

Современный веб-браузер, построенный на движке **Servo**.

![Haze Browser](haze_logo.png)

## Особенности

- 🚀 Быстрый рендеринг на движке Servo
- 🔒 Безопасность из коробки (Rust + MPL-2.0)
- 🎨 Современный UI
- 📑 Поддержка вкладок (в разработке)
- 🎮 Поддержка WebGPU и WebGL
- 🌐 WebXR для VR/AR контента

## Требования

### Windows

- **Windows 10/11** с последними обновлениями
- **Visual Studio 2022** с компонентами:
  - Windows 10/11 SDK (>= 10.0.19041.0)
  - MSVC v143 - VS 2022 C++ x64/x86 build tools
  - C++ ATL for latest v143 build tools
- **Rustup** (менеджер версий Rust)
- **uv** (менеджер Python проектов)

### Linux

- **gcc**, **make**, и другие инструменты сборки
- **curl**
- **Rustup**
- **uv**

### macOS

- **Xcode Command Line Tools**
- **Homebrew**
- **Rustup**
- **uv**

## Установка зависимостей

### Windows

```powershell
# Установите rustup с https://win.rustup.rs/
# Установите uv:
irm https://astral.sh/uv/install.ps1 | iex

# Перезапустите терминал и установите зависимости:
.\servo\mach bootstrap
```

### Linux/macOS

```bash
# Установите rustup:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Установите uv:
curl -LsSf https://astral.sh/uv/install.sh | sh

# Перезапустите терминал и установите зависимости:
./servo/mach bootstrap
```

## Сборка

```bash
# Отладочная сборка
cargo build

# Релизная сборка
cargo build --release

# Запуск
cargo run

# Запуск релизной версии
cargo run --release
```

## Горячие клавиши

| Клавиши | Действие |
|---------|----------|
| `Ctrl+L` | Фокус на адресную строку |
| `Ctrl+R` | Перезагрузить страницу |
| `Ctrl+T` | Новая вкладка |
| `Ctrl+W` | Закрыть вкладку |
| `Alt+←` | Назад |
| `Alt+→` | Вперёд |
| `Ctrl+0` | Сбросить масштаб |
| `Ctrl++` | Увеличить масштаб |
| `Ctrl+-` | Уменьшить масштаб |
| `F11` | Полноэкранный режим |

## Структура проекта

```
Haze/
├── src/
│   ├── main.rs          # Точка входа
│   ├── lib.rs           # Библиотека Haze
│   ├── browser.rs       # Основная логика браузера
│   ├── config.rs        # Конфигурация
│   └── ui.rs            # UI компоненты
├── servo/               # Движок Servo (субмодуль)
├── Cargo.toml           # Конфигурация Rust
├── .gitignore           # Игнорируемые файлы
└── README.md            # Этот файл
```

## Разработка

### Добавление новых функций

1. Создайте ветку: `git checkout -b feature/my-feature`
2. Внесите изменения
3. Запустите тесты: `cargo test`
4. Проверьте стиль: `cargo clippy`
5. Создайте Pull Request

### Отладка

```bash
# Запуск с подробным логированием
RUST_LOG=debug cargo run

# Запуск с отладчиком (требуется lldb)
cargo run -- --debug
```

## Известные ограничения

- ⚠️ Проект находится в активной разработке
- ⚠️ Некоторые функции могут работать нестабильно
- ⚠️ Интеграция с Servo требует дополнительной настройки

## Планы

- [ ] Полная интеграция с Servo
- [ ] Поддержка вкладок
- [ ] Закладки и история
- [ ] Менеджер загрузок
- [ ] Расширения
- [ ] Синхронизация между устройствами
- [ ] Тёмная тема

## Лицензия

Этот проект распространяется под лицензией **MPL-2.0** (Mozilla Public License 2.0).

## Благодарности

- [Servo](https://github.com/servo/servo) — движок браузера
- [Winit](https://github.com/rust-windowing/winit) — управление окнами
- [Egui](https://github.com/emilk/egui) — UI библиотека (планируется)

## Контакты

- GitHub Issues: [Сообщить о проблеме](https://github.com/yourusername/haze/issues)
- Zulip: [Servo Community](https://servo.zulipchat.com/)

---

**Haze Browser** © 2026 The Haze Project Developers
