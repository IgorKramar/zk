# Планируемые релизы

## [0.1.1] - планируется

В этом релизе мы сфокусируемся на улучшении пользовательского опыта, делая работу с заметками более удобной и интуитивной.

### Будет добавлено
- [x] Команда `open` для открытия заметок во внешних приложениях
  ```bash
  # Открытие в конкретном приложении
  zk open --id abc123 --app nvim
  zk open --id abc123 --app code
  
  # Открытие через системный обработчик
  zk open --id abc123  # использует xdg-open
  
  # Открытие активной заметки
  zk open  # открывает последнюю просмотренную заметку
  ```

- [ ] Концепция "активной заметки"
  - Автоматическое обновление при просмотре/создании/редактировании
  - Сохранение в конфигурации последней активной заметки
  - Использование в командах без указания ID:
    ```bash
    # Работа с активной заметкой
    zk tag add tag1 tag2  # добавление тегов
    zk link add def456    # создание связи
    zk show               # просмотр содержимого
    ```

- [ ] Интерактивный выбор через fzf
  - Поиск по названию в командах:
    ```bash
    zk show **           # интерактивный поиск по названию
    zk link add **       # выбор заметки для связи
    zk tag add tag1 **   # выбор заметки для тегов
    ```
  - Предпросмотр содержимого в fzf
  - История выбора для быстрого доступа

### Будет улучшено
- [ ] Сокращённые версии команд (алиасы)
  ```bash
  zk n "Новая заметка"  # new
  zk s abc123           # show
  zk t add tag1         # tag add
  zk o                  # open
  zk l add def456       # link add
  ```

- [ ] Более компактные ID заметок
  - Уменьшение длины до 6-8 символов (было 12)
  - Проверка коллизий при генерации
  - Обратная совместимость со старыми ID
  - Опция миграции старых ID на новый формат

- [ ] Улучшенная обработка ошибок
  - Цветное форматирование сообщений об ошибках
  - Контекстные подсказки по исправлению:
    ```bash
    $ zk show invalid_id
    Ошибка: Заметка с ID 'invalid_id' не найдена
    Подсказка: Используйте 'zk show **' для интерактивного поиска
    ```
  - Логирование ошибок для отладки

### Будет исследовано
- [ ] Возможность создания TUI-интерфейса
  - Навигация по заметкам
  - Просмотр и редактирование
  - Управление тегами и связями
  - Интеграция с внешними редакторами

- [ ] Интеграция с Neovim
  - Плагин для прямой работы с заметками
  - Telescope интеграция для поиска
  - Подсветка синтаксиса для метаданных
  - Автодополнение тегов и ID

- [ ] Вдохновение из TaskWarrior
  - Система контекстов
  - Фильтры и отчёты
  - Цветовые схемы
  - Интерактивные команды

### Технические улучшения
- [ ] Оптимизация производительности
  - Кэширование метаданных заметок
  - Индексация для быстрого поиска
  - Ленивая загрузка содержимого

- [ ] Улучшение тестового покрытия
  - Интеграционные тесты для новых команд
  - Тесты для интерактивных функций
  - Бенчмарки производительности

## [0.2.0] - планируется

### Milestone 1: Визуализация связей
- 📊 ASCII-граф в консоли
  - [ ] Базовое отображение прямых связей
  - [ ] Добавление обратных связей
  - [ ] Фильтрация по тегам
  - [ ] Настройка глубины отображения
  - [ ] Цветовое выделение текущей заметки
  - [ ] Опция экспорта в DOT формат

### Milestone 2: Поддержка org-mode
- ✅ Синтаксис для задач
  - [ ] Базовые статусы TODO/DONE
  - [ ] Дополнительные статусы (IN-PROGRESS, WAITING)
  - [ ] Приоритеты [#A] [#B] [#C]
  - [ ] Теги для задач
  - [ ] Дедлайны и планирование
  - [ ] Команда `tasks` для просмотра всех задач

### Milestone 3: Периодические заметки
- 📅 Автоматизация ведения дневника
  - [ ] Шаблоны для разных периодов
  - [ ] Команда `daily` для работы с дневником
  - [ ] Команда `weekly` для обзоров
  - [ ] Команда `monthly` для ретроспектив
  - [ ] Автоматическое связывание заметок по времени
  - [ ] Навигация по периодическим заметкам

### Общие улучшения
- [ ] Улучшение обработки ошибок
- [ ] Добавление прогресс-баров для длительных операций
- [ ] Интерактивный режим для сложных команд
- [ ] Улучшение документации
- [ ] Оптимизация производительности